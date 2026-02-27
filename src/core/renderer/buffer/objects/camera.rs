use ash::vk;
use gpu_allocator::vulkan as vk_alloc;

pub struct CameraDescriptor {}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {}

impl Camera {
    pub fn new(desc: &CameraDescriptor) -> Self {
        Self {}
    }

    pub fn layout_binding() -> vk::DescriptorSetLayoutBinding<'static> {
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::RAYGEN_KHR | vk::ShaderStageFlags::CLOSEST_HIT_KHR)
    }

    pub fn buffer_create_info() -> vk::BufferCreateInfo<'static> {
        let buffer_size = size_of::<Self>() as u64;

        vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
    }

    pub fn allocation_info() -> vk_alloc::AllocationCreateDesc<'static> {
        const BUFFER_NAME: &str = "Camera buffer allocation";

        vk_alloc::AllocationCreateDesc {
            name: BUFFER_NAME,
            requirements: vk::MemoryRequirements::default(),
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            linear: false,
            allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
        }
    }
}
