// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    Aabb, Allocator, Device, Encoder, Mesh, RtErr, RtError, StagingData, VkState, util::align_up,
};
use ash::vk;
use gpu_allocator::vulkan::{self as vk_alloc, Allocation};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy)]
pub enum BufferType<'data> {
    Vertex(&'data Mesh),
    Aabb(&'data Aabb),
    Staging(u64),
    Blas(u64),
    Tlas(u64),
    Scratch(u64),
}

pub struct Buffer<T>
where
    T: Sized + Clone,
{
    buffer: vk::Buffer,
    allocation: Option<Allocation>,
    gpu_address: Vec<vk::DeviceAddress>,
    is_host_visible: bool,
    device: Arc<Device>,
    allocator: Arc<Mutex<Allocator>>,
    _data: PhantomData<T>,
}

impl<T> Buffer<T>
where
    T: Sized + Clone,
{
    pub fn new(state: &VkState, allocator: Arc<Mutex<Allocator>>, ty: BufferType) -> RtErr<Self> {
        let as_properties = {
            let mut properties = vk::PhysicalDeviceAccelerationStructurePropertiesKHR::default();
            let mut device_properties =
                vk::PhysicalDeviceProperties2::default().push_next(&mut properties);

            unsafe {
                state.instance.instance().get_physical_device_properties2(
                    state.device.physical_device(),
                    &mut device_properties,
                )
            }

            properties
        };
        match ty {
            BufferType::Vertex(mesh) => {
                Self::create_vertex_buffer(state.device.clone(), allocator, mesh)
            }
            BufferType::Aabb(aabb) => {
                Self::create_aabb_buffer(state.device.clone(), allocator, aabb)
            }
            BufferType::Staging(size) => {
                Self::create_staging_buffer(state.device.clone(), allocator, size)
            }
            BufferType::Blas(size) | BufferType::Tlas(size) => {
                Self::create_acceleration_structure_buffer(state.device.clone(), allocator, size)
            }
            BufferType::Scratch(size) => Self::create_scratch_buffer(
                state.device.clone(),
                allocator,
                size,
                as_properties.min_acceleration_structure_scratch_offset_alignment as u64,
            ),
            #[allow(unused)] // For future patterns, keep this code
            _ => todo!(),
        }
    }

    #[inline]
    pub fn buffer(&self) -> vk::Buffer {
        self.buffer
    }

    #[inline]
    pub(crate) fn gpu_address(&self) -> &[vk::DeviceAddress] {
        &self.gpu_address
    }

    pub fn write(
        &mut self,
        encoder: &Encoder,
        dst_buffer: &Self,
        data: &dyn StagingData,
    ) -> RtErr<()> {
        if !self.is_host_visible {
            return Err(RtError::BufferNonHostVisible);
        }

        if let Some(allocation) = self.allocation.as_mut()
            && let Some(mapped_ptr) = allocation.mapped_ptr()
        {
            let device = self.device.device();
            let dest_slice = unsafe {
                std::slice::from_raw_parts_mut(mapped_ptr.as_ptr() as *mut u8, data.size() as usize)
            };
            let copy_regions = data.copy_regions();
            let copy_buffer_info = vk::CopyBufferInfo2::default()
                .src_buffer(self.buffer)
                .dst_buffer(dst_buffer.buffer)
                .regions(&copy_regions);
            let memory_barrier = data.copy_barrier();
            let dependency_info = vk::DependencyInfo::default()
                .memory_barriers(std::slice::from_ref(&memory_barrier));

            data.write(dest_slice);
            encoder.submit_single_command_buffer(|command_buffer| unsafe {
                device.cmd_copy_buffer2(command_buffer, &copy_buffer_info);
                device.cmd_pipeline_barrier2(command_buffer, &dependency_info);
            })
        } else {
            Err(RtError::NoAllocation)
        }
    }

    fn create_vertex_buffer(
        device: Arc<Device>,
        allocator: Arc<Mutex<Allocator>>,
        mesh: &Mesh,
    ) -> RtErr<Self> {
        let (create_infos, index_offset) = if let Some(index_offset) = mesh.index_offset()
            && let Some(index_size) = mesh.index_size()
        {
            let buffer_size = index_offset + index_size;
            // Vertex Buffer + Index Buffer
            let create_info = vk::BufferCreateInfo::default()
                .flags(vk::BufferCreateFlags::empty())
                .size(buffer_size)
                .usage(
                    vk::BufferUsageFlags::TRANSFER_DST
                        | vk::BufferUsageFlags::VERTEX_BUFFER
                        | vk::BufferUsageFlags::INDEX_BUFFER
                        | vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                        | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                )
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            (create_info, index_offset)
        } else {
            // Vertex Buffer
            let create_info = vk::BufferCreateInfo::default()
                .flags(vk::BufferCreateFlags::empty())
                .size(mesh.vertex_size())
                .usage(
                    vk::BufferUsageFlags::TRANSFER_DST
                        | vk::BufferUsageFlags::VERTEX_BUFFER
                        | vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                        | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                )
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            (create_info, 0)
        };
        let (buffer, requirements) = Self::create_buffer(device.clone(), &create_infos)?;
        let allocation = Self::allocate_memory(
            device.clone(),
            allocator.clone(),
            buffer,
            &requirements,
            gpu_allocator::MemoryLocation::GpuOnly,
        )?;
        let vertex_gpu_address = Self::get_buffer_device_address(device.clone(), buffer);
        let gpu_address = if index_offset != 0 {
            vec![vertex_gpu_address, vertex_gpu_address + index_offset]
        } else {
            vec![vertex_gpu_address]
        };

        Ok(Self {
            buffer,
            allocation,
            gpu_address,
            is_host_visible: false,
            device,
            allocator,
            _data: PhantomData,
        })
    }

    fn create_aabb_buffer(
        device: Arc<Device>,
        allocator: Arc<Mutex<Allocator>>,
        aabb: &Aabb,
    ) -> RtErr<Self> {
        let create_info = vk::BufferCreateInfo::default()
            .size(aabb.size())
            .usage(
                vk::BufferUsageFlags::TRANSFER_DST
                    | vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let (buffer, requirements) = Self::create_buffer(device.clone(), &create_info)?;
        let allocation = Self::allocate_memory(
            device.clone(),
            allocator.clone(),
            buffer,
            &requirements,
            gpu_allocator::MemoryLocation::GpuOnly,
        )?;
        let gpu_address = vec![Self::get_buffer_device_address(device.clone(), buffer)];

        Ok(Self {
            buffer,
            allocation,
            gpu_address,
            is_host_visible: false,
            device,
            allocator,
            _data: PhantomData,
        })
    }

    fn create_staging_buffer(
        device: Arc<Device>,
        allocator: Arc<Mutex<Allocator>>,
        size: u64,
    ) -> RtErr<Self> {
        let create_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let (buffer, requirements) = Self::create_buffer(device.clone(), &create_info)?;
        let allocation = Self::allocate_memory(
            device.clone(),
            allocator.clone(),
            buffer,
            &requirements,
            gpu_allocator::MemoryLocation::CpuToGpu,
        )?;
        let gpu_address = vec![Self::get_buffer_device_address(device.clone(), buffer)];

        Ok(Self {
            buffer,
            allocation,
            gpu_address,
            is_host_visible: true,
            device,
            allocator,
            _data: PhantomData,
        })
    }

    fn create_acceleration_structure_buffer(
        device: Arc<Device>,
        allocator: Arc<Mutex<Allocator>>,
        size: u64,
    ) -> RtErr<Self> {
        let create_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(
                vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let (buffer, requirements) = Self::create_buffer(device.clone(), &create_info)?;
        let allocation = Self::allocate_memory(
            device.clone(),
            allocator.clone(),
            buffer,
            &requirements,
            gpu_allocator::MemoryLocation::GpuOnly,
        )?;
        let gpu_address = vec![Self::get_buffer_device_address(device.clone(), buffer)];

        Ok(Self {
            buffer,
            allocation,
            gpu_address,
            is_host_visible: false,
            device,
            allocator,
            _data: PhantomData,
        })
    }

    fn create_scratch_buffer(
        device: Arc<Device>,
        allocator: Arc<Mutex<Allocator>>,
        size: u64,
        alignment: u64,
    ) -> RtErr<Self> {
        let create_info = vk::BufferCreateInfo::default()
            .size(align_up!(size, alignment))
            .usage(
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let (buffer, requirements) = Self::create_buffer(device.clone(), &create_info)?;
        let allocation = Self::allocate_memory(
            device.clone(),
            allocator.clone(),
            buffer,
            &requirements,
            gpu_allocator::MemoryLocation::GpuOnly,
        )?;
        let gpu_address = vec![align_up!(
            Self::get_buffer_device_address(device.clone(), buffer),
            alignment
        )];

        Ok(Self {
            buffer,
            allocation,
            gpu_address,
            is_host_visible: false,
            device,
            allocator,
            _data: PhantomData,
        })
    }

    fn create_buffer(
        device: Arc<Device>,
        create_info: &vk::BufferCreateInfo,
    ) -> RtErr<(vk::Buffer, vk::MemoryRequirements)> {
        let buffer = unsafe { device.device().create_buffer(create_info, None) }
            .map_err(|err| RtError::CreateBuffer(err.into()))?;
        let buf_mem_requirements =
            vk::DeviceBufferMemoryRequirements::default().create_info(create_info);
        let mut mem_requirements = vk::MemoryRequirements2::default();

        unsafe {
            device
                .device()
                .get_device_buffer_memory_requirements(&buf_mem_requirements, &mut mem_requirements)
        };

        Ok((buffer, mem_requirements.memory_requirements))
    }

    fn allocate_memory(
        device: Arc<Device>,
        allocator: Arc<Mutex<Allocator>>,
        buffer: vk::Buffer,
        requirements: &vk::MemoryRequirements,
        location: gpu_allocator::MemoryLocation,
    ) -> RtErr<Option<Allocation>> {
        const BUFFER_MEMORY_NAME: &str = "Buffer bound memory";

        let mut guard = allocator.lock().map_err(|_| RtError::LockMutex)?;
        let allocation = guard
            .allocator
            .allocate(&vk_alloc::AllocationCreateDesc {
                name: BUFFER_MEMORY_NAME,
                requirements: *requirements,
                location,
                linear: true,
                allocation_scheme: vk_alloc::AllocationScheme::DedicatedBuffer(buffer),
            })
            .map_err(|err| RtError::AllocateMemory(err.into()))?;
        let bind_info = vk::BindBufferMemoryInfo::default()
            .buffer(buffer)
            .memory(unsafe { allocation.memory() })
            .memory_offset(allocation.offset());

        unsafe {
            device
                .device()
                .bind_buffer_memory2(std::slice::from_ref(&bind_info))
        }
        .map_err(|err| RtError::BindBufferMemory(err.into()))?;

        Ok(Some(allocation))
    }

    fn get_buffer_device_address(device: Arc<Device>, buffer: vk::Buffer) -> vk::DeviceAddress {
        let addr_info = vk::BufferDeviceAddressInfo::default().buffer(buffer);

        unsafe { device.device().get_buffer_device_address(&addr_info) }
    }
}

impl<T> Drop for Buffer<T>
where
    T: Sized + Clone,
{
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_buffer(self.buffer, None);
        }
        if let Ok(mut guard) = self.allocator.lock()
            && let Some(allocation) = self.allocation.take()
            && let Err(err) = guard.free(allocation)
        {
            eprintln!("{err}");
        }
    }
}
