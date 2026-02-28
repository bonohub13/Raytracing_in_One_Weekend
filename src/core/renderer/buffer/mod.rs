mod acceleration;
mod blas;
mod descriptor_set;
mod graphics;
mod objects;
mod texture;

pub(crate) use acceleration::*;
pub(crate) use descriptor_set::*;
pub(crate) use graphics::*;
pub(crate) use objects::*;

use crate::core::{
    RtError, RtResult,
    device::Device,
    params,
    util::{lock_mutex, lock_mutex_with_fallback},
};
use ash::vk;
use gpu_allocator::vulkan::{self as vk_alloc, Allocation, AllocationCreateDesc};
use std::{marker::PhantomData, sync::Arc};

pub enum BufferType {
    ExlusiveToFrame,
    Shared,
}

pub struct BufferDescriptor<'desc, T>
where
    T: Sized + Clone + Copy,
{
    pub device: Arc<Device>,
    pub data: Option<&'desc [T]>,
    pub ty: BufferType,
    pub create_info: vk::BufferCreateInfo<'desc>,
    pub alloc_info: AllocationCreateDesc<'desc>,
}

pub struct Buffer<T>
where
    T: Sized + Clone + Copy,
{
    device: Arc<Device>,
    buffers: Vec<vk::Buffer>,
    allocations: Option<Vec<Allocation>>,
    size: vk::DeviceSize,
    device_address: Option<Vec<vk::DeviceAddress>>,
    _buffer_type: PhantomData<T>,
}

impl<T> Buffer<T>
where
    T: Sized + Clone + Copy,
{
    pub fn new(desc: &BufferDescriptor<T>) -> RtResult<Self> {
        let max_buffers = match desc.ty {
            BufferType::ExlusiveToFrame => params::MAX_FRAMES_IN_FLIGHT,
            BufferType::Shared => 1,
        };
        let buffers: Vec<vk::Buffer> = (0..max_buffers)
            .map(|_| Self::create_buffer(desc.device.clone(), &desc.create_info))
            .collect::<RtResult<Vec<_>>>()?;
        let allocations: Vec<Allocation> = buffers
            .iter()
            .map(|buffer| {
                let mut allocation =
                    Self::create_allocation(desc.device.clone(), desc.alloc_info.clone(), *buffer)?;

                if let Some(data) = desc.data
                    && let Some(mapped_ptr) = allocation.mapped_slice_mut()
                {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            data.as_ptr() as *const u8,
                            mapped_ptr.as_mut_ptr(),
                            size_of_val(data),
                        )
                    }
                }

                Ok(allocation)
            })
            .collect::<RtResult<_>>()?;
        let size = desc.create_info.size;
        let device_address = buffers
            .iter()
            .map(|buffer| {
                if desc
                    .create_info
                    .usage
                    .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
                {
                    Some(unsafe {
                        desc.device.raw().get_buffer_device_address(
                            &vk::BufferDeviceAddressInfo::default().buffer(*buffer),
                        )
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(Self {
            device: desc.device.clone(),
            buffers,
            allocations: Some(allocations),
            size,
            device_address,
            _buffer_type: PhantomData::<T>,
        })
    }

    #[inline]
    pub const fn buffers(&self) -> &[vk::Buffer] {
        self.buffers.as_slice()
    }

    pub fn write(&mut self, data: &[T]) -> RtResult<()> {
        if let Some(allocations) = self.allocations.as_mut() {
            let mapped_ranges = allocations
                .iter()
                .map(|allocation| {
                    vk::MappedMemoryRange::default()
                        .memory(unsafe { allocation.memory() })
                        .offset(allocation.offset())
                        .size(allocation.size())
                })
                .collect::<Vec<_>>();

            allocations
                .iter_mut()
                .zip(mapped_ranges)
                .try_for_each(|(allocation, mapped_range)| {
                    if let Some(mapped_ptr) = allocation.mapped_slice_mut() {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                data.as_ptr() as *const u8,
                                mapped_ptr.as_mut_ptr(),
                                size_of_val(data),
                            );
                            self.device
                                .raw()
                                .flush_mapped_memory_ranges(std::slice::from_ref(&mapped_range))
                                .map_err(|err| RtError::FlushMappedMemoryRanges(err.into()))
                        }
                    } else {
                        Ok(())
                    }
                })
        } else {
            Err(RtError::NoAllocation)
        }
    }

    fn buffer_info(&self, data: &[T], current_frame: usize) -> vk::DescriptorBufferInfo {
        vk::DescriptorBufferInfo::default()
            .buffer(self.buffers[current_frame])
            .offset(0)
            .range(size_of_val(data) as u64)
    }

    fn create_buffer(
        device: Arc<Device>,
        create_info: &vk::BufferCreateInfo,
    ) -> RtResult<vk::Buffer> {
        unsafe { device.raw().create_buffer(create_info, None) }
            .map_err(|err| RtError::CreateBuffer(err.into()))
    }

    fn create_allocation(
        device: Arc<Device>,
        mut alloc_info: AllocationCreateDesc,
        buffer: vk::Buffer,
    ) -> RtResult<Allocation> {
        let (mem_requiremnts, requires_dedicated_allocation) = {
            let info = vk::BufferMemoryRequirementsInfo2::default().buffer(buffer);
            let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
            let mut mem_requiremnts =
                vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);

            unsafe {
                device
                    .raw()
                    .get_buffer_memory_requirements2(&info, &mut mem_requiremnts)
            };

            (
                mem_requiremnts.memory_requirements,
                dedicated_requirements.requires_dedicated_allocation == vk::TRUE,
            )
        };

        alloc_info.requirements = mem_requiremnts;
        if requires_dedicated_allocation {
            alloc_info.allocation_scheme = vk_alloc::AllocationScheme::DedicatedBuffer(buffer);
        }

        let allocation = {
            let mut guard = lock_mutex!(device.allocator())?;

            guard
                .allocate(&alloc_info)
                .map_err(|err| RtError::CreateAllocation(err.into()))
        }?;
        let bind_info = vk::BindBufferMemoryInfo::default()
            .buffer(buffer)
            .memory(unsafe { allocation.memory() })
            .memory_offset(allocation.offset());

        unsafe {
            device
                .raw()
                .bind_buffer_memory2(std::slice::from_ref(&bind_info))
        }
        .map_err(|err| RtError::BindBufferMemory(err.into()))?;

        Ok(allocation)
    }
}

impl<T> Drop for Buffer<T>
where
    T: Sized + Clone + Copy,
{
    fn drop(&mut self) {
        let device = self.device.raw();

        if let Some(allocations) = self.allocations.take() {
            {
                let mut guard = lock_mutex_with_fallback!(self.device.allocator());

                allocations.into_iter().for_each(|allocation| {
                    if let Err(err) = guard
                        .free(allocation)
                        .map_err(|err| RtError::FreeAllocation(err.into()))
                    {
                        eprintln!("{err}");
                    }
                });
            }
            self.buffers
                .iter()
                .for_each(|buffer| unsafe { device.destroy_buffer(*buffer, None) });
        }
    }
}
