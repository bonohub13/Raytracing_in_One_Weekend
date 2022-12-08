pub fn create_semaphore(
    device: &ash::Device,
    semaphore_type: &str,
) -> Result<ash::vk::Semaphore, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating semaphore ({})", semaphore_type);

    let create_info = vk::SemaphoreCreateInfo::builder().build();

    let semaphore_sg = {
        let semaphore = unsafe {
            device
                .create_semaphore(&create_info, None)
                .map_err(|_| format!("failed to create semaphore ({})", semaphore_type))?
        };

        guard(semaphore, |semaphore| {
            log::warn!("semaphore scopeguard");

            unsafe {
                device.destroy_semaphore(semaphore, None);
            }
        })
    };

    log::info!("created semaphore ({})", semaphore_type);

    Ok(ScopeGuard::into_inner(semaphore_sg))
}
