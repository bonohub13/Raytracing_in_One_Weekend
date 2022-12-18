struct VkSwapChainDetail {
    capabities: ash::vk::SurfaceCapabilitiesKHR,
    format: ash::vk::SurfaceFormatKHR,
}

pub fn create_swapchain(
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    surface: &crate::VkSurface,
    device: &ash::Device,
    width: u32,
    height: u32,
    surface_capabilities: &ash::vk::SurfaceCapabilitiesKHR,
    present_mode: ash::vk::PresentModeKHR,
    surface_format: &ash::vk::SurfaceFormatKHR,
) -> Result<crate::VkSwapchain, String> {
    use crate::vk_init;
    use ash::{extensions::khr::Swapchain, vk};
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating VkSwapchain");

    log::info!("creating swap chain loader");
    let loader = Swapchain::new(instance, device);
    log::info!("created swap chain loader");

    let pre_transform = choose_swapchain_transform(surface_capabilities);
    let composite_alpha = choose_swapchain_composite(surface_capabilities);
    let extent = if surface_capabilities.current_extent.width == u32::MAX {
        vk::Extent2D { width, height }
    } else {
        surface_capabilities.current_extent
    };
    let desired_number_of_swapchain_images = if (surface_capabilities.min_image_count + 1)
        > surface_capabilities.max_image_count
        && surface_capabilities.max_image_count > 0
    {
        surface_capabilities.max_image_count
    } else {
        surface_capabilities.min_image_count + 1
    };
    let mut image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
    if surface_capabilities
        .supported_usage_flags
        .contains(vk::ImageUsageFlags::TRANSFER_SRC)
    {
        image_usage |= vk::ImageUsageFlags::TRANSFER_SRC;
    }
    if surface_capabilities
        .supported_usage_flags
        .contains(vk::ImageUsageFlags::TRANSFER_DST)
    {
        image_usage |= vk::ImageUsageFlags::TRANSFER_DST;
    }

    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface.surface)
        .min_image_count(desired_number_of_swapchain_images)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_usage(image_usage)
        .pre_transform(pre_transform)
        .image_array_layers(1)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .present_mode(present_mode)
        .old_swapchain(vk::SwapchainKHR::null())
        .clipped(true)
        .composite_alpha(composite_alpha)
        .build();

    log::info!("creating swap chain");

    let swapchain_sg = {
        let swapchain = unsafe {
            loader
                .create_swapchain(&create_info, None)
                .map_err(|_| String::from("failed to create swap chain"))?
        };

        guard(swapchain, |swapchain| {
            log::warn!("swap chain scopeguard");

            unsafe {
                loader.destroy_swapchain(swapchain, None);
            }
        })
    };

    log::info!("created swap chain with extent: {:?}", extent);

    let swapchain_images = get_swapchain_images(&loader, *swapchain_sg)?;
    let swapchain_image_views =
        create_swapchain_image_views(device, &swapchain_images, surface_format.format)?;

    log::info!("created VkSwapchain");

    Ok(crate::VkSwapchain {
        image_views: swapchain_image_views,
        images: swapchain_images,
        extent,
        swapchain: ScopeGuard::into_inner(swapchain_sg),
        loader,
    })
}

fn get_swapchain_images(
    swapchain_loader: &ash::extensions::khr::Swapchain,
    swapchain: ash::vk::SwapchainKHR,
) -> Result<Vec<ash::vk::Image>, String> {
    log::info!("finding swapchain images");
    let swapchain_images = unsafe {
        swapchain_loader
            .get_swapchain_images(swapchain)
            .map_err(|_| String::from("failed to get images swapchain"))?
    };

    if swapchain_images.is_empty() {
        return Err(String::from("failed to get any images swapchain"));
    }

    log::info!("found swapchain images");

    Ok(swapchain_images)
}

