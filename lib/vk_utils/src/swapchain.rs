pub struct SupportDetails {
    pub capabilities: ash::vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<ash::vk::SurfaceFormatKHR>,
    pub present_modes: Vec<ash::vk::PresentModeKHR>,
}

pub struct SwapChain {
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: ash::vk::SwapchainKHR,
    pub support_details: SupportDetails,
    min_image_count: u32,
    present_mode: ash::vk::PresentModeKHR,
    format: ash::vk::Format,
    extent: ash::vk::Extent2D,
    pub images: Vec<ash::vk::Image>,
    pub image_views: Vec<crate::ImageView>,
    physical_device: ash::vk::PhysicalDevice,
}

impl SwapChain {
    pub fn new(
        window: &crate::window::Window,
        instance: &crate::Instance,
        device: &crate::Device,
        surface: &crate::Surface,
        present_mode: ash::vk::PresentModeKHR,
    ) -> Result<Self, String> {
        use ash::{extensions::khr::Swapchain, vk};
        use scopeguard::{guard, ScopeGuard};

        let details = match Self::query_swap_chain_support(device.physical_device(), surface) {
            Err(crate::surface::SurfaceError::PhysicalDeviceSurfaceFormatsError)
            | Err(crate::surface::SurfaceError::PhysicalDeviceSurfacePresentModesError) => {
                Err(String::from("empty swap chain support"))
            }
            Err(err) => Err(format!("{}", err)),
            Ok(details) => Ok(details),
        }?;

        let surface_format = Self::choose_swap_surface_format(&details.formats)?;
        let actual_present_mode =
            Self::choose_swap_surface_present_mode(&details.present_modes, present_mode)?;
        let extent = Self::choose_swap_extent(window, &details.capabilities);
        let image_count = Self::choose_image_count(&details.capabilities);
        let graphics_queue_index = device.graphics_queue_index();
        let present_queue_index = device.present_queue_index();
        let queue_family_indices = if graphics_queue_index != present_queue_index {
            vec![graphics_queue_index, present_queue_index]
        } else {
            vec![]
        };

        let swapchain_loader = Swapchain::new(&instance.instance, &device.device);

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_DST)
            .pre_transform(details.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(actual_present_mode)
            .clipped(true)
            .image_sharing_mode(if graphics_queue_index != present_queue_index {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(&queue_family_indices)
            .build();

        let swapchain_sg = {
            let swapchain = unsafe {
                swapchain_loader
                    .create_swapchain(&create_info, None)
                    .map_err(|_| String::from("failed to create swap chain"))?
            };

            guard(swapchain, |swapchain| {
                log::warn!("swap chain scopeguard");

                unsafe {
                    swapchain_loader.destroy_swapchain(swapchain, None);
                }
            })
        };

        let images = unsafe {
            swapchain_loader
                .get_swapchain_images(*swapchain_sg)
                .map_err(|_| String::from("failed to get swapchin images"))?
        };

        let mut image_views = Vec::new();
        for &image in images.iter() {
            let image_view = crate::ImageView::new(
                device,
                image,
                surface_format.format,
                vk::ImageAspectFlags::COLOR,
            )?;

            image_views.push(image_view);
        }

        Ok(Self {
            swapchain: ScopeGuard::into_inner(swapchain_sg),
            swapchain_loader,
            format: surface_format.format,
            min_image_count: details.capabilities.min_image_count.max(2),
            support_details: details,
            present_mode,
            extent,
            images,
            image_views,
            physical_device: device.physical_device(),
        })
    }

    pub fn physical_device(&self) -> ash::vk::PhysicalDevice {
        self.physical_device
    }

    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }

    pub fn images(&self) -> Vec<ash::vk::Image> {
        self.images.clone()
    }

    pub fn extent(&self) -> ash::vk::Extent2D {
        self.extent
    }

    pub fn format(&self) -> ash::vk::Format {
        self.format
    }

    pub fn present_mode(&self) -> ash::vk::PresentModeKHR {
        self.present_mode
    }

    fn query_swap_chain_support(
        physical_device: ash::vk::PhysicalDevice,
        surface: &crate::Surface,
    ) -> Result<SupportDetails, crate::surface::SurfaceError> {
        let capabilities = surface.get_physical_device_surface_capabilities(physical_device)?;
        let formats = surface.get_physical_device_surface_formats(physical_device)?;
        let present_modes = surface.get_physical_device_surface_present_modes(physical_device)?;

        Ok(SupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }

    fn choose_swap_surface_format(
        formats: &Vec<ash::vk::SurfaceFormatKHR>,
    ) -> Result<ash::vk::SurfaceFormatKHR, String> {
        use ash::vk;

        if formats.len() == 1 && formats[0].format == vk::Format::UNDEFINED {
            let surface_format = vk::SurfaceFormatKHR::builder()
                .format(vk::Format::R8G8B8_UNORM)
                .color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
                .build();

            Ok(surface_format)
        } else {
            match formats.iter().find(|&format| {
                format.format == vk::Format::R8G8B8_UNORM
                    && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            }) {
                Some(format) => Ok(format.clone()),
                None => Err(String::from("found no suitable surface format")),
            }
        }
    }

    fn choose_swap_surface_present_mode(
        present_modes: &Vec<ash::vk::PresentModeKHR>,
        present_mode: ash::vk::PresentModeKHR,
    ) -> Result<ash::vk::PresentModeKHR, String> {
        use ash::vk;

        match present_mode {
            vk::PresentModeKHR::IMMEDIATE
            | vk::PresentModeKHR::MAILBOX
            | vk::PresentModeKHR::FIFO
            | vk::PresentModeKHR::FIFO_RELAXED => {
                if present_modes.contains(&present_mode) {
                    Ok(present_mode)
                } else {
                    Ok(vk::PresentModeKHR::FIFO)
                }
            }
            _ => Err(String::from("Unknown present mode")),
        }
    }

    fn choose_swap_extent(
        window: &crate::window::Window,
        capabilities: &ash::vk::SurfaceCapabilitiesKHR,
    ) -> ash::vk::Extent2D {
        if capabilities.current_extent.width != std::u32::MAX {
            capabilities.current_extent
        } else {
            let mut actual_extent = window.framebuffer_size();

            actual_extent.width = capabilities
                .min_image_extent
                .width
                .max(capabilities.max_image_extent.width.min(actual_extent.width));
            actual_extent.height = capabilities.min_image_extent.height.max(
                capabilities
                    .max_image_extent
                    .height
                    .min(actual_extent.height),
            );

            actual_extent
        }
    }

    fn choose_image_count(capabilities: &ash::vk::SurfaceCapabilitiesKHR) -> u32 {
        let image_count = capabilities.min_image_count.max(2);

        if capabilities.max_image_count > 0 && image_count > capabilities.max_image_count {
            capabilities.max_image_count
        } else {
            image_count
        }
    }

    pub fn cleanup(device: &crate::Device, swapchain: &mut Self) {
        log::info!("performing cleanup for SwapChain");

        for iv in swapchain.image_views.iter_mut() {
            crate::ImageView::cleanup(device, iv);
        }

        unsafe {
            swapchain
                .swapchain_loader
                .destroy_swapchain(swapchain.swapchain, None);
        }
    }
}
