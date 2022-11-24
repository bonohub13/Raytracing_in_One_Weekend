pub struct Surface {
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: ash::vk::SurfaceKHR,
}

impl Surface {
    pub fn new(
        appbase: &crate::AppBase,
        instance: &crate::Instance,
        window: &crate::window::Window,
    ) -> Result<Surface, SurfaceError> {
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
                .map_err(|_| SurfaceError::SurfaceCreateError)?
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

    pub fn get_physical_device_surface_support(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        index: u32,
    ) -> Result<bool, SurfaceError> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_support(physical_device, index, self.surface)
                .map_err(|_| SurfaceError::PhysicalDeviceSurfaceSupportError)
        }
    }

    pub fn get_physical_device_surface_capabilities(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<ash::vk::SurfaceCapabilitiesKHR, SurfaceError> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(physical_device, self.surface)
                .map_err(|_| SurfaceError::PhysicalDeviceSurfaceCapabilitiesError)
        }
    }

    pub fn get_physical_device_surface_formats(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Vec<ash::vk::SurfaceFormatKHR>, SurfaceError> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(physical_device, self.surface)
                .map_err(|_| SurfaceError::PhysicalDeviceSurfaceFormatsError)
        }
    }

    pub fn get_physical_device_surface_present_modes(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Vec<ash::vk::PresentModeKHR>, SurfaceError> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(physical_device, self.surface)
                .map_err(|_| SurfaceError::PhysicalDeviceSurfacePresentModesError)
        }
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

#[derive(Debug)]
pub enum SurfaceError {
    SurfaceCreateError,
    PhysicalDeviceSurfaceSupportError,
    PhysicalDeviceSurfaceCapabilitiesError,
    PhysicalDeviceSurfaceFormatsError,
    PhysicalDeviceSurfacePresentModesError,
}

impl std::fmt::Display for SurfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SurfaceCreateError => write!(f, "failed to create surface"),
            Self::PhysicalDeviceSurfaceSupportError => {
                write!(f, "failed to get surface support for physical device")
            }
            Self::PhysicalDeviceSurfaceCapabilitiesError => {
                write!(f, "failed to get surface capabilities for physical device")
            }
            Self::PhysicalDeviceSurfaceFormatsError => {
                write!(f, "failed to get surface formats for physical device")
            }
            Self::PhysicalDeviceSurfacePresentModesError => {
                write!(f, "failed to get surface present modes for physical device")
            }
        }
    }
}
