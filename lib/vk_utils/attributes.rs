pub trait Vertex {
    fn get_binding_description() -> Vec<ash::vk::VertexInputBindingDescription>;
    fn get_attribute_descriptions() -> Vec<ash::vk::VertexInputAttributeDescription>;
}

pub trait Pipeline {
    fn create_graphics_pipeline(
        device: &ash::Device,
        msaa_samples: ash::vk::SampleCountFlags,
        swapchain_extent: ash::vk::Extent2D,
        render_pass: ash::vk::RenderPass,
        descriptor_set_layout: ash::vk::DescriptorSetLayout,
        vertex_shader_path: &std::path::Path,
        fragment_shader_path: &std::path::Path,
    ) -> Result<(ash::vk::Pipeline, ash::vk::PipelineLayout), String>;
}
