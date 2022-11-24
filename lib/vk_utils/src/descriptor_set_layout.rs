pub struct DescriptorSetLayout {
    pub layout: ash::vk::DescriptorSetLayout,
}

impl DescriptorSetLayout {
    pub fn new(
        device: &crate::Device,
        descriptor_bindings: &Vec<crate::DescriptorBinding>,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating DescriptorSetLayout");

        let layout_bindings: Vec<vk::DescriptorSetLayoutBinding> = descriptor_bindings
            .iter()
            .map(|binding| {
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(binding.binding)
                    .descriptor_count(binding.descriptor_count)
                    .descriptor_type(binding.type_)
                    .stage_flags(binding.stage)
                    .build()
            })
            .collect();

        let layout_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(&layout_bindings)
            .build();

        let layout = device.create_descriptor_set_layout(&layout_info, None)?;

        log::info!("created DescriptorSetLayout");

        Ok(Self { layout })
    }

    pub fn cleanup(device: &crate::Device, layout: &mut Self) {
        log::info!("performing cleanup for DescriptorSetLayout");

        device.destroy_descriptor_set_layout(layout.layout, None);
    }
}
