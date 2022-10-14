pub struct VkSurfaceInfo {
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: ash::vk::SurfaceKHR,
    pub screen_width: u32,
    pub screen_height: u32,
}

pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::window::Window,
) -> Result<VkSurfaceInfo, String> {
    use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
    use ash::extensions::khr::Surface;
    use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

    let surface = unsafe {
        ash_window::create_surface(
            entry,
            instance,
            window.raw_display_handle(),
            window.raw_window_handle(),
            None,
        )
    };

    if surface.is_err() {
        return Err(String::from("failed to create window surface!"));
    }

    let surface_loader = Surface::new(entry, instance);

    Ok(VkSurfaceInfo {
        surface_loader,
        surface: surface.unwrap(),
        screen_width: WINDOW_WIDTH,
        screen_height: WINDOW_HEIGHT,
    })
}

pub mod choose_swap {
    pub fn surface_format(
        available_formats: &Vec<ash::vk::SurfaceFormatKHR>,
    ) -> ash::vk::SurfaceFormatKHR {
        use ash::vk;

        for available_format in available_formats.iter() {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *available_format;
            }
        }

        *available_formats.first().unwrap()
    }

    pub fn present_mode(
        available_present_modes: &Vec<ash::vk::PresentModeKHR>,
    ) -> ash::vk::PresentModeKHR {
        use ash::vk;

        available_present_modes
            .iter()
            .cloned()
            .find(|&available_present_mode| available_present_mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn extent(capabilities: &ash::vk::SurfaceCapabilitiesKHR) -> ash::vk::Extent2D {
        use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
        use ash::vk;
        use num::clamp;

        match capabilities.current_extent.width {
            std::u32::MAX => vk::Extent2D {
                width: clamp(
                    WINDOW_WIDTH,
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: clamp(
                    WINDOW_HEIGHT,
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            },
            _ => capabilities.current_extent,
        }
    }
}
