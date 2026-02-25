use crate::core::{RtError, RtResult, device::Device, instance::Instance, surface::Surface};
use ash::{khr::swapchain, vk};
use std::sync::Arc;

pub struct SwapchainDescriptor {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
}

pub struct Swapchain {
    surface: Arc<Surface>,
    device: Arc<Device>,
    loader: swapchain::Device,
    raw: vk::SwapchainKHR,
    image_format: vk::Format,
    extent: vk::Extent2D,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
}

impl Swapchain {
    pub fn new(desc: &SwapchainDescriptor) -> RtResult<Self> {
        let loader = swapchain::Device::new(desc.instance.raw(), desc.device.raw());
        let (swapchain, image_format, extent) =
            Self::create_swapchain(&loader, desc.device.clone(), desc.surface.clone())?;
        let images = Self::create_swapchain_images(&loader, swapchain)?;
        let image_views =
            Self::create_swapchain_image_views(desc.device.clone(), image_format, &images)?;

        Ok(Self {
            surface: desc.surface.clone(),
            device: desc.device.clone(),
            loader,
            raw: swapchain,
            image_format,
            extent,
            images,
            image_views,
        })
    }

    #[inline]
    pub const fn loader(&self) -> &swapchain::Device {
        &self.loader
    }

    #[inline]
    pub const fn raw(&self) -> vk::SwapchainKHR {
        self.raw
    }

    #[inline]
    pub const fn image_format(&self) -> vk::Format {
        self.image_format
    }

    #[inline]
    pub const fn extent(&self) -> vk::Extent2D {
        self.extent
    }

    #[inline]
    pub const fn images(&self) -> &[vk::Image] {
        self.images.as_slice()
    }

    #[inline]
    pub const fn image_view(&self) -> &[vk::ImageView] {
        self.image_views.as_slice()
    }

    pub fn recreate_swapchain(&self) -> RtResult<Self> {
        self.device.device_wait_idle()?;

        let (raw, image_format, extent) =
            Self::create_swapchain(&self.loader, self.device.clone(), self.surface.clone())?;
        let images = Self::create_swapchain_images(&self.loader, raw)?;
        let image_views =
            Self::create_swapchain_image_views(self.device.clone(), image_format, &images)?;

        Ok(Self {
            surface: self.surface.clone(),
            device: self.device.clone(),
            loader: self.loader.clone(),
            raw,
            image_format,
            extent,
            images,
            image_views,
        })
    }

    fn create_swapchain(
        loader: &swapchain::Device,
        device: Arc<Device>,
        surface: Arc<Surface>,
    ) -> RtResult<(vk::SwapchainKHR, vk::Format, vk::Extent2D)> {
        let swapchain_support = device.query_swapchain_support(surface.clone())?;
        let surface_format = swapchain_support.choose_swap_surface_format();
        let present_mode = swapchain_support.choose_swap_present_mode();
        let extent = swapchain_support.choose_swap_extent(surface.inner_size());
        let image_count = if (swapchain_support.capabilities.max_image_count > 0)
            && ((swapchain_support.capabilities.min_image_count + 1)
                > swapchain_support.capabilities.max_image_count)
        {
            swapchain_support.capabilities.max_image_count
        } else {
            swapchain_support.capabilities.min_image_count + 1
        };
        let indices = device.queue_families();
        let queue_family_indices = [indices.graphics_family, indices.present_family];
        let create_info = {
            let create_info = vk::SwapchainCreateInfoKHR::default()
                .surface(surface.raw())
                .min_image_count(image_count)
                .image_format(surface_format.format)
                .image_color_space(surface_format.color_space)
                .image_extent(extent)
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .pre_transform(swapchain_support.capabilities.current_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true)
                .old_swapchain(vk::SwapchainKHR::null());

            if queue_family_indices[0] != queue_family_indices[1] {
                create_info
                    .image_sharing_mode(vk::SharingMode::CONCURRENT)
                    .queue_family_indices(&queue_family_indices)
            } else {
                create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            }
        };

        match unsafe { loader.create_swapchain(&create_info, None) } {
            Ok(swapchain) => Ok((swapchain, surface_format.format, extent)),
            Err(err) => Err(RtError::CreateSwapchain(err.into())),
        }
    }

    fn create_swapchain_images(
        loader: &swapchain::Device,
        swapchain: vk::SwapchainKHR,
    ) -> RtResult<Vec<vk::Image>> {
        match unsafe { loader.get_swapchain_images(swapchain) } {
            Ok(images) => Ok(images),
            Err(err) => Err(RtError::GetSwapchainImages(err.into())),
        }
    }

    fn create_swapchain_image_views(
        device: Arc<Device>,
        format: vk::Format,
        images: &[vk::Image],
    ) -> RtResult<Vec<vk::ImageView>> {
        let create_info = vk::ImageViewCreateInfo::default()
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            })
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        images
            .iter()
            .map(|image| {
                let create_info = create_info.image(*image);

                match unsafe { device.raw().create_image_view(&create_info, None) } {
                    Ok(image_view) => Ok(image_view),
                    Err(err) => Err(RtError::CreateImageView(err.into())),
                }
            })
            .collect()
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.image_views.iter().for_each(|image_view| unsafe {
            self.device.raw().destroy_image_view(*image_view, None)
        });
        unsafe { self.loader.destroy_swapchain(self.raw, None) };
    }
}
