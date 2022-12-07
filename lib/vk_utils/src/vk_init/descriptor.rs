pub fn create_descriptor_set_layout(
    device: &ash::Device,
) -> Result<ash::vk::DescriptorSetLayout, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let bindings = vec![
        vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .build(),
        vk::DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .build(),
        vk::DescriptorSetLayoutBinding::builder()
            .binding(2)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .build(),
        vk::DescriptorSetLayoutBinding::builder()
            .binding(3)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .build(),
    ];
    let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(&bindings)
        .build();

    log::info!("creating descriptor set layout");

    let descriptor_set_layout_sg = {
        let descriptor_set_layout = unsafe {
            device
                .create_descriptor_set_layout(&create_info, None)
                .map_err(|_| String::from("failed to create descriptor set layout"))?
        };

        guard(descriptor_set_layout, |layout| {
            log::warn!("descriptor set layout scopeguard");

            unsafe {
                device.destroy_descriptor_set_layout(layout, None);
            }
        })
    };

    log::info!("created descriptor set layout");

    Ok(ScopeGuard::into_inner(descriptor_set_layout_sg))
}

pub fn create_descriptor_pool(device: &ash::Device) -> Result<ash::vk::DescriptorPool, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating descriptor pool");

    let pool_size = vec![
        vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::STORAGE_IMAGE)
            .descriptor_count(2)
            .build(),
        vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(2)
            .build(),
    ];
    let create_info = vk::DescriptorPoolCreateInfo::builder()
        .max_sets(1)
        .pool_sizes(&pool_size)
        .build();

    let pool_sg = {
        let pool = unsafe {
            device
                .create_descriptor_pool(&create_info, None)
                .map_err(|_| String::from("failed to create descriptor pool"))?
        };

        guard(pool, |pool| {
            log::warn!("descriptor pool scopeguard");

            unsafe {
                device.destroy_descriptor_pool(pool, None);
            }
        })
    };

    log::info!("created descriptor pool");

    Ok(ScopeGuard::into_inner(pool_sg))
}

pub fn create_descriptor_sets() {}
