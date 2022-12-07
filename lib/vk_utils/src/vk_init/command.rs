pub fn create_command_pool(
    device: &ash::Device,
    queue_family_index: u32,
) -> Result<ash::vk::CommandPool, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let create_info = vk::CommandPoolCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .build();

    log::info!("creating command pool");

    let command_pool_sg = {
        let command_pool = unsafe {
            device
                .create_command_pool(&create_info, None)
                .map_err(|_| String::from("failed to create command pool"))?
        };

        guard(command_pool, |pool| {
            log::warn!("command pool scopeguard");

            unsafe {
                device.destroy_command_pool(pool, None);
            }
        })
    };

    log::info!("created command pool");

    Ok(ScopeGuard::into_inner(command_pool_sg))
}
