pub fn create_image_views(
    device: &ash::Device,
    surface_format: ash::vk::Format,
    images: &Vec<ash::vk::Image>,
) -> Result<Vec<ash::vk::ImageView>, String> {
    use ash::vk;

    let mut swapchain_image_views: Vec<vk::ImageView> = Vec::new();

    for &image in images.iter() {
        let result = create_image_view(
            device,
            image,
            surface_format,
            vk::ImageAspectFlags::COLOR,
            1,
        );

        if result.is_err() {
            return Err(result.err().unwrap());
        }

        swapchain_image_views.push(result.unwrap());
    }

    Ok(swapchain_image_views)
}

pub fn create_image_view(
    device: &ash::Device,
    texture_image: ash::vk::Image,
    format: ash::vk::Format,
    aspect_flags: ash::vk::ImageAspectFlags,
    mip_levels: u32,
) -> Result<ash::vk::ImageView, String> {
    use ash::vk;

    let view_info = vk::ImageViewCreateInfo::builder()
        .image(texture_image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .components(vk::ComponentMapping {
            r: vk::ComponentSwizzle::IDENTITY,
            g: vk::ComponentSwizzle::IDENTITY,
            b: vk::ComponentSwizzle::IDENTITY,
            a: vk::ComponentSwizzle::IDENTITY,
        })
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: aspect_flags,
            base_mip_level: 0,
            level_count: mip_levels,
            base_array_layer: 0,
            layer_count: 1,
        });

    let result = unsafe { device.create_image_view(&view_info, None) };

    match result {
        Ok(image_view) => Ok(image_view),
        Err(_) => Err(String::from("failed to create texture image view!")),
    }
}
