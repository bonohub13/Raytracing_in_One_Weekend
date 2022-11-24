pub struct CommandBuffers {
    pub command_buffers: Vec<ash::vk::CommandBuffer>,
}

impl CommandBuffers {
    pub fn new(
        device: &crate::Device,
        pool: &crate::CommandPool,
        size: u32,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating CommandBuffers");

        let alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(size)
            .build();

        let command_buffers = device.allocate_command_buffer(pool, &alloc_info)?;

        log::info!("created CommandBuffers");

        Ok(Self { command_buffers })
    }

    pub fn size(&self) -> usize {
        self.command_buffers.len()
    }

    pub fn at(&self, index: usize) -> ash::vk::CommandBuffer {
        self.command_buffers[index]
    }

    pub fn begin(&self, device: &crate::Device, index: usize) -> ash::vk::CommandBuffer {
        use ash::vk;

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)
            .build();

        device.begin_command_buffer(self.command_buffers[index], &begin_info);

        self.command_buffers[index]
    }

    pub fn end(&self, device: &crate::Device, index: usize) {
        device.end_command_buffer(self.command_buffers[index]);
    }

    pub fn cleanup(device: &crate::Device, pool: &crate::CommandPool, command_buffers: &mut Self) {
        log::info!("performing cleanup for CommandBuffers");

        device.free_command_buffers(pool, &command_buffers.command_buffers);
    }
}
