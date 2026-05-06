mod encoder;
mod graphics;
mod shader;

pub use encoder::*;
pub use graphics::*;
pub(crate) use shader::*;

use crate::{Device, RtErr, RtError, VkState};
use ash::vk;
use std::sync::Arc;

pub struct PipelineLayout {
    pub(crate) layout: vk::PipelineLayout,
    device: Arc<Device>,
}

impl PipelineLayout {
    pub fn new(state: &VkState) -> RtErr<Self> {
        let layout = Self::create_pipeline_layout(state.device.clone())?;

        Ok(Self {
            layout,
            device: state.device.clone(),
        })
    }

    fn create_pipeline_layout(device: Arc<Device>) -> RtErr<vk::PipelineLayout> {
        let create_info = vk::PipelineLayoutCreateInfo::default();

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
