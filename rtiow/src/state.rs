// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{DebugUtilsMessenger, Device, Instance, InstanceDesc, RtErr, Surface};
use std::{ffi::CStr, sync::Arc};
use winit::window::Window;

pub struct VkState {
    _device: Device,
    _surface: Surface,
    _debug_messenger: Option<DebugUtilsMessenger>,
    _instance: Arc<Instance>,
}

#[derive(Debug)]
pub struct VkStateDesc<'desc> {
    pub window: Arc<Window>,
    pub app_name: &'desc CStr,
    pub app_version: u32,
}

impl VkState {
    pub fn new(desc: &VkStateDesc) -> RtErr<Self> {
        let instance = Arc::new(Instance::new(&InstanceDesc {
            window: desc.window.clone(),
            app_name: desc.app_name,
            app_version: desc.app_version,
        })?);
        #[cfg(debug_assertions)]
        let debug_messenger = Some(DebugUtilsMessenger::new(instance.clone())?);
        #[cfg(not(debug_assertions))]
        let debug_messenger = None;
        let surface = Surface::new(desc.window.clone(), instance.clone())?;
        let device = Device::new(instance.clone(), &surface)?;

        Ok(Self {
            _instance: instance,
            _debug_messenger: debug_messenger,
            _surface: surface,
            _device: device,
        })
    }
}
