use crate::core::{
    RtError, RtResult,
    device::Device,
    params,
    renderer::buffer::{
        self, Aabb, Buffer, DescriptorSet,
        blas::{self, BottomLevelAs},
        objects::{Camera, CameraDescriptor},
    },
};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;

pub struct AccelerationBufferDescriptor {
    pub device: Arc<Device>,
    pub camera_desc: CameraDescriptor,
}

pub struct AccelerationBuffer {
    device: Arc<Device>,
    camera_buffer: Buffer<Camera>,
    camera: Camera,
    aabb_buffer: Buffer<Aabb>,
    scratch_buffer: Buffer<()>,
}

impl AccelerationBuffer {
    const PER_DESCRIPTOR_SET_COUNT: u32 = params::MAX_FRAMES_IN_FLIGHT as u32;

    pub fn new(desc: &AccelerationBufferDescriptor) -> RtResult<Self> {
        const SPHERE_AABB: Aabb = buffer::Sphere::aabb();

        let camera = Camera::new(&desc.camera_desc);
        let camera_buffer = Buffer::new(&buffer::BufferDescriptor {
            device: desc.device.clone(),
            data: Some(std::slice::from_ref(&camera)),
            ty: buffer::BufferType::ExlusiveToFrame,
            create_info: Camera::buffer_create_info(),
            alloc_info: Camera::allocation_info(),
        })?;
        let aabbs = [SPHERE_AABB];
        let aabb_buffer = Buffer::new(&buffer::BufferDescriptor {
            device: desc.device.clone(),
            data: Some(&aabbs),
            ty: buffer::BufferType::Shared,
            create_info: buffer::Aabb::buffer_create_info(),
            alloc_info: buffer::Aabb::allocation_info(),
        })?;
        let geometries = Self::create_geometry(desc.device.clone(), &aabbs, &aabb_buffer)?;
        let size_info = {
            let mut size_info = vk::AccelerationStructureBuildSizesInfoKHR::default();

            Self::query_build_sizes_info(desc.device.clone(), &aabbs, &geometries, &mut size_info);

            size_info
        };
        let scratch_buffer = Self::create_scratch_buffer(desc.device.clone(), &size_info)?;
        /*
        let blas = BottomLevelAs::new(&blas::BottomLevelAsDescriptor {
            device: desc.device.clone(),
            aabb_data: &aabbs,
            aabb_buffer: &aabb_buffer,
            size_info,
        })?;
        */

        Ok(Self {
            device: desc.device.clone(),
            camera,
            camera_buffer,
            aabb_buffer,
            scratch_buffer,
        })
    }

    pub fn update_camera(&mut self) -> RtResult<()> {
        self.camera_buffer.write(std::slice::from_ref(&self.camera))
    }

    pub fn write_descriptor_sets(&self, descriptor_set: &DescriptorSet, current_frame: usize) {
        let buffer_info = self
            .camera_buffer
            .buffer_info(std::slice::from_ref(&self.camera), current_frame);
        let write = vk::WriteDescriptorSet::default()
            .dst_set(descriptor_set.acceleration_sets()[current_frame])
            .dst_binding(1)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .buffer_info(std::slice::from_ref(&buffer_info));

        unsafe {
            self.device
                .raw()
                .update_descriptor_sets(std::slice::from_ref(&write), &[])
        }
    }

    pub fn layout_binding() -> Vec<vk::DescriptorSetLayoutBinding<'static>> {
        vec![
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(
                    vk::ShaderStageFlags::RAYGEN_KHR | vk::ShaderStageFlags::CLOSEST_HIT_KHR,
                ),
            vk::DescriptorSetLayoutBinding::default()
                .binding(1)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(
                    vk::ShaderStageFlags::RAYGEN_KHR | vk::ShaderStageFlags::CLOSEST_HIT_KHR,
                ),
        ]
    }

    pub fn pool_sizes() -> Vec<vk::DescriptorPoolSize> {
        vec![
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(Self::PER_DESCRIPTOR_SET_COUNT),
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(Self::PER_DESCRIPTOR_SET_COUNT),
        ]
    }

    fn create_geometry<'geom>(
        device: Arc<Device>,
        aabb_data: &[Aabb],
        aabb_buffer: &Buffer<Aabb>,
    ) -> RtResult<Vec<vk::AccelerationStructureGeometryKHR<'geom>>> {
        if aabb_data.is_empty() {
            return Err(RtError::NoGemoetryData);
        }

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

        Ok(match aabb_geometry {
            Some(aabb_geometry) => vec![aabb_geometry],
            None => vec![],
        })
    }

    fn query_build_sizes_info(
        device: Arc<Device>,
        aabb_data: &[Aabb],
        geometries: &[vk::AccelerationStructureGeometryKHR],
        size_info: &mut vk::AccelerationStructureBuildSizesInfoKHR,
    ) {
        let build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(&geometries);
        let primitive_counts = [
            // AABBs
            aabb_data.len() as u32,
        ];

        unsafe {
            device.rt_loader().get_acceleration_structure_build_sizes(
                vk::AccelerationStructureBuildTypeKHR::DEVICE,
                &build_info,
                &primitive_counts,
                size_info,
            )
        }
    }

    fn create_scratch_buffer(
        device: Arc<Device>,
        size_info: &vk::AccelerationStructureBuildSizesInfoKHR,
    ) -> RtResult<Buffer<()>> {
        const ALLOCATION_NAME: &str = "Scratch buffer allocation";

        let scratch_size = size_info.update_scratch_size;

        Buffer::new(&buffer::BufferDescriptor {
            device,
            data: None,
            ty: buffer::BufferType::Shared,
            create_info: vk::BufferCreateInfo::default().size(scratch_size).usage(
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            ),
            alloc_info: vk_alloc::AllocationCreateDesc {
                name: ALLOCATION_NAME,
                requirements: vk::MemoryRequirements::default(),
                location: gpu_allocator::MemoryLocation::GpuOnly,
                linear: false,
                allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
            },
        })
    }
}
