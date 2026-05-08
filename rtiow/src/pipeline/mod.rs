// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod encoder;
mod graphics;
mod shader;
mod sync;

pub use encoder::*;
pub use graphics::*;
pub(crate) use shader::*;
pub use sync::*;

use crate::{Descriptor, Device, RtErr, RtError, VkState};
use ash::vk;
use std::sync::Arc;

pub struct PipelineLayout {
    pub(crate) layout: vk::PipelineLayout,
    device: Arc<Device>,
}

impl PipelineLayout {
    pub fn new(state: &VkState, descriptors: &[Descriptor]) -> RtErr<Self> {
        let layout = Self::create_pipeline_layout(state.device.clone(), descriptors)?;

        Ok(Self {
            layout,
            device: state.device.clone(),
        })
    }

    fn create_pipeline_layout(
        device: Arc<Device>,
        descriptors: &[Descriptor],
    ) -> RtErr<vk::PipelineLayout> {
        let set_layouts: Vec<_> = descriptors.iter().map(|desc| desc.set_layout()).collect();
        let create_info = vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts);

        unsafe { device.device().create_pipeline_layout(&create_info, None) }
            .map_err(|err| RtError::CreatePipelineLayout(err.into()))
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}
