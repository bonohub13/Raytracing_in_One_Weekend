// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Instance, InstanceDesc, RtErr};
use std::{ffi::CStr, sync::Arc};
use winit::window::Window;

pub struct VkState {
    _instance: Instance,
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

        Ok(Self {
            _instance: instance,
        })
    }
}
