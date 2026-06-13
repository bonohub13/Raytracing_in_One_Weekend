// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod encoder;
mod graphics;
mod ray_tracing;
mod shader;
mod sync;

pub use encoder::*;
pub use graphics::*;
pub use ray_tracing::*;
pub(crate) use shader::*;
pub use sync::*;

use crate::{Descriptor, Device, RtErr, RtError, VkState};
use ash::{ext::vertex_input_dynamic_state, khr::ray_tracing_pipeline, vk};
use std::sync::Arc;

pub struct PipelineLayout {
    pub(crate) layout: vk::PipelineLayout,
    pub(crate) vertex_input_loader: vertex_input_dynamic_state::Device,
    pub(crate) rt_loader: ray_tracing_pipeline::Device,
    device: Arc<Device>,
}

impl PipelineLayout {
    pub fn new(state: &VkState, descriptors: &[Descriptor]) -> RtErr<Self> {
        let layout = Self::create_pipeline_layout(state.device.clone(), descriptors)?;
        let vertex_input_loader = vertex_input_dynamic_state::Device::new(
            state.instance.instance(),
            state.device.device(),
        );
        let rt_loader =
            ray_tracing_pipeline::Device::new(state.instance.instance(), state.device.device());

        Ok(Self {
            layout,
            device: state.device.clone(),
            vertex_input_loader,
            rt_loader,
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
