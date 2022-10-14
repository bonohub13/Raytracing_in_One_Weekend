pub struct VkSwapchainDetail {
    pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<ash::vk::SurfaceFormatKHR>,
    pub present_modes: Vec<ash::vk::PresentModeKHR>,
}

pub struct VkSwapchainInfo {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: ash::vk::SwapchainKHR,
    pub swapchain_images: Vec<ash::vk::Image>,
    pub swapchain_format: ash::vk::Format,
    pub swapchain_extent: ash::vk::Extent2D,
}

impl VkSwapchainDetail {
    #[inline]
    pub fn query_swapchain_support(
        physical_device: ash::vk::PhysicalDevice,
        surface_info: &crate::surface::VkSurfaceInfo,
    ) -> Result<VkSwapchainDetail, String> {
        let capabilities = unsafe {
            surface_info
                .surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface_info.surface)
        };

        if capabilities.is_err() {
            return Err(String::from("failed to query for surface capabilities."));
        }

        let formats = unsafe {
            surface_info
                .surface_loader
                .get_physical_device_surface_formats(physical_device, surface_info.surface)
        };

        if formats.is_err() {
            return Err(String::from("failed to query for surface formats."));
        }

        let present_modes = unsafe {
            surface_info
                .surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface_info.surface)
        };

        if present_modes.is_err() {
            return Err(String::from("failed to query for surface present modes."));
        }

        Ok(VkSwapchainDetail {
            capabilities: capabilities.unwrap(),
            formats: formats.unwrap(),
            present_modes: present_modes.unwrap(),
        })
    }

    #[inline]
    pub fn image_count(&self) -> u32 {
        if self.capabilities.max_image_count > 0
            && (self.capabilities.min_image_count + 1) > self.capabilities.max_image_count
        {
            self.capabilities.max_image_count
        } else {
            self.capabilities.min_image_count
        }
    }

    #[inline]
    pub fn pre_transform(&self) -> ash::vk::SurfaceTransformFlagsKHR {
        use ash::vk;

        if self
            .capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            self.capabilities.current_transform
        }
    }
}

pub fn create_swapchain(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: ash::vk::PhysicalDevice,
    surface_info: &crate::surface::VkSurfaceInfo,
    queue_family: &crate::queue::QueueFamilyIndices,
) -> Result<VkSwapchainInfo, String> {
    use crate::surface::choose_swap;
    use ash::{extensions::khr::Swapchain, vk};

    let swapchain_support =
        VkSwapchainDetail::query_swapchain_support(physical_device, surface_info);

    if swapchain_support.is_err() {
        return Err(swapchain_support.err().unwrap());
    }

    let swapchain_support = swapchain_support.unwrap();
    let surface_format = choose_swap::surface_format(&swapchain_support.formats);
    let present_mode = choose_swap::present_mode(&swapchain_support.present_modes);
    let extent = choose_swap::extent(&swapchain_support.capabilities);

    let image_count = swapchain_support.image_count();
    let pre_transform = swapchain_support.pre_transform();
    let (image_sharing_mode, queue_family_indices) = queue_family.sharing_mode();

    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface_info.surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(image_sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(pre_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true);

    let swapchain_loader = Swapchain::new(instance, device);
    let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None) };

    if swapchain.is_err() {
        return Err(String::from("failed to get swap chain!"));
    }

    let swapchain = swapchain.unwrap();
    let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain) };

    if swapchain_images.is_err() {
        return Err(String::from("failed to get swap chain images!"));
    }

    Ok(VkSwapchainInfo {
        swapchain_loader,
        swapchain,
        swapchain_images: swapchain_images.unwrap(),
        swapchain_format: surface_format.format,
        swapchain_extent: extent,
    })
}
