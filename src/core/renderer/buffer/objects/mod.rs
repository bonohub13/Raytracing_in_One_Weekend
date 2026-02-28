mod camera;
mod sphere;

pub use camera::*;
pub use sphere::Sphere;

use ash::vk;
use gpu_allocator::vulkan as vk_alloc;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl Aabb {
    pub fn buffer_create_info() -> vk::BufferCreateInfo<'static> {
        let buffer_size = size_of::<Self>() as u64;

        vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(
                vk::BufferUsageFlags::STORAGE_BUFFER
                    | vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
    }

    pub fn allocation_info() -> vk_alloc::AllocationCreateDesc<'static> {
        const BUFFER_NAME: &str = "AABB buffer allocation";

        vk_alloc::AllocationCreateDesc {
            name: BUFFER_NAME,
            requirements: vk::MemoryRequirements::default(),
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            linear: false,
            allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
        }
    }
}
