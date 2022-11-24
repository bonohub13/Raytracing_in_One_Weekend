pub struct CommandPool {
    pub command_pool: ash::vk::CommandPool,
}

impl CommandPool {
    pub fn new(
        device: &crate::Device,
        queue_family_index: u32,
        allow_reset: bool,
    ) -> Result<Self, String> {
        use ash::vk;

        let pool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .flags(if allow_reset {
                vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
            } else {
                vk::CommandPoolCreateFlags::empty()
            })
            .build();

        let command_pool = device.create_command_pool(&pool_info, None)?;

        Ok(Self { command_pool })
    }

    pub fn cleanup(device: &crate::Device, pool: &mut Self) {
        device.destroy_command_pool(pool.command_pool, None);
    }
}
