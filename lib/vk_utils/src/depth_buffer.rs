pub struct DepthBuffer {
    format: ash::vk::Format,
    image: crate::Image,
    image_memory: crate::DeviceMemory,
    image_view: crate::ImageView,
}

impl DepthBuffer {
    pub fn new(
        instance: &crate::Instance,
        device: &crate::Device,
        pool: &crate::CommandPool,
        extent: ash::vk::Extent2D,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating DepthBuffer");

        let format = find_depth_format(instance, device)?;

        let mut image = crate::Image::new(
            device,
            extent,
            format,
            Some(vk::ImageTiling::OPTIMAL),
            Some(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT),
        )?;
        let image_memory =
            image.allocate_memory(instance, device, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;
        let image_view =
            crate::ImageView::new(device, image.image, format, vk::ImageAspectFlags::DEPTH)?;

        image.transition_image_layout(
            device,
            pool,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        );

        log::info!("created DepthBuffer");

        Ok(Self {
            format,
            image_view,
            image_memory,
            image,
        })
    }

    pub fn format(&self) -> ash::vk::Format {
        self.format
    }

    pub fn has_stencil_component(format: ash::vk::Format) -> bool {
        use ash::vk::Format;

        format == Format::D32_SFLOAT_S8_UINT || format == Format::D24_UNORM_S8_UINT
    }

    pub fn cleanup(device: &crate::Device, depth_buffer: &mut Self) {
        crate::ImageView::cleanup(device, &mut depth_buffer.image_view);
        crate::Image::cleanup(device, &mut depth_buffer.image);
        crate::DeviceMemory::cleanup(device, &mut depth_buffer.image_memory);
    }
}

fn find_support_format(
    instance: &crate::Instance,
    device: &crate::Device,
    candidates: &Vec<ash::vk::Format>,
    tiling: ash::vk::ImageTiling,
    features: ash::vk::FormatFeatureFlags,
) -> Result<ash::vk::Format, String> {
    use ash::vk;

    match candidates.iter().find(|&format| {
        let props =
            instance.get_physical_device_format_properties(device.physical_device(), *format);

        let linear_tiling =
            tiling == vk::ImageTiling::LINEAR && props.linear_tiling_features.contains(features);
        let optimal_tiling =
            tiling == vk::ImageTiling::OPTIMAL && props.optimal_tiling_features.contains(features);

        linear_tiling || optimal_tiling
    }) {
        Some(&format) => Ok(format),
        None => Err(String::from("failed to find supported format")),
    }
}

fn find_depth_format(
    instance: &crate::Instance,
    device: &crate::Device,
) -> Result<ash::vk::Format, String> {
    use ash::vk;

    find_support_format(
        instance,
        device,
        &vec![
            vk::Format::D32_SFLOAT,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::Format::D24_UNORM_S8_UINT,
        ],
        vk::ImageTiling::OPTIMAL,
        vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
    )
}
