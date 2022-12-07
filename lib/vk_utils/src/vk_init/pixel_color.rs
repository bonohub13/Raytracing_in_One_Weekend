pub fn create_summed_pixel_color_image(
    settings: &crate::VkSettings,
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    device: &ash::Device,
    format: ash::vk::Format,
) -> Result<crate::VkImage, String> {
    use crate::vk_init;
    use ash::vk;

    log::info!("creating pixel color image");

    let summed_pixel_color_image = vk_init::create_image(
        settings,
        instance,
        physical_device,
        device,
        format,
        vk::ImageUsageFlags::STORAGE,
    )
    .map_err(|err| {
        log::error!("{}", err);

        String::from("failed to create pixel color image")
    })?;

    log::info!("created pixel color image");

    Ok(summed_pixel_color_image)
}
