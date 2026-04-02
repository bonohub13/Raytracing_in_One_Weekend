use crate::{Device, RtErr, RtError, Surface};
use ash::vk;
use std::sync::Arc;
use winit::window::Window;

pub struct Swapchain {
    device: Arc<Device>,
    swapchain: vk::SwapchainKHR,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
}

impl Swapchain {
    pub(crate) fn new(window: Arc<Window>, surface: &Surface, device: Arc<Device>) -> RtErr<Self> {
        let swapchain_support =
            surface.query_swapchain_support(device.clone().physical_device())?;
        let surface_capabilities = swapchain_support.capabilities;
        let surface_format = swapchain_support.choose_swap_surface_format();
        let present_mode = swapchain_support.choose_swap_present_mode();
        let extent = swapchain_support.choose_swap_extent(window);
        let swapchain = Self::create_swapchain(
            surface,
            device.clone(),
            &surface_capabilities,
            &surface_format,
            present_mode,
            extent,
        )?;

        Ok(Self {
            device,
            swapchain,
            surface_capabilities,
            surface_format,
            present_mode,
            extent,
        })
    }

    #[inline]
    pub(crate) fn swapchain(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    pub(crate) fn recreate_swapchain(&mut self, surface: &Surface) -> RtErr<Self> {
        let image_count = {
            let image_count = self.surface_capabilities.min_image_count + 1;

            if self.surface_capabilities.max_image_count > 0
                && image_count > self.surface_capabilities.max_image_count
            {
                self.surface_capabilities.max_image_count
            } else {
                image_count
            }
        };
        let queue_family_indices = surface
            .find_queue_families(self.device.physical_device())?
            .unique_queue_families()?;
        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.surface())
            .min_image_count(image_count)
            .image_format(self.surface_format.format)
            .image_color_space(self.surface_format.color_space)
            .image_extent(self.extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(if queue_family_indices.len() == 1 {
                vk::SharingMode::EXCLUSIVE
            } else {
                vk::SharingMode::CONCURRENT
            })
            .queue_family_indices(&queue_family_indices)
            .pre_transform(self.surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(self.present_mode)
            .old_swapchain(self.swapchain);
        let swapchain = unsafe {
            self.device
                .swapchain_loader()
                .create_swapchain(&create_info, None)
        }
        .map_err(|err| RtError::CreateSwapchain(err.into()))?;

        Ok(Self {
            device: self.device.clone(),
            swapchain,
            surface_capabilities: self.surface_capabilities,
            surface_format: self.surface_format,
            present_mode: self.present_mode,
            extent: self.extent,
        })
    }

    fn create_swapchain(
        surface: &Surface,
        device: Arc<Device>,
        surface_capabilities: &vk::SurfaceCapabilitiesKHR,
        surface_format: &vk::SurfaceFormatKHR,
        present_mode: vk::PresentModeKHR,
        extent: vk::Extent2D,
    ) -> RtErr<vk::SwapchainKHR> {
        let image_count = {
            let image_count = surface_capabilities.min_image_count + 1;

            if surface_capabilities.max_image_count > 0
                && image_count > surface_capabilities.max_image_count
            {
                surface_capabilities.max_image_count
            } else {
                image_count
            }
        };
        let queue_family_indices = surface
            .find_queue_families(device.physical_device())?
            .unique_queue_families()?;
        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.surface())
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(if queue_family_indices.len() == 1 {
                vk::SharingMode::EXCLUSIVE
            } else {
                vk::SharingMode::CONCURRENT
            })
            .queue_family_indices(&queue_family_indices)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .old_swapchain(vk::SwapchainKHR::null());

        unsafe {
            device
                .swapchain_loader()
                .create_swapchain(&create_info, None)
        }
        .map_err(|err| RtError::CreateSwapchain(err.into()))
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.device
                .swapchain_loader()
                .destroy_swapchain(self.swapchain, None);
        }
    }
}