fn create_swapchain_image_views(
    device: &ash::Device,
    swapchain_images: &Vec<ash::vk::Image>,
    swapchain_format: ash::vk::Format,
) -> Result<Vec<ash::vk::ImageView>, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating swapchain image views");

    let mut swapchain_image_views = Vec::new();
    for &image in swapchain_images.iter() {
        let create_info = vk::ImageViewCreateInfo::builder()
            .format(swapchain_format)
            .components(vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            })
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .view_type(vk::ImageViewType::TYPE_2D)
            .image(image)
            .build();

        let iv_sg = {
            let image_view = unsafe {
                device
                    .create_image_view(&create_info, None)
                    .map_err(|_| String::from("failed to create image view"))?
            };

            guard(image_view, |iv| {
                log::warn!("image view scopeguard");

                unsafe {
                    device.destroy_image_view(iv, None);
                }
            })
        };

        swapchain_image_views.push(ScopeGuard::into_inner(iv_sg));
    }

    log::info!("created swapchain image views");

    Ok(swapchain_image_views)
}

pub fn choose_swapchain_present_mode(
    present_modes: &Vec<ash::vk::PresentModeKHR>,
) -> ash::vk::PresentModeKHR {
    use ash::vk;

    log::info!("finding suitable present mode");

    let mut present_mode = vk::PresentModeKHR::FIFO;

    for &current_present_mode in present_modes.iter() {
        if current_present_mode == vk::PresentModeKHR::MAILBOX {
            log::info!("found suitable present mode (MAILBOX)");

            return current_present_mode;
        }

        if present_mode != vk::PresentModeKHR::MAILBOX
            && current_present_mode == vk::PresentModeKHR::IMMEDIATE
        {
            present_mode = vk::PresentModeKHR::IMMEDIATE
        }
    }

    log::info!(
        "found suitable present mode ({})",
        match present_mode {
            vk::PresentModeKHR::FIFO => "[FIFO]",
            vk::PresentModeKHR::IMMEDIATE => "[IMMEDIATE]",
            _ => "[UNKNOWN]",
        }
    );

    present_mode
}

pub fn choose_swapchain_format(
    surface_formats: &Vec<ash::vk::SurfaceFormatKHR>,
) -> Result<ash::vk::SurfaceFormatKHR, String> {
    use ash::vk;

    log::info!("finding suitable surface format");

    let mut format = if surface_formats.is_empty() {
        None
    } else {
        Some(surface_formats[0])
    };

    for &surface_format in surface_formats.iter() {
        if surface_format.format == vk::Format::B8G8R8A8_UNORM {
            format = Some(surface_format);

            break;
        }
    }

    match format {
        Some(surface_format) => {
            log::info!("found suitable surface format ({:?})", surface_format);

            Ok(surface_format)
        }
        None => Err(String::from("failed to find suitable surface format")),
    }
}

pub fn choose_swapchain_transform(
    surface_capabilities: &ash::vk::SurfaceCapabilitiesKHR,
) -> ash::vk::SurfaceTransformFlagsKHR {
    use ash::vk;

    log::info!("finding suitable surface transform");

    let mut transform = vk::SurfaceTransformFlagsKHR::IDENTITY;

    if !surface_capabilities
        .supported_transforms
        .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
    {
        transform = surface_capabilities.current_transform
    }

    log::info!("found suitable surface transform ({:?})", transform);

    transform
}

pub fn choose_swapchain_composite(
    surface_capabilities: &ash::vk::SurfaceCapabilitiesKHR,
) -> ash::vk::CompositeAlphaFlagsKHR {
    use ash::vk;

    log::info!("finding suitable surface composite alpha");

    let mut composite_flags = vk::CompositeAlphaFlagsKHR::OPAQUE;
    for &composite_alpha_flags in vec![
        vk::CompositeAlphaFlagsKHR::OPAQUE,
        vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
        vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
        vk::CompositeAlphaFlagsKHR::INHERIT,
    ]
    .iter()
    {
        if surface_capabilities
            .supported_composite_alpha
            .contains(composite_flags)
        {
            composite_flags = composite_alpha_flags;

            break;
        }
    }

    log::info!(
        "found suitable surface composite alpha ({:?})",
        composite_flags
    );

    composite_flags
}

fn query_swapchain_support(
    physical_device: ash::vk::PhysicalDevice,
    surface: &crate::VkSurface,
    format: ash::vk::Format,
    color_space: ash::vk::ColorSpaceKHR,
) -> Result<VkSwapChainDetail, String> {
    log::info!("querying swap chain support");

    let capabities = surface.get_physical_device_surface_capabilities(physical_device)?;
    let format =
        surface.find_suitable_swap_chain_surface_format(physical_device, format, color_space)?;

    log::info!("querying swap chain support");

    Ok(VkSwapChainDetail { capabities, format })
}
