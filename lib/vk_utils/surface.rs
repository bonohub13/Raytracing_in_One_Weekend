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
