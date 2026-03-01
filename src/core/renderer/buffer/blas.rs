use crate::core::{
    RtError, RtResult,
    device::Device,
    renderer::{
        buffer::{self, Aabb, Buffer},
        command::Command,
        sync::SyncObject,
    },
};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;

pub struct BottomLevelAsDescriptor<'desc> {
    pub device: Arc<Device>,
    pub sync_object: &'desc SyncObject,
    pub command: &'desc Command,
    pub aabb_data: &'desc [Aabb],
    pub aabb_buffer: &'desc Buffer<Aabb>,
}

pub struct BottomLevelAs {
    device: Arc<Device>,
    aabb_blas_buffer: Buffer<()>,
    aabb_handle: vk::AccelerationStructureKHR,
}

struct CreateBlasParams<'param, T>
where
    T: Sized + Copy + Clone,
{
    device: Arc<Device>,
    sync_object: &'param SyncObject,
    command: &'param Command,
    data: &'param [T],
    buffer: &'param Buffer<T>,
    geometries: &'param [vk::AccelerationStructureGeometryKHR<'param>],
    blas_size: vk::AccelerationStructureBuildSizesInfoKHR<'param>,
    blas_handle: vk::AccelerationStructureKHR,
}

impl BottomLevelAs {
    pub fn new(desc: &BottomLevelAsDescriptor) -> RtResult<Self> {
        const AABB_BUFFER_NAME: &str = "BLAS buffer";

        let aabb_geometry =
            Self::create_aabb_geometry(desc.device.clone(), desc.aabb_data, desc.aabb_buffer);
        let blas_size = {
            let mut size = vk::AccelerationStructureBuildSizesInfoKHR::default();

            Self::query_build_sizes_info(
                desc.device.clone(),
                desc.aabb_data,
                &aabb_geometry,
                &mut size,
            );

            size
        };
        let aabb_blas_buffer =
            Self::create_blas_buffer(desc.device.clone(), AABB_BUFFER_NAME, &blas_size)?;
        let aabb_handle = Self::create_handle(desc.device.clone(), &blas_size, &aabb_blas_buffer)?;

        Self::build_blas(&CreateBlasParams {
            device: desc.device.clone(),
            sync_object: desc.sync_object,
            command: desc.command,
            data: desc.aabb_data,
            buffer: desc.aabb_buffer,
            geometries: &aabb_geometry,
            blas_size,
            blas_handle: aabb_handle,
        })?;

        Ok(Self {
            device: desc.device.clone(),
            aabb_blas_buffer,
            aabb_handle,
        })
    }

    fn create_blas_buffer(
        device: Arc<Device>,
        name: &str,
        size_info: &vk::AccelerationStructureBuildSizesInfoKHR,
    ) -> RtResult<Buffer<()>> {
        Buffer::new(&buffer::BufferDescriptor {
            device,
            data: None,
            ty: buffer::BufferType::Shared,
            create_info: vk::BufferCreateInfo::default()
                .size(size_info.acceleration_structure_size)
                .usage(
                    vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR
                        | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                )
                .sharing_mode(vk::SharingMode::EXCLUSIVE),
            alloc_info: vk_alloc::AllocationCreateDesc {
                name,
                allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
                requirements: vk::MemoryRequirements::default(),
                location: gpu_allocator::MemoryLocation::GpuOnly,
                linear: false,
            },
        })
    }

