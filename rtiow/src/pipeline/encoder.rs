use crate::Device;
use ash::vk;
use std::sync::Arc;

pub struct Encoder {
    command_buffers: Vec<vk::CommandBuffer>,
    pool: vk::CommandPool,
    device: Arc<Device>,
}

impl Encoder {
    pub fn command_buffers(&self) -> &[vk::CommandBuffer] {
        &self.command_buffers
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_command_pool(self.pool, None);
        }
    }
}
