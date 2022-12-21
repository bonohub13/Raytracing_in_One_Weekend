pub fn create_command_pool(
    device: &ash::Device,
    queue_family_index: u32,
    pool_type: &str,
) -> Result<ash::vk::CommandPool, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let create_info = vk::CommandPoolCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .build();

    log::info!("creating command pool ({})", pool_type);

    let command_pool_sg = {
        let command_pool = unsafe {
            device
                .create_command_pool(&create_info, None)
                .map_err(|_| format!("failed to create command pool ({})", pool_type))?
        };

        guard(command_pool, |pool| {
            log::warn!("command pool scopeguard ({})", pool_type);

            unsafe {
                device.destroy_command_pool(pool, None);
            }
        })
    };

    log::info!("created command pool ({})", pool_type);

    Ok(ScopeGuard::into_inner(command_pool_sg))
}

pub fn create_command_buffer(
    device: &ash::Device,
    command_pool: &ash::vk::CommandPool,
) -> Result<ash::vk::CommandBuffer, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating command buffer");

    let alloc_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(*command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1)
        .build();

    let command_buffer_sg = {
        let command_buffer = unsafe {
            device
                .allocate_command_buffers(&alloc_info)
                .map_err(|_| String::from("failed to allocate command buffer"))?
        }[0];

        guard(command_buffer, |buffer| {
            log::warn!("command buffer scopeguard");

            unsafe {
                device.free_command_buffers(*command_pool, &[buffer]);
            }
        })
    };

    log::info!("created command buffer");

    Ok(ScopeGuard::into_inner(command_buffer_sg))
}

pub fn create_command_buffers(
    device: &ash::Device,
    command_pool: &ash::vk::CommandPool,
    size: usize,
) -> Result<Vec<ash::vk::CommandBuffer>, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating command_buffers");

    let alloc_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(*command_pool)
        .command_buffer_count(size as u32)
        .level(vk::CommandBufferLevel::PRIMARY)
        .build();

    let cb_sg = {
        let commandbuffers = unsafe {
            device
                .allocate_command_buffers(&alloc_info)
                .map_err(|_| String::from("failed to create command buffers"))?
        };

        guard(commandbuffers, |buffers| {
            log::warn!("command buffers scopeguard");

            unsafe {
                device.free_command_buffers(*command_pool, &buffers);
            }
        })
    };

    log::info!("created command_buffers");

    Ok(ScopeGuard::into_inner(cb_sg))
}
