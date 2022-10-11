pub struct VkSwapchainDetail {
    pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<ash::vk::SurfaceFormatKHR>,
    pub present_modes: Vec<ash::vk::PresentModeKHR>,
}

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
