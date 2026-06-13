// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    AllocatedImage, Allocator, Device, Encoder, ImageType, RtErr, RtError, SwapchainSupportDetails,
    SyncObject, VkState,
};
use ash::{khr::swapchain, vk};
use std::{
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};
use winit::window::Window;

static SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
    aspect_mask: vk::ImageAspectFlags::COLOR,
    base_mip_level: 0,
    level_count: 1,
    base_array_layer: 0,
    layer_count: 1,
};

pub struct Swapchain {
    swapchain_image_views: Vec<vk::ImageView>,
    swapchain_images: Vec<vk::Image>,
    handle: vk::SwapchainKHR,
    loader: swapchain::Device,
    color_image: ManuallyDrop<AllocatedImage>,
    depth_image: ManuallyDrop<AllocatedImage>,
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
    pub(crate) samples_count: vk::SampleCountFlags,
    device: Arc<Device>,
    allocator: Arc<Mutex<Allocator>>,
}

impl Swapchain {
    pub fn new(
        window: Arc<Window>,
        encoder: &Encoder,
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
    ) -> RtErr<Self> {
        let swapchain_support = state
            .surface
            .query_swapchain_support(state.device.physical_device())?;
        let surface_capabilities = swapchain_support.capabilities;
        let surface_format = swapchain_support.choose_swap_surface_format();
        let present_mode = swapchain_support.choose_swap_present_mode();
        let samples_count = Self::max_usable_sample_count(state.device.clone());
        let extent = swapchain_support.choose_swap_extent(window.clone());
        let color_image = ManuallyDrop::new(AllocatedImage::new(
            state,
            allocator.clone(),
            encoder,
            samples_count,
            ImageType::Color(&extent, surface_format.format),
        )?);
        let depth_image = ManuallyDrop::new(AllocatedImage::new(
            state,
            allocator.clone(),
            encoder,
            samples_count,
            ImageType::Depth(&extent),
        )?);
        let loader = swapchain::Device::new(state.instance.instance(), state.device.device());
        let handle = Self::create_swapchain(window, state, &loader, &swapchain_support)?;
        let swapchain_images = Self::create_images(&loader, handle)?;
        let swapchain_image_views = Self::create_image_views(
            state.device.clone(),
            &swapchain_images,
            surface_format.format,
        )?;

        Ok(Self {
            samples_count,
            surface_capabilities,
            surface_format,
            present_mode,
            extent,
            color_image,
            depth_image,
            loader,
            handle,
            swapchain_images,
            swapchain_image_views,
            device: state.device.clone(),
            allocator,
        })
    }

    #[inline]
    pub fn swapchain(&self) -> vk::SwapchainKHR {
        self.handle
    }

    #[inline]
    pub fn extent(&self) -> &vk::Extent2D {
        &self.extent
    }

    #[inline]
    pub fn image_views(&self) -> &[vk::ImageView] {
        &self.swapchain_image_views
    }

    pub fn resize(
        &mut self,
        window: Arc<Window>,
        state: &VkState,
        encoder: &Encoder,
        physical_size: winit::dpi::PhysicalSize<u32>,
    ) -> RtErr<()> {
        if physical_size.width > 0 && physical_size.height > 0 {
            self.recreate_swapchain(window, state, encoder, Some(physical_size))?;
        }

        Ok(())
    }

    pub fn acquire_next_image(
        &mut self,
        window: Arc<Window>,
        state: &VkState,
        encoder: &Encoder,
        sync: &SyncObject,
        current_frame: usize,
    ) -> RtErr<Option<(usize, bool)>> {
        let image_info = vk::AcquireNextImageInfoKHR::default()
            .swapchain(self.handle)
            .timeout(u64::MAX)
            .semaphore(sync.image_available_semaphores()[current_frame])
            .fence(vk::Fence::null())
            .device_mask(1);

        match unsafe { self.loader.acquire_next_image2(&image_info) } {
            Ok((image_index, is_suboptimal)) => Ok(Some((image_index as usize, is_suboptimal))),
            Err(err) => match err {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => self
                    .recreate_swapchain(window, state, encoder, None)
                    .map(|_| None),
                _ => Err(RtError::AcquireNextImage(err.into())),
            },
        }
    }

    pub fn present_queue(
        &self,
        sync_object: &SyncObject,
        image_index: u32,
        current_frame: usize,
    ) -> RtErr<bool> {
        let render_finished_semaphore = sync_object.render_finished_semaphores()[current_frame];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(&render_finished_semaphore))
            .swapchains(std::slice::from_ref(&self.handle))
            .image_indices(std::slice::from_ref(&image_index));

