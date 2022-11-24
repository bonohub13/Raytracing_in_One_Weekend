pub struct ImageView {
    pub image_view: ash::vk::ImageView,
    image: ash::vk::Image,
    format: ash::vk::Format,
}

impl ImageView {
    pub fn new(
        device: &crate::Device,
        image: ash::vk::Image,
        format: ash::vk::Format,
        aspect_flags: ash::vk::ImageAspectFlags,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating ImageView");

        let create_info = vk::ImageViewCreateInfo::builder()
            .image(image)
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
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .build();

        let image_view = device.create_image_view(&create_info, None)?;

        log::info!("created ImageView");

        Ok(Self {
            image_view,
            image,
            format,
        })
    }

    pub fn cleanup(device: &crate::Device, image_view: &mut Self) {
        log::info!("performing cleanup for ImageView");

        device.destroy_image_view(image_view.image_view, None);
    }
}
