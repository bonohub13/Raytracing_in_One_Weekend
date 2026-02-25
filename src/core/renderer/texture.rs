use crate::core::device::Device;
use ash::vk;
use std::sync::Arc;

pub struct Texture {
    device: Arc<Device>,
    image: vk::Image,
    image_view: vk::ImageView,
    sampler: vk::Sampler,
}

impl Drop for Texture {
    fn drop(&mut self) {
        let device = self.device.raw();

        unsafe {
            device.destroy_sampler(self.sampler, None);
            device.destroy_image_view(self.image_view, None);
            device.destroy_image(self.image, None);
        }
    }
}
