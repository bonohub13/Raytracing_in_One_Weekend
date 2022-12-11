pub fn create_descriptor_set_layout(
    device: &ash::Device,
    descriptor_bindings: &Vec<ash::vk::DescriptorSetLayoutBinding>,
) -> Result<ash::vk::DescriptorSetLayout, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(descriptor_bindings)
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

pub fn create_descriptor_pool(
    device: &ash::Device,
    pool_sizes: &Vec<ash::vk::DescriptorPoolSize>,
    max_sets: u32,
) -> Result<ash::vk::DescriptorPool, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating descriptor pool");

    let create_info = vk::DescriptorPoolCreateInfo::builder()
        .max_sets(max_sets)
        .pool_sizes(pool_sizes)
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

pub fn create_descriptor_set(
    device: &ash::Device,
    descriptor_set_layout: ash::vk::DescriptorSetLayout,
    descriptor_pool: ash::vk::DescriptorPool,
) -> Result<ash::vk::DescriptorSet, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let alloc_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(descriptor_pool)
        .set_layouts(&[descriptor_set_layout])
        .build();

    log::info!("creating descriptor set");

    // descriptor set scopeguard
    let ds_sg = {
        let descriptor_set = unsafe {
            device
                .allocate_descriptor_sets(&alloc_info)
                .map_err(|_| String::from("failed to allocate descriptor set"))?
        }[0];

        guard(descriptor_set, |ds| {
            log::info!("descriptor set scopeguard");

            match unsafe { device.free_descriptor_sets(descriptor_pool, &[ds]) } {
                Ok(_) => (),
                Err(_) => {
                    log::error!("failed to free descriptor set");
                }
            }
        })
    };

    log::info!("created descriptor set");

    Ok(ScopeGuard::into_inner(ds_sg))
}

pub fn update_descriptor_sets(
    device: &ash::Device,
    descriptor_writes: &Vec<ash::vk::WriteDescriptorSet>,
) {
    log::info!("updating descriptor sets");

    unsafe {
        device.update_descriptor_sets(descriptor_writes, &[]);
    }

    log::info!("updated descriptor sets");
}
