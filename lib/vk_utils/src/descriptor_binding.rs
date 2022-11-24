pub struct DescriptorBinding {
    pub binding: u32,
    pub descriptor_count: u32,
    pub type_: ash::vk::DescriptorType,
    pub stage: ash::vk::ShaderStageFlags,
}
