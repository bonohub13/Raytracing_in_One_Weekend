pub fn create_surface(
    app_base: &crate::AppBase,
    window: &crate::window::Window,
    instance: &ash::Instance,
) -> Result<crate::VkSurface, String> {
    use ash::extensions::khr::Surface;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating VkSurface");

    let surface_loader = Surface::new(app_base.entry(), instance);

    let surface_sg = {
        let surface = unsafe {
            ash_window::create_surface(
                app_base.entry(),
                instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None,
            )
            .map_err(|_| String::from("failed to create surface"))?
        };

        guard(surface, |surface| {
            log::info!("surface scopeguard");

            unsafe {
                surface_loader.destroy_surface(surface, None);
            }
        })
    };

    Ok(crate::VkSurface {
        surface: ScopeGuard::into_inner(surface_sg),
        surface_loader,
    })
}
