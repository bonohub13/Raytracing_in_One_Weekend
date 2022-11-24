pub struct DescriptorPool {
    pub descriptor_pool: ash::vk::DescriptorPool,
}

impl DescriptorPool {
    pub fn new(
        device: &crate::Device,
        descriptor_bindings: &Vec<crate::DescriptorBinding>,
        max_sets: u32,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating DescriptorPool");

        let pool_sizes: Vec<vk::DescriptorPoolSize> = descriptor_bindings
            .iter()
            .map(|binding| {
                vk::DescriptorPoolSize::builder()
                    .ty(binding.type_)
                    .descriptor_count(binding.descriptor_count * max_sets)
                    .build()
            })
            .collect();

        let pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(&pool_sizes)
            .max_sets(max_sets)
            .build();

        let descriptor_pool = device.create_descriptor_pool(&pool_info, None)?;

        log::info!("created DescriptorPool");

        Ok(Self { descriptor_pool })
    }

    pub fn cleanup(device: &crate::Device, pool: &mut Self) {
        log::info!("performing cleanup for DescriptorPool");

        device.destroy_descriptor_pool(pool.descriptor_pool, None);
    }
}
