// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Instance, QueueFamilyIndices, RtErr, RtError};
use ash::vk::{self, SurfaceKHR};
use std::sync::Arc;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct Surface {
    instance: Arc<Instance>,
    surface: SurfaceKHR,
}

#[derive(Debug, Clone)]
pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl Surface {
    pub(crate) fn new(window: Arc<Window>, instance: Arc<Instance>) -> RtErr<Self> {
        let surface = Self::create_surface(window, instance.clone())?;

        Ok(Self { surface, instance })
    }

    #[inline]
    pub(crate) fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }

    pub(crate) fn find_queue_families(
        &self,
        device: vk::PhysicalDevice,
    ) -> RtErr<QueueFamilyIndices> {
        let instance = self.instance.instance();
        let mut indices = QueueFamilyIndices::default();
        let queue_families = {
            let queue_family_len =
                unsafe { instance.get_physical_device_queue_family_properties2_len(device) };
            let mut queue_families = vec![vk::QueueFamilyProperties2::default(); queue_family_len];

            unsafe {
                instance.get_physical_device_queue_family_properties2(device, &mut queue_families)
            };

            queue_families
        };

        for (i, queue_family) in queue_families.iter().enumerate() {
            let current_family_index = i as u32;
            if queue_family
                .queue_family_properties
                .queue_flags
                .contains(vk::QueueFlags::GRAPHICS)
            {
                indices.graphics_family = Some(current_family_index)
            }

            if self.get_physical_device_surface_support(device, current_family_index)? {
                indices.present_family = Some(current_family_index)
            }

            if indices.is_complete() {
                break;
            }
        }

        Ok(indices)
    }

    pub(crate) fn query_swapchain_support(
        &self,
        device: vk::PhysicalDevice,
    ) -> RtErr<SwapchainSupportDetails> {
        let capabilities = self.get_physical_device_surface_capabilities(device)?;
        let formats = self.get_physical_device_surface_formats(device)?;
        let present_modes = self.get_physical_device_surface_present_modes(device)?;

        Ok(SwapchainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }

    pub(crate) fn get_physical_device_surface_support(
        &self,
        device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> RtErr<bool> {
        unsafe {
            self.instance
                .surface_loader()
                .get_physical_device_surface_support(device, queue_family_index, self.surface)
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfaceSupport(err.into()))
    }

    fn get_physical_device_surface_capabilities(
        &self,
        device: vk::PhysicalDevice,
    ) -> RtErr<vk::SurfaceCapabilitiesKHR> {
        let surface_info = vk::PhysicalDeviceSurfaceInfo2KHR::default().surface(self.surface);
        let mut capabilites = vk::SurfaceCapabilities2KHR::default();

        unsafe {
            self.instance
                .surface_capabilities()
                .get_physical_device_surface_capabilities2(device, &surface_info, &mut capabilites)
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfaceCapabilities(err.into()))?;

        Ok(capabilites.surface_capabilities)
    }

    fn get_physical_device_surface_formats(
        &self,
        device: vk::PhysicalDevice,
    ) -> RtErr<Vec<vk::SurfaceFormatKHR>> {
        let instance = self.instance.surface_capabilities();
        let surface_info = vk::PhysicalDeviceSurfaceInfo2KHR::default().surface(self.surface);
        let format_count =
            unsafe { instance.get_physical_device_surface_formats2_len(device, &surface_info) }
                .map_err(|err| RtError::GetPhysicalDeviceSurfaceFormats(err.into()))?;
        let mut formats = vec![vk::SurfaceFormat2KHR::default(); format_count];

        unsafe {
            instance.get_physical_device_surface_formats2(device, &surface_info, &mut formats)
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfaceFormats(err.into()))?;

        Ok(formats.iter().map(|format| format.surface_format).collect())
    }

    fn get_physical_device_surface_present_modes(
        &self,
        device: vk::PhysicalDevice,
    ) -> RtErr<Vec<vk::PresentModeKHR>> {
        unsafe {
            self.instance
                .surface_loader()
                .get_physical_device_surface_present_modes(device, self.surface)
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfacePresentModes(err.into()))
    }

    fn create_surface(window: Arc<Window>, instance: Arc<Instance>) -> RtErr<SurfaceKHR> {
        let display_handle = window
            .display_handle()
            .map_err(|err| RtError::DisplayHandle(err.into()))?
            .as_raw();
        let window_handle = window
            .window_handle()
            .map_err(|err| RtError::WindowHandle(err.into()))?
            .as_raw();

        unsafe {
            ash_window::create_surface(
                instance.entry(),
                instance.instance(),
                display_handle,
                window_handle,
                None,
            )
        }
        .map_err(|err| RtError::CreateSurface(err.into()))
    }
}

impl SwapchainSupportDetails {
    pub(crate) fn choose_swap_surface_format(&self) -> vk::SurfaceFormatKHR {
        self.formats
            .iter()
            .copied()
            .find(|format| {
                format.format == vk::Format::B8G8R8A8_SRGB
                    && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(self.formats[0])
    }

    pub(crate) fn choose_swap_present_mode(&self) -> vk::PresentModeKHR {
        self.present_modes
            .iter()
            .copied()
            .find(|present_mode| *present_mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub(crate) fn choose_swap_extent(&self, window: Arc<Window>) -> vk::Extent2D {
        if self.capabilities.current_extent.width != u32::MAX {
            self.capabilities.current_extent
        } else {
            let inner_size = window.inner_size();

            vk::Extent2D {
                width: inner_size.width.clamp(
                    self.capabilities.min_image_extent.width,
                    self.capabilities.max_image_extent.width,
                ),
                height: inner_size.height.clamp(
                    self.capabilities.min_image_extent.height,
                    self.capabilities.max_image_extent.height,
                ),
            }
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.instance
                .surface_loader()
                .destroy_surface(self.surface, None);
        }
    }
}
