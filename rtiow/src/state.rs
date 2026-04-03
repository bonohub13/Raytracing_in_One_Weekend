// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{DebugUtilsMessenger, Device, Instance, InstanceDesc, RtErr, Surface, Swapchain};
use std::{ffi::CStr, sync::Arc};
use winit::window::Window;

pub struct VkState {
    instance: Instance,
    debug_messenger: Option<DebugUtilsMessenger>,
    surface: Surface,
    device: Device,
    swapchain: Swapchain,
}

#[derive(Debug)]
pub struct VkStateDesc<'desc> {
    pub window: Arc<Window>,
    pub app_name: &'desc CStr,
    pub app_version: u32,
}

impl VkState {
    pub fn new(desc: &VkStateDesc) -> RtErr<Self> {
        let instance = Instance::new(&InstanceDesc {
            window: desc.window.clone(),
            app_name: desc.app_name,
            app_version: desc.app_version,
        })?;
        #[cfg(debug_assertions)]
        let debug_messenger = Some(DebugUtilsMessenger::new(&instance)?);
        #[cfg(not(debug_assertions))]
        let debug_messenger = None;
        let surface = Surface::new(desc.window.clone(), &instance)?;
        let device = Device::new(&instance, &surface)?;
        let swapchain = Swapchain::new(desc.window.clone(), &instance, &surface, &device)?;

        Ok(Self {
            instance,
            debug_messenger,
            surface,
            device,
            swapchain,
        })
    }
}

impl Drop for VkState {
    fn drop(&mut self) {
        while let Err(err) = self.device.device_wait_idle() {
            eprintln!("{err}");
        }

        unsafe {
            self.swapchain.destroy();
            self.device.destroy();
            self.surface.destroy();
        }
        if let Some(debug_messenger) = self.debug_messenger.take() {
            unsafe { debug_messenger.destroy() }
        }
        unsafe {
            self.instance.destroy();
        }
    }
}
