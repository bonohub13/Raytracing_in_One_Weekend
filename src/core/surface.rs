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