        unsafe {
            self.loader
                .queue_present(self.device.present_queue(), &present_info)
        }
        .map_err(|err| RtError::QueuePresent(err.into()))
    }

    pub fn transition_surface_to_color_attachment(
        &self,
        command_buffer: vk::CommandBuffer,
        image_index: usize,
    ) {
        let barrier = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::NONE)
            .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .image(self.swapchain_images[image_index])
            .subresource_range(SUBRESOURCE_RANGE);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info)
        }
    }

    pub fn transition_surface_to_present(
        &self,
        command_buffer: vk::CommandBuffer,
        image_index: usize,
    ) {
        let barrier = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)
            .dst_access_mask(vk::AccessFlags2::NONE)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .image(self.swapchain_images[image_index])
            .subresource_range(SUBRESOURCE_RANGE);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info)
        }
    }

    pub fn recreate_swapchain(
        &mut self,
        window: Arc<Window>,
        state: &VkState,
        encoder: &Encoder,
        physical_size: Option<winit::dpi::PhysicalSize<u32>>,
    ) -> RtErr<()> {
        static mut MAX_SIZE: vk::Extent2D = vk::Extent2D {
            width: 0,
            height: 0,
        };

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
        let swapchain_support = state
            .surface
            .query_swapchain_support(self.device.physical_device())?;
        let old_extent = self.extent;
        self.surface_capabilities = swapchain_support.capabilities;
        self.surface_format = swapchain_support.choose_swap_surface_format();
        self.present_mode = swapchain_support.choose_swap_present_mode();
        self.extent = if let Some(physical_size) = physical_size {
            let min_extent = self.surface_capabilities.min_image_extent;
            let max_extent = self.surface_capabilities.max_image_extent;

            vk::Extent2D {
                width: if min_extent.width < physical_size.width
                    && physical_size.width < max_extent.width
                {
                    physical_size.width
                } else {
                    min_extent
                        .width
                        .max(physical_size.width.min(max_extent.width))
                },
                height: if min_extent.height < physical_size.height
                    && physical_size.height < max_extent.height
                {
                    physical_size.height
                } else {
                    min_extent
                        .height
                        .max(physical_size.height.min(max_extent.height))
                },
            }
        } else {
            swapchain_support.choose_swap_extent(window.clone())
        };
        if unsafe { MAX_SIZE.width * MAX_SIZE.height } == 0 {
            unsafe {
                MAX_SIZE.width = old_extent.width;
                MAX_SIZE.height = old_extent.height;
            }
        }
        let queue_family_indices = state
            .surface
            .find_queue_families(&state.instance, self.device.physical_device())?
            .unique_queue_families()?;
        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(state.surface.surface())
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

        if (self.extent.width * self.extent.height) > unsafe { MAX_SIZE.width * MAX_SIZE.height } {
            unsafe {
                MAX_SIZE.width = self.extent.width;
                MAX_SIZE.height = self.extent.height;
                ManuallyDrop::drop(&mut self.color_image);
                ManuallyDrop::drop(&mut self.depth_image);
            }
            self.color_image = ManuallyDrop::new(AllocatedImage::new(
                state,
                self.allocator.clone(),
                encoder,
                self.samples_count,
                ImageType::Color(&self.extent, self.surface_format.format),
            )?);
            self.depth_image = ManuallyDrop::new(AllocatedImage::new(
                state,
                self.allocator.clone(),
                encoder,
                self.samples_count,
                ImageType::Depth(&self.extent),
            )?);
        }
        self.handle = handle;
        self.swapchain_images = Self::create_images(&self.loader, self.handle)?;
        self.swapchain_image_views = Self::create_image_views(
            self.device.clone(),
            &self.swapchain_images,
            self.surface_format.format,
        )?;

        Ok(())
    }

    pub(crate) fn color_attachment<'attachment>(
        &self,
        image_index: usize,
    ) -> vk::RenderingAttachmentInfo<'attachment> {
        static CLEAR_VALUE: vk::ClearValue = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0f32, 0f32, 0f32, 1f32],
            },
        };

        vk::RenderingAttachmentInfo::default()
            .image_view(self.color_image.image_view())
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .resolve_mode(vk::ResolveModeFlags::AVERAGE)
            .resolve_image_view(self.swapchain_image_views[image_index])
            .resolve_image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .clear_value(CLEAR_VALUE)
    }

    pub(crate) fn depth_attachment<'attachment>(&self) -> vk::RenderingAttachmentInfo<'attachment> {
        static DEPTH_CLEAR_VALUE: vk::ClearValue = vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue {
                depth: 0f32,
                stencil: 0,
            },
        };

        vk::RenderingAttachmentInfo::default()
            .image_view(self.depth_image.image_view())
            .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(DEPTH_CLEAR_VALUE)
    }

    unsafe fn destroy(&mut self) {
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
        state: &VkState,
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
        let queue_family_indices = state
            .surface
            .find_queue_families(&state.instance, state.device.physical_device())?
            .unique_queue_families()?;
        let create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(state.surface.surface())
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

    fn max_usable_sample_count(device: Arc<Device>) -> vk::SampleCountFlags {
        static MAX_SAMPLE_COUNTS: [vk::SampleCountFlags; 6] = [
            vk::SampleCountFlags::TYPE_64,
            vk::SampleCountFlags::TYPE_32,
            vk::SampleCountFlags::TYPE_16,
            vk::SampleCountFlags::TYPE_8,
            vk::SampleCountFlags::TYPE_4,
            vk::SampleCountFlags::TYPE_2,
        ];

        MAX_SAMPLE_COUNTS
            .iter()
            .copied()
            .find(|sample| {
                device
                    .properties()
                    .limits
                    .framebuffer_color_sample_counts
                    .contains(*sample)
            })
            .unwrap_or(vk::SampleCountFlags::TYPE_1)
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.destroy();
            ManuallyDrop::drop(&mut self.color_image);
            ManuallyDrop::drop(&mut self.depth_image);
        }
    }
}
