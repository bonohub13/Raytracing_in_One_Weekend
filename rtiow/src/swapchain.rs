use crate::{Device, Instance, RtErr, RtError, Surface, SwapchainSupportDetails, VkState};
use ash::{khr::swapchain, vk};
use std::sync::Arc;
use winit::window::Window;

pub struct Swapchain {
    swapchain_image_views: Vec<vk::ImageView>,
    swapchain_images: Vec<vk::Image>,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
    handle: vk::SwapchainKHR,
    loader: swapchain::Device,
    device: Arc<Device>,
}

impl Swapchain {
    pub(crate) fn new(
        window: Arc<Window>,
        instance: &Instance,
        surface: &Surface,
        device: Arc<Device>,
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
            device.clone(),
            &loader,
            &swapchain_support,
        )?;
        let swapchain_images = Self::create_images(&loader, handle)?;
        let swapchain_image_views =
            Self::create_image_views(device.clone(), &swapchain_images, surface_format.format)?;

        Ok(Self {
            device,
            loader,
            handle,
            surface_capabilities,
            surface_format,
            present_mode,
            extent,
            swapchain_images,
            swapchain_image_views,
        })
    }

    #[inline]
    pub(crate) fn swapchain(&self) -> vk::SwapchainKHR {
        self.handle
    }

    #[inline]
    pub(crate) fn extent(&self) -> &vk::Extent2D {
        &self.extent
    }

    #[inline]
    pub(crate) fn image_views(&self) -> &[vk::ImageView] {
        &self.swapchain_image_views
    }

    pub(crate) fn acquire_next_image(
        &mut self,
        window: Arc<Window>,
        instance: &Instance,
        surface: &Surface,
        current_frame: usize,
    ) -> RtErr<Option<(usize, bool)>> {
        let image_info = vk::AcquireNextImageInfoKHR::default()
            .swapchain(self.handle)
            .timeout(u64::MAX)
            .semaphore(todo!())
            .fence(vk::Fence::null())
            .device_mask(1);

        match unsafe { self.loader.acquire_next_image2(&image_info) } {
            Ok((image_index, is_suboptimal)) => Ok(Some((image_index as usize, is_suboptimal))),
            Err(err) => match err {
                vk::Result::ERROR_OUT_OF_DATE_KHR => self
                    .recreate_swapchain(window, instance, surface)
                    .map(|_| None),
                _ => Err(RtError::AcquireNextImage(err.into())),
            },
        }
    }

    pub(crate) fn recreate_swapchain(
        &mut self,
        window: Arc<Window>,
        instance: &Instance,
        surface: &Surface,
    ) -> RtErr<()> {
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
        let swapchain_support = surface.query_swapchain_support(self.device.physical_device())?;
        self.surface_capabilities = swapchain_support.capabilities;
        self.surface_format = swapchain_support.choose_swap_surface_format();
        self.present_mode = swapchain_support.choose_swap_present_mode();
        self.extent = swapchain_support.choose_swap_extent(window.clone());
        let queue_family_indices = surface
            .find_queue_families(instance, self.device.physical_device())?
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

        unsafe { self.destroy() };

        self.handle = handle;
        self.swapchain_images = Self::create_images(&self.loader, self.handle)?;
        self.swapchain_image_views = Self::create_image_views(
            self.device.clone(),
            &self.swapchain_images,
            self.surface_format.format,
        )?;

        Ok(())
    }

    unsafe fn destroy(&self) {
        self.swapchain_image_views
            .iter()
            .for_each(|image_view| unsafe {
                self.device.device().destroy_image_view(*image_view, None);
            });
        unsafe {
            self.loader.destroy_swapchain(self.handle, None);
        }
    }

    fn create_swapchain(
        window: Arc<Window>,
        instance: &Instance,
        surface: &Surface,
        device: Arc<Device>,
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

    fn create_images(
        loader: &swapchain::Device,
        swapchain: vk::SwapchainKHR,
    ) -> RtErr<Vec<vk::Image>> {
        unsafe { loader.get_swapchain_images(swapchain) }
            .map_err(|err| RtError::CreateImages(err.into()))
    }

    fn create_image_views(
        device: Arc<Device>,
        images: &[vk::Image],
        format: vk::Format,
    ) -> RtErr<Vec<vk::ImageView>> {
        static COMPONENTS: vk::ComponentMapping = vk::ComponentMapping {
            r: vk::ComponentSwizzle::IDENTITY,
            g: vk::ComponentSwizzle::IDENTITY,
            b: vk::ComponentSwizzle::IDENTITY,
            a: vk::ComponentSwizzle::IDENTITY,
        };
        static SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        images
            .iter()
            .copied()
            .map(|image| {
                let create_info = vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(format)
                    .components(COMPONENTS)
                    .subresource_range(SUBRESOURCE_RANGE);

                unsafe { device.device().create_image_view(&create_info, None) }
                    .map_err(|err| RtError::CreateImageView(err.into()))
            })
            .collect()
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe { self.destroy() }
    }
}
