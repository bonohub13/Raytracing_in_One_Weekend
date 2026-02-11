use crate::core::error::{RtError, RtResult};
use ash::{Entry, Instance, khr::surface, vk};
use std::sync::Arc;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub struct Surface {
    instance: surface::Instance,
    surface: vk::SurfaceKHR,
}

pub struct SwapchainSupportDetails {
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

impl Surface {
    pub fn new(window: Arc<Window>, entry: &Entry, instance: &Instance) -> RtResult<Self> {
        let surface = Self::create_surface(window, entry, instance)?;
        let instance = surface::Instance::new(entry, instance);

        Ok(Self { instance, surface })
    }

    #[inline]
    pub fn surface(&self) -> &vk::SurfaceKHR {
        &self.surface
    }

    pub fn get_surface_support(
        &self,
        physical_device: &vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> RtResult<bool> {
        match unsafe {
            self.instance.get_physical_device_surface_support(
                *physical_device,
                queue_family_index,
                self.surface,
            )
        } {
            Ok(supported) => Ok(supported),
            Err(err) => Err(RtError::GetSurfaceSupport(err.into())),
        }
    }

    pub fn get_surface_capabilities(
        &self,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<vk::SurfaceCapabilitiesKHR> {
        match unsafe {
            self.instance
                .get_physical_device_surface_capabilities(*physical_device, self.surface)
        } {
            Ok(capabilities) => Ok(capabilities),
            Err(err) => Err(RtError::GetSurfaceCapabilities(err.into())),
        }
    }

    pub fn get_surface_formats(
        &self,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<Vec<vk::SurfaceFormatKHR>> {
        match unsafe {
            self.instance
                .get_physical_device_surface_formats(*physical_device, self.surface)
        } {
            Ok(formats) => Ok(formats),
            Err(err) => Err(RtError::GetSurfaceFormats(err.into())),
        }
    }

    pub fn get_surface_present_modes(
        &self,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<Vec<vk::PresentModeKHR>> {
        match unsafe {
            self.instance
                .get_physical_device_surface_present_modes(*physical_device, self.surface)
        } {
            Ok(present_modes) => Ok(present_modes),
            Err(err) => Err(RtError::GetSurfacePresentModes(err.into())),
        }
    }

    #[inline]
    pub unsafe fn destroy(&mut self) {
        unsafe { self.instance.destroy_surface(self.surface, None) };
    }

    fn create_surface(
        window: Arc<Window>,
        entry: &Entry,
        instance: &Instance,
    ) -> RtResult<vk::SurfaceKHR> {
        let display_handle = match window.display_handle() {
            Ok(handle) => Ok(handle),
            Err(err) => Err(RtError::DisplayHandle(err)),
        }?;
        let window_handle = match window.window_handle() {
            Ok(handle) => Ok(handle),
            Err(err) => Err(RtError::WindowHandle(err)),
        }?;

        match unsafe {
            ash_window::create_surface(
                entry,
                instance,
                display_handle.as_raw(),
                window_handle.as_raw(),
                None,
            )
        } {
            Ok(surface) => Ok(surface),
            Err(err) => Err(RtError::CreateSurface(err.into())),
        }
    }
}

impl SwapchainSupportDetails {
    pub fn query_swapchain_support(
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<Self> {
        let capabilities = surface.get_surface_capabilities(physical_device)?;
        let formats = surface.get_surface_formats(physical_device)?;
        let present_modes = surface.get_surface_present_modes(physical_device)?;

        Ok(Self {
            capabilities,
            formats,
            present_modes,
        })
    }

    #[inline]
    pub fn capabilities(&self) -> &vk::SurfaceCapabilitiesKHR {
        &self.capabilities
    }

    #[inline]
    pub const fn is_adequate(&self) -> bool {
        !self.formats.is_empty() && !self.present_modes.is_empty()
    }

    pub fn choose_swap_surface_format(&self) -> vk::SurfaceFormatKHR {
        self.formats
            .iter()
            .copied()
            .find(|format| {
                (format.format == vk::Format::B8G8R8_SRGB)
                    && (format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
            })
            .unwrap_or(self.formats[0])
    }

    pub fn choose_swap_present_mode(&self) -> vk::PresentModeKHR {
        self.present_modes
            .iter()
            .copied()
            .find(|present_mode| *present_mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn choose_swap_extent(&self, window: Arc<Window>) -> vk::Extent2D {
        if self.capabilities.current_extent.width != u32::MAX {
            return self.capabilities.current_extent;
        }

        let size = window.inner_size();

        vk::Extent2D {
            width: size.width.clamp(
                self.capabilities.min_image_extent.width,
                self.capabilities.max_image_extent.width,
            ),
            height: size.height.clamp(
                self.capabilities.min_image_extent.height,
                self.capabilities.max_image_extent.height,
            ),
        }
    }
}
