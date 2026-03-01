use crate::core::{
    RtError, RtResult,
    device::Device,
    renderer::{
        buffer::{self, Buffer},
        path_tracer::Pipeline,
    },
    util::align_up,
};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;

pub struct ShaderBindingTableDescriptor<'desc> {
    pub device: Arc<Device>,
    pub rt_pipeline: Pipeline,
    pub name: &'desc str,
}

pub struct ShaderBindingTable {
    buffer: Buffer<u8>,
    ray_generation: vk::StridedDeviceAddressRegionKHR,
    miss: vk::StridedDeviceAddressRegionKHR,
    closest_hit: vk::StridedDeviceAddressRegionKHR,
    call_region: vk::StridedDeviceAddressRegionKHR,
}

struct ShaderStride {
    raygen_stride: u64,
    raygen_size: u64,
    miss_stride: u64,
    miss_size: u64,
    hit_stride: u64,
    hit_size: u64,
}

impl ShaderBindingTable {
    const MISS_SHADER_COUNT: u32 = 1;
    const HIT_SHADER_COUNT: u32 = 1;
    pub const MESH_HIT_SBT_OFFSET: u32 = 0;
    pub const AABB_HIT_SBT_OFFSET: u32 = 1;

    pub fn new(desc: &ShaderBindingTableDescriptor) -> RtResult<Self> {
        let properties = Self::query_device_properties(desc.device.clone());
        let sbt_handles_raw = Self::query_shader_group_handles(
            desc.device.clone(),
            desc.rt_pipeline.inner(),
            properties,
        )?;
        let (buffer, shader_strides) =
            Self::create_buffer(desc.device.clone(), properties, desc.name, &sbt_handles_raw)?;
        let device_address = buffer.device_address()?[0];
        let raygen_region = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(device_address)
            .stride(shader_strides.raygen_stride)
            .size(shader_strides.raygen_size);
        let miss_region = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(device_address + shader_strides.raygen_size)
            .stride(shader_strides.miss_stride)
            .size(shader_strides.miss_size);
        let hit_region = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(
                device_address + shader_strides.raygen_size + shader_strides.miss_stride,
            )
            .stride(shader_strides.hit_stride)
            .size(shader_strides.hit_size);

        Ok(Self {
            buffer,
            ray_generation: raygen_region,
            miss: miss_region,
            closest_hit: hit_region,
            call_region: vk::StridedDeviceAddressRegionKHR::default(),
        })
    }

    #[inline]
    pub const fn ray_generation_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.ray_generation
    }

    #[inline]
    pub const fn miss_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.miss
    }

    #[inline]
    pub const fn hit_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.closest_hit
    }

    #[inline]
    pub const fn call_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.call_region
    }

    fn query_device_properties<'properties>(
        device: Arc<Device>,
    ) -> vk::PhysicalDeviceRayTracingPipelinePropertiesKHR<'properties> {
        let mut rt_pipeline_props = vk::PhysicalDeviceRayTracingPipelinePropertiesKHR::default();
        let mut props = vk::PhysicalDeviceProperties2::default().push_next(&mut rt_pipeline_props);

        device.query_device_properties(&mut props);

        rt_pipeline_props
    }

    fn query_shader_group_handles(
        device: Arc<Device>,
        pipeline: vk::Pipeline,
        properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR,
    ) -> RtResult<Vec<u8>> {
        let handle_size = properties.shader_group_handle_size;
        let group_count = 1 + Self::MISS_SHADER_COUNT + Self::HIT_SHADER_COUNT;

        unsafe {
            device.rt_loader().get_ray_tracing_shader_group_handles(
                pipeline,
                0,
                group_count,
                (group_count * handle_size) as usize,
            )
        }
        .map_err(|err| RtError::GetRTShaderGroupHandles(err.into()))
    }

    fn create_buffer(
        device: Arc<Device>,
        properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR,
        name: &str,
        sbt_handles_raw: &[u8],
    ) -> RtResult<(Buffer<u8>, ShaderStride)> {
        let handle_size = properties.shader_group_handle_size;
        let base_align = properties.shader_group_base_alignment;
        // Ray Generation
        let raygen_stride = align_up!(handle_size, base_align);
        let raygen_size = raygen_stride;
        // Miss
        let miss_stride = handle_size;
        let miss_size = align_up!(Self::MISS_SHADER_COUNT * handle_size, base_align);
        let miss_offset = raygen_size as usize;
        let miss_handle_start = handle_size as usize;
        // Hit (Mesh + AABB)
        let hit_stride = handle_size;
        let hit_size = align_up!(Self::HIT_SHADER_COUNT * handle_size, base_align);
        let hit_offset = (raygen_size + miss_size) as usize;
        let hit_handle_start = ((1 + Self::MISS_SHADER_COUNT) * handle_size) as usize;
        let sbt_size: u32 = [raygen_size, miss_size, hit_size].iter().sum();
        let mut sbt_host_data = vec![0u8; sbt_size as usize];

        sbt_host_data[0..handle_size as usize]
            .copy_from_slice(&sbt_handles_raw[0..handle_size as usize]);
        sbt_host_data[miss_offset..(miss_offset + handle_size as usize)].copy_from_slice(
            &sbt_handles_raw[miss_handle_start..miss_handle_start + handle_size as usize],
        );
        sbt_host_data[hit_offset..(hit_offset + (Self::HIT_SHADER_COUNT * handle_size) as usize)]
            .copy_from_slice(&sbt_handles_raw[hit_handle_start..]);

        let buffer = Buffer::new(&buffer::BufferDescriptor {
            device,
            ty: buffer::BufferType::Shared,
            data: Some(sbt_host_data.as_slice()),
            create_info: vk::BufferCreateInfo::default()
                .size(sbt_size as u64)
                .usage(
                    vk::BufferUsageFlags::SHADER_BINDING_TABLE_KHR
                        | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                )
                .sharing_mode(vk::SharingMode::EXCLUSIVE),
            alloc_info: vk_alloc::AllocationCreateDesc {
                name,
                requirements: vk::MemoryRequirements::default(),
                allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
                linear: false,
                location: gpu_allocator::MemoryLocation::CpuToGpu,
            },
        })?;
        let shader_stride = ShaderStride {
            raygen_stride: raygen_stride.into(),
            raygen_size: raygen_size.into(),
            miss_stride: miss_stride.into(),
            miss_size: miss_size.into(),
            hit_stride: hit_stride.into(),
            hit_size: hit_size.into(),
        };

        Ok((buffer, shader_stride))
    }
}
