pub struct Semaphore {
    pub semaphore: ash::vk::Semaphore,
}

impl Semaphore {
    pub fn new(device: &crate::Device) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating Semaphore");

        let semaphore_info = vk::SemaphoreCreateInfo::builder().build();

        let semaphore = device.create_semaphore(&semaphore_info, None)?;

        log::info!("created Semaphore");

        Ok(Self { semaphore })
    }

    pub fn cleanup(device: &crate::Device, semaphore: &mut Self) {
        log::info!("performing cleanup for Semaphore");

        device.destroy_semaphore(semaphore.semaphore, None);
    }
}
