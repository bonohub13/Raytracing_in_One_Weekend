// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Instance, RtErr, RtError};
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

impl Surface {
    pub(crate) fn new(window: Arc<Window>, instance: Arc<Instance>) -> RtErr<Self> {
        let surface = Self::create_surface(window, instance.clone())?;

        Ok(Self { surface, instance })
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

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.instance
                .surface_loader()
                .destroy_surface(self.surface, None);
        }
    }
}
