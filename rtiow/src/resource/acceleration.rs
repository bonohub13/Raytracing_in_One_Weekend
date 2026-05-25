// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::VkState;
use ash::khr::acceleration_structure;

#[derive(Clone)]
pub struct AsLoader {
    raw: acceleration_structure::Device,
}

impl AsLoader {
    pub fn new(state: &VkState) -> Self {
        let raw =
            acceleration_structure::Device::new(state.instance.instance(), state.device.device());

        Self { raw }
    }

    pub(crate) fn raw(&self) -> &acceleration_structure::Device {
        &self.raw
    }
}
