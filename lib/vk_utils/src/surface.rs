pub struct Surface {
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: ash::vk::SurfaceKHR,
}

impl Surface {
    pub fn new(
        appbase: &crate::AppBase,
        instance: &crate::Instance,
        window: &crate::window::Window,
    ) -> Result<Surface, String> {
        use ash::extensions::khr;
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating Surface");
        log::info!("creating surface loader");

        let surface_loader = khr::Surface::new(&appbase.entry, &instance.instance);

        log::info!("created surface loader");
        log::info!("creating surface");

        let surface_sg = {
            let surface = unsafe {
                ash_window::create_surface(
                    &appbase.entry,
                    &instance.instance,
                    window.raw_display_handle(),
                    window.raw_window_handle(),
                    None,
                )
                .map_err(|_| String::from("failed to create surface"))?
            };

            guard(surface, |surface| {
                log::warn!("surface scopeguard");

                unsafe {
                    surface_loader.destroy_surface(surface, None);
                }
            })
        };

        log::info!("created surface");

        Ok(Self {
            surface: ScopeGuard::into_inner(surface_sg),
            surface_loader,
        })
    }

    pub fn cleanup(surface: &mut Self) {
        log::info!("performing cleanup for Surface");

        unsafe {
            surface
                .surface_loader
                .destroy_surface(surface.surface, None);
        }
    }
}
