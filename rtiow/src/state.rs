// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{DebugUtilsMessenger, Device, Instance, InstanceDesc, RtErr, Surface};
use std::{ffi::CStr, sync::Arc};
use winit::window::Window;

pub struct VkState {
    pub(crate) device: Arc<Device>,
    pub(crate) surface: Surface,
    pub(crate) debug_messenger: Option<DebugUtilsMessenger>,
    pub(crate) instance: Instance,
    entry: ash::Entry,
}

#[derive(Debug)]
pub struct VkStateDesc<'desc> {
    pub window: Arc<Window>,
    pub app_name: &'desc CStr,
    pub app_version: u32,
}

impl VkState {
    pub fn new(desc: &VkStateDesc) -> RtErr<Self> {
        let entry = ash::Entry::linked();
        let instance = Instance::new(&InstanceDesc {
            window: desc.window.clone(),
            entry: &entry,
            app_name: desc.app_name,
            app_version: desc.app_version,
        })?;
        #[cfg(debug_assertions)]
        let debug_messenger = Some(DebugUtilsMessenger::new(&entry, &instance)?);
        #[cfg(not(debug_assertions))]
        let debug_messenger = None;
        let surface = Surface::new(desc.window.clone(), &entry, &instance)?;
        let device = Arc::new(Device::new(&instance, &surface)?);

        Ok(Self {
            entry,
            instance,
            debug_messenger,
            surface,
            device,
        })
    }

    pub fn device_wait_idle(&self) -> RtErr<()> {
        self.device.device_wait_idle()
    }
}
