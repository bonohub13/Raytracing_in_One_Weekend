// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{RtError, RtResult, instance::Instance};
use ash::vk::SurfaceKHR;
use std::sync::Arc;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct SurfaceDescriptor {
    pub window: Arc<Window>,
    pub instance: Arc<Instance>,
}

pub struct Surface {
    window: Arc<Window>,
    instance: Arc<Instance>,
    raw: SurfaceKHR,
}

impl Surface {
    pub fn new(desc: &SurfaceDescriptor) -> RtResult<Self> {
        let surface = Self::create_surface(desc)?;

        Ok(Self {
            window: desc.window.clone(),
            instance: desc.instance.clone(),
            raw: surface,
        })
    }

    #[inline]
    pub const fn raw(&self) -> SurfaceKHR {
        self.raw
    }

    pub fn inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window.inner_size()
    }

    fn create_surface(desc: &SurfaceDescriptor) -> RtResult<SurfaceKHR> {
        let raw_window_handle = desc
            .window
            .window_handle()
            .map_err(|err| RtError::WindowHandle(err.into()))?
            .as_raw();
        let raw_display_handle = desc
            .window
            .display_handle()
            .map_err(|err| RtError::DisplayHandle(err.into()))?
            .as_raw();

        unsafe {
            ash_window::create_surface(
                desc.instance.entry(),
                desc.instance.raw(),
                raw_display_handle,
                raw_window_handle,
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
                .destroy_surface(self.raw, None)
        };
    }
}
