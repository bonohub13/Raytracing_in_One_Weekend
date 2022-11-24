pub struct DescriptorSetManager {
    pub pool: crate::DescriptorPool,
    pub layout: crate::DescriptorSetLayout,
}

impl DescriptorSetManager {
    pub fn new(
        device: &crate::Device,
        descriptor_bindings: &Vec<crate::DescriptorBinding>,
        max_sets: u32,
    ) -> Result<Self, String> {
        use ash::vk;
        use std::collections::HashMap;

        log::info!("creating DescriptorSetManager");

        let mut bindings: HashMap<u32, vk::DescriptorType> = HashMap::new();
        for binding in descriptor_bindings.iter() {
            match bindings.insert(binding.binding, binding.type_) {
                Some(_) => {}
                None => return Err(String::from("binding collision")),
            }
        }

        let descriptor_pool = crate::DescriptorPool::new(device, descriptor_bindings, max_sets)?;
        let descriptor_set_layout = crate::DescriptorSetLayout::new(device, descriptor_bindings)?;

        log::info!("created DescriptorSetManager");

        Ok(Self {
            pool: descriptor_pool,
            layout: descriptor_set_layout,
        })
    }

    pub fn cleanup(device: &crate::Device, dsm: &mut Self) {
        log::info!("performing cleanup for DescriptorSetManager");

        crate::DescriptorSetLayout::cleanup(device, &mut dsm.layout);
        crate::DescriptorPool::cleanup(device, &mut dsm.pool);
    }
}
