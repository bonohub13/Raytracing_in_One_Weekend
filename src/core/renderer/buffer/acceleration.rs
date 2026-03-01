use crate::core::{
    RtResult,
    device::Device,
    params,
    renderer::{
        buffer::{
            self, Aabb, Buffer, DescriptorSet,
            blas::{self, BottomLevelAs},
            objects::{Camera, CameraDescriptor},
        },
        command::Command,
        sync::SyncObject,
    },
};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;

pub struct AccelerationBufferDescriptor<'desc> {
    pub device: Arc<Device>,
    pub sync_object: &'desc SyncObject,
    pub command: &'desc Command,
    pub camera_desc: CameraDescriptor,
}

pub struct AccelerationBuffer {
    device: Arc<Device>,
    camera_buffer: Buffer<Camera>,
    camera: Camera,
    aabb_buffer: Buffer<Aabb>,
    blas: BottomLevelAs,
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
        let blas = BottomLevelAs::new(&blas::BottomLevelAsDescriptor {
            device: desc.device.clone(),
            aabb_data: &aabbs,
            aabb_buffer: &aabb_buffer,
            sync_object: desc.sync_object,
            command: desc.command,
        })?;

        Ok(Self {
            device: desc.device.clone(),
            camera,
            camera_buffer,
            aabb_buffer,
            blas,
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
            .dst_binding(2)
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
                .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
                .descriptor_count(1)
                .stage_flags(
                    vk::ShaderStageFlags::RAYGEN_KHR | vk::ShaderStageFlags::CLOSEST_HIT_KHR,
                ),
            vk::DescriptorSetLayoutBinding::default()
                .binding(1)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(
                    vk::ShaderStageFlags::RAYGEN_KHR | vk::ShaderStageFlags::CLOSEST_HIT_KHR,
                ),
            vk::DescriptorSetLayoutBinding::default()
                .binding(2)
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
                .ty(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
                .descriptor_count(Self::PER_DESCRIPTOR_SET_COUNT),
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(Self::PER_DESCRIPTOR_SET_COUNT),
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(Self::PER_DESCRIPTOR_SET_COUNT),
        ]
    }
}

pub fn create_scratch_buffer(device: Arc<Device>, scratch_size: u64) -> RtResult<Buffer<()>> {
    const ALLOCATION_NAME: &str = "Scratch buffer allocation";
    const ALIGNMENT: u64 = 0x100;

    Buffer::new(&buffer::BufferDescriptor {
        device,
        data: None,
        ty: buffer::BufferType::Shared,
        create_info: vk::BufferCreateInfo::default()
            .size(scratch_size + ALIGNMENT)
            .usage(
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE),
        alloc_info: vk_alloc::AllocationCreateDesc {
            name: ALLOCATION_NAME,
            requirements: vk::MemoryRequirements::default(),
            location: gpu_allocator::MemoryLocation::GpuOnly,
            linear: false,
            allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
        },
    })
}
