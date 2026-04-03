use crate::{Device, Instance, RtErr, RtError, Surface, SwapchainSupportDetails};
use ash::{khr::swapchain, vk};
use std::sync::Arc;
use winit::window::Window;

pub struct Swapchain {
    loader: swapchain::Device,
    handle: vk::SwapchainKHR,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
}

impl Swapchain {
    pub(crate) fn new(
        window: Arc<Window>,
        instance: &Instance,
        surface: &Surface,
        device: &Device,
    ) -> RtErr<Self> {
        let swapchain_support = surface.query_swapchain_support(device.physical_device())?;
        let surface_capabilities = swapchain_support.capabilities;
        let surface_format = swapchain_support.choose_swap_surface_format();
        let present_mode = swapchain_support.choose_swap_present_mode();
        let extent = swapchain_support.choose_swap_extent(window.clone());
        let loader = swapchain::Device::new(instance.instance(), device.device());
        let handle = Self::create_swapchain(
            window,
            instance,
            surface,
            device,
            &loader,
            &swapchain_support,
        )?;

        Ok(Self {
            loader,
            handle,
            surface_capabilities,
            surface_format,
            present_mode,
            extent,
        })
    }

    #[inline]
    pub(crate) fn swapchain(&self) -> vk::SwapchainKHR {
        self.handle
    }

    pub(crate) unsafe fn destroy(&self) {
        unsafe {
            self.loader.destroy_swapchain(self.handle, None);
        }
    }

    pub(crate) fn recreate_swapchain(
        &mut self,
        instance: &Instance,
        surface: &Surface,
        device: &Device,
    ) -> RtErr<Self> {
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
            .find_queue_families(instance, device.physical_device())?
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
            .old_swapchain(self.handle);
        let handle = unsafe { self.loader.create_swapchain(&create_info, None) }
            .map_err(|err| RtError::CreateSwapchain(err.into()))?;

        Ok(Self {
            loader: self.loader.clone(),
            handle,
            surface_capabilities: self.surface_capabilities,
            surface_format: self.surface_format,
            present_mode: self.present_mode,
            extent: self.extent,
        })
    }

    fn create_swapchain(
        window: Arc<Window>,
        instance: &Instance,
        surface: &Surface,
        device: &Device,
        loader: &swapchain::Device,
        swapchain_support: &SwapchainSupportDetails,
    ) -> RtErr<vk::SwapchainKHR> {
        let surface_capabilities = swapchain_support.capabilities;
        let surface_format = swapchain_support.choose_swap_surface_format();
        let present_mode = swapchain_support.choose_swap_present_mode();
        let extent = swapchain_support.choose_swap_extent(window);
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
            .find_queue_families(instance, device.physical_device())?
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

        unsafe { loader.create_swapchain(&create_info, None) }
            .map_err(|err| RtError::CreateSwapchain(err.into()))
    }
}
