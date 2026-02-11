use crate::core::{
    QueueFamilyIndices,
    error::{RtError, RtResult},
    renderer::Sync,
    surface::{self, Surface},
};
use ash::{khr, vk};
use std::sync::Arc;
use winit::window::Window;

pub struct Swapchain {
    device: khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    image_format: vk::Format,
    extent: vk::Extent2D,
}

impl Swapchain {
    const IMAGE_ARRAY_LAYERS: u32 = 1;

    pub fn new(
        window: Arc<Window>,
        instance: &ash::Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
        device: &ash::Device,
    ) -> RtResult<Self> {
        let swapchain_device = khr::swapchain::Device::new(instance, device);
        let (swapchain, image_format, extent) = Self::create_swapchain(
            window,
            &swapchain_device,
            instance,
            surface,
            physical_device,
        )?;
        let images = Self::get_images(&swapchain_device, &swapchain)?;
        let image_views = Self::get_image_views(device, &images, image_format)?;

        Ok(Self {
            device: swapchain_device,
            swapchain,
            images,
            image_views,
            image_format,
            extent,
        })
    }

    #[allow(unused)]
    #[inline]
    pub fn images(&self) -> &[vk::Image] {
        &self.images
    }

    #[inline]
    pub fn image_views(&self) -> &[vk::ImageView] {
        &self.image_views
    }

    #[inline]
    pub fn image_format(&self) -> &vk::Format {
        &self.image_format
    }

    #[inline]
    pub fn extent(&self) -> &vk::Extent2D {
        &self.extent
    }

    pub fn resize(
        &mut self,
        window: Arc<Window>,
        instance: &ash::Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
        device: &ash::Device,
    ) -> RtResult<()> {
        unsafe { self.destroy(device) };

        (self.swapchain, self.image_format, self.extent) = Self::create_swapchain(
            window.clone(),
            &self.device,
            instance,
            surface,
            physical_device,
        )?;
        self.images = Self::get_images(&self.device, &self.swapchain)?;
        self.image_views = Self::get_image_views(device, &self.images, self.image_format)?;

        Ok(())
    }

    pub fn queue_present(
        &self,
        present_queue: &vk::Queue,
        sync_object: &Sync,
        image_index: u32,
        current_frame: usize,
    ) -> RtResult<bool> {
        let image_index = [image_index];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(
                sync_object.render_finished_semaphore(current_frame),
            ))
            .swapchains(std::slice::from_ref(&self.swapchain))
            .image_indices(&image_index);

        match unsafe { self.device.queue_present(*present_queue, &present_info) } {
            Ok(presented_queue) => Ok(presented_queue),
            Err(err) => Err(RtError::QueuePresent(err.into())),
        }
    }

    pub fn acquire_next_image(
        &self,
        sync_object: &Sync,
        current_frame: usize,
    ) -> RtResult<Option<(usize, bool)>> {
        match unsafe {
            self.device.acquire_next_image(
                self.swapchain,
                u64::MAX,
                *sync_object.image_available_semaphore(current_frame),
                vk::Fence::null(),
            )
        } {
            Ok((image_index, acquired_image)) => Ok(Some((image_index as usize, acquired_image))),
            Err(err) => match err {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => Ok(None),
                _ => Err(RtError::AcquiredNextImage(err.into())),
            },
        }
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        self.image_views
            .iter()
            .for_each(|image_view| unsafe { device.destroy_image_view(*image_view, None) });
        unsafe { self.device.destroy_swapchain(self.swapchain, None) }
    }

    fn create_swapchain(
        window: Arc<Window>,
        device: &khr::swapchain::Device,
        instance: &ash::Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<(vk::SwapchainKHR, vk::Format, vk::Extent2D)> {
        let swapchain_support =
            surface::SwapchainSupportDetails::query_swapchain_support(surface, physical_device)?;
        let surface_format = swapchain_support.choose_swap_surface_format();
        let present_mode = swapchain_support.choose_swap_present_mode();
        let extent = swapchain_support.choose_swap_extent(window);
        let image_count = {
            let image_count = swapchain_support.capabilities().min_image_count + 1;

            if (swapchain_support.capabilities().max_image_count > 0)
                && (image_count > swapchain_support.capabilities().max_image_count)
            {
                swapchain_support.capabilities().max_image_count
            } else {
                image_count
            }
        };
        let indices = QueueFamilyIndices::find_queue_families(instance, surface, physical_device);
        let unique_queue_families = indices.unique_queue_indices()?;
        let create_info = {
            let create_info = vk::SwapchainCreateInfoKHR::default()
                .surface(*surface.surface())
                .min_image_count(image_count)
                .image_format(surface_format.format)
                .image_color_space(surface_format.color_space)
                .image_extent(extent)
                .image_array_layers(Self::IMAGE_ARRAY_LAYERS)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .pre_transform(swapchain_support.capabilities().current_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true);

            if unique_queue_families.len() == 2 {
                create_info
                    .image_sharing_mode(vk::SharingMode::CONCURRENT)
                    .queue_family_indices(&unique_queue_families)
            } else {
                create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            }
        };

        match unsafe { device.create_swapchain(&create_info, None) } {
            Ok(swapchain) => Ok((
                swapchain,
                create_info.image_format,
                create_info.image_extent,
            )),
            Err(err) => Err(RtError::CreateSwapchain(err.into())),
        }
    }

    fn get_images(
        device: &khr::swapchain::Device,
        swapchain: &vk::SwapchainKHR,
    ) -> RtResult<Vec<vk::Image>> {
        match unsafe { device.get_swapchain_images(*swapchain) } {
            Ok(images) => Ok(images),
            Err(err) => Err(RtError::GetSwapchainImages(err.into())),
        }
    }

    fn get_image_views(
        device: &ash::Device,
        images: &[vk::Image],
        format: vk::Format,
    ) -> RtResult<Vec<vk::ImageView>> {
        let mut image_views: Vec<vk::ImageView> = Vec::with_capacity(images.len());
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

        for image in images.iter() {
            let create_info = create_info.image(*image);
            let image_view = match unsafe { device.create_image_view(&create_info, None) } {
                Ok(image_view) => Ok(image_view),
                Err(err) => Err(RtError::CreateSwapchainImageViews(err.into())),
            }?;

            image_views.push(image_view);
        }

        Ok(image_views)
    }
}
