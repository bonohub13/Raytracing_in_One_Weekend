pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn new(graphics_family: Option<u32>, present_family: Option<u32>) -> Self {
        Self {
            graphics_family,
            present_family,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    pub fn sharing_mode(&self) -> (ash::vk::SharingMode, Vec<u32>) {
        use ash::vk;

        if self.graphics_family.unwrap() != self.present_family.unwrap() {
            (
                vk::SharingMode::CONCURRENT,
                vec![self.graphics_family.unwrap(), self.present_family.unwrap()],
            )
        } else {
            (vk::SharingMode::EXCLUSIVE, vec![])
        }
    }

    pub fn device_queues(
        &self,
        device: &ash::Device,
        graphics_queue_index: u32,
        present_queue_index: u32,
    ) -> Result<(ash::vk::Queue, ash::vk::Queue), String> {
        if self.is_complete() {
            Ok(unsafe {
                (
                    device.get_device_queue(self.graphics_family.unwrap(), graphics_queue_index),
                    device.get_device_queue(self.present_family.unwrap(), present_queue_index),
                )
            })
        } else {
            match (
                self.graphics_family.is_none(),
                self.present_family.is_none(),
            ) {
                (true, false) => Err(String::from(
                    "failed to get device queue for graphics queue.",
                )),
                (false, true) => Err(String::from(
                    "failed to get device queue for present queue.",
                )),
                _ => Err(String::from("failed to get device queues for both queues.")),
            }
        }
    }
}
