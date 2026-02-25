// Copyright 2026 Kensuke
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
        let raw_window_handle = match desc.window.window_handle() {
            Ok(window_handle) => Ok(window_handle.as_raw()),
            Err(err) => Err(RtError::WindowHandle(err.into())),
        }?;
        let raw_display_handle = match desc.window.display_handle() {
            Ok(display_handle) => Ok(display_handle.as_raw()),
            Err(err) => Err(RtError::DisplayHandle(err.into())),
        }?;

        match unsafe {
            ash_window::create_surface(
                desc.instance.entry(),
                desc.instance.raw(),
                raw_display_handle,
                raw_window_handle,
                None,
            )
        } {
            Ok(surface) => Ok(surface),
            Err(err) => Err(RtError::CreateSurface(err.into())),
        }
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