    fn create_aabb_geometry<'geom>(
        device: Arc<Device>,
        aabb_data: &[Aabb],
        aabb_buffer: &Buffer<Aabb>,
    ) -> Vec<vk::AccelerationStructureGeometryKHR<'geom>> {
        let aabb_geometry = if !aabb_data.is_empty() {
            let address_info =
                vk::BufferDeviceAddressInfo::default().buffer(aabb_buffer.buffers()[0]);
            let device_address = unsafe { device.raw().get_buffer_device_address(&address_info) };
            let buffer_data = vk::AccelerationStructureGeometryAabbsDataKHR::default()
                .data(vk::DeviceOrHostAddressConstKHR { device_address })
                .stride(size_of_val(aabb_data) as u64);

            Some(
                vk::AccelerationStructureGeometryKHR::default()
                    .geometry_type(vk::GeometryTypeKHR::AABBS)
                    .geometry(vk::AccelerationStructureGeometryDataKHR { aabbs: buffer_data })
                    .flags(vk::GeometryFlagsKHR::OPAQUE),
            )
        } else {
            None
        };

        match aabb_geometry {
            Some(aabb_geometry) => vec![aabb_geometry],
            None => vec![],
        }
    }

    fn query_build_sizes_info<T>(
        device: Arc<Device>,
        data: &[T],
        geometries: &[vk::AccelerationStructureGeometryKHR],
        size_info: &mut vk::AccelerationStructureBuildSizesInfoKHR,
    ) where
        T: Sized + Clone + Copy,
    {
        let build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(&geometries);
        let primitive_counts = [data.len() as u32];

        unsafe {
            device.as_loader().get_acceleration_structure_build_sizes(
                vk::AccelerationStructureBuildTypeKHR::DEVICE,
                &build_info,
                &primitive_counts,
                size_info,
            )
        }
    }

    fn create_handle(
        device: Arc<Device>,
        blas_sizes: &vk::AccelerationStructureBuildSizesInfoKHR,
        blas_buffer: &Buffer<()>,
    ) -> RtResult<vk::AccelerationStructureKHR> {
        let create_info = vk::AccelerationStructureCreateInfoKHR::default()
            .buffer(blas_buffer.buffers()[0])
            .offset(0)
            .size(blas_sizes.acceleration_structure_size)
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL);

        unsafe {
            device
                .as_loader()
                .create_acceleration_structure(&create_info, None)
        }
        .map_err(|err| RtError::CreateAccelerationStructure(err.into()))
    }

    fn build_blas<T>(param: &CreateBlasParams<T>) -> RtResult<()>
    where
        T: Sized + Clone + Copy,
    {
        const SCRATCH_REQUIRED_ALIGNMENT: u64 = 0x100;

        let command_buffer = param.command.allocate_temporary_command_buffer()?;
        let scratch_buffer = buffer::create_scratch_buffer(
            param.device.clone(),
            param.blas_size.build_scratch_size,
        )?;
        let buffer_barrier = vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR)
            .dst_access_mask(vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR)
            .buffer(param.buffer.buffers()[0])
            .size(vk::WHOLE_SIZE);
        let buffer_barrier_dependency = vk::DependencyInfo::default()
            .buffer_memory_barriers(std::slice::from_ref(&buffer_barrier));
        let build_range_info = vk::AccelerationStructureBuildRangeInfoKHR::default()
            .primitive_count(param.data.len() as u32)
            .primitive_offset(0)
            .first_vertex(0)
            .transform_offset(0);
        let as_barrier = vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR)
            .src_access_mask(vk::AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR)
            .dst_stage_mask(vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR)
            .dst_access_mask(vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR);
        let as_barrier_dependency =
            vk::DependencyInfo::default().memory_barriers(std::slice::from_ref(&as_barrier));
        // Scratch buffer should be aligned by 256bytes
        // This is for accessing the scratch buffer with an offset to align
        // memory access
        let scratch_address = (scratch_buffer.device_address()?[0]
            + (SCRATCH_REQUIRED_ALIGNMENT - 1))
            & !(SCRATCH_REQUIRED_ALIGNMENT - 1);
        let build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .dst_acceleration_structure(param.blas_handle)
            .geometries(param.geometries)
            .scratch_data(vk::DeviceOrHostAddressKHR {
                device_address: scratch_address,
            });
        let command_buffer_info = vk::CommandBufferSubmitInfo::default()
            .command_buffer(command_buffer)
            .device_mask(0);
        let submit_info = vk::SubmitInfo2::default()
            .command_buffer_infos(std::slice::from_ref(&command_buffer_info));
        let device = param.device.raw();

        param.sync_object.reset_fences(0)?;
        param.command.begin_command_buffer(command_buffer)?;
        unsafe {
            device.cmd_pipeline_barrier2(command_buffer, &buffer_barrier_dependency);
            param.device.as_loader().cmd_build_acceleration_structures(
                command_buffer,
                std::slice::from_ref(&build_info),
                &[&[build_range_info]],
            );
            device.cmd_pipeline_barrier2(command_buffer, &as_barrier_dependency);
        }
        param.command.end_command_buffer(command_buffer)?;
        unsafe {
            device
                .queue_submit2(
                    param.device.graphics_queue(),
                    std::slice::from_ref(&submit_info),
                    param.sync_object.in_flight_fences()[0],
                )
                .map_err(|err| RtError::SubmitQueue(err.into()))?;
        }
        param.sync_object.wait_for_fences(0)?;
        param.device.device_wait_idle()?;

        drop(scratch_buffer);

        Ok(())
    }
}

impl Drop for BottomLevelAs {
    fn drop(&mut self) {
        unsafe {
            self.device
                .as_loader()
                .destroy_acceleration_structure(self.aabb_handle, None);
        }
    }
}
