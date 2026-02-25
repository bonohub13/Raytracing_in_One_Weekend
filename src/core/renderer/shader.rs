// Copyright 2026 Kensuke
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::core::{RtError, RtResult, device::Device};
use ash::{util as vk_util, vk};
use std::{ffi::CStr, fs::File, sync::Arc};

pub struct RenderShadersDescriptor<'shader> {
    pub device: Arc<Device>,
    pub vertex_shader_path: &'shader str,
    pub vertex_main: &'shader CStr,
    pub fragment_shader_path: &'shader str,
    pub fragment_main: &'shader CStr,
}

pub struct RenderShaders {
    device: Arc<Device>,
    vertex: vk::ShaderModule,
    vertex_entry: Box<CStr>,
    fragment: vk::ShaderModule,
    fragment_entry: Box<CStr>,
}

impl RenderShaders {
    pub fn new(desc: &RenderShadersDescriptor) -> RtResult<Self> {
        let vertex = load_shader_module(desc.device.clone(), desc.vertex_shader_path)?;
        let fragment = load_shader_module(desc.device.clone(), desc.fragment_shader_path)?;

        Ok(Self {
            device: desc.device.clone(),
            vertex,
            vertex_entry: desc.vertex_main.into(),
            fragment,
            fragment_entry: desc.fragment_main.into(),
        })
    }

    pub fn shader_stages<'shader>(
        &'shader self,
    ) -> [vk::PipelineShaderStageCreateInfo<'shader>; 2] {
        [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(self.vertex)
                .name(&self.vertex_entry),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(self.fragment)
                .name(&self.fragment_entry),
        ]
    }
}

impl Drop for RenderShaders {
    fn drop(&mut self) {
        unsafe {
            self.device.raw().destroy_shader_module(self.vertex, None);
            self.device.raw().destroy_shader_module(self.fragment, None);
        }
    }
}

fn load_shader_module(device: Arc<Device>, shader_path: &str) -> RtResult<vk::ShaderModule> {
    let mut file = match File::open(shader_path) {
        Ok(file) => Ok(file),
        Err(err) => Err(RtError::LoadShader(err.into())),
    }?;
    let shader_payload = match vk_util::read_spv(&mut file) {
        Ok(payload) => Ok(payload),
        Err(err) => Err(RtError::LoadShader(err.into())),
    }?;
    let create_info = vk::ShaderModuleCreateInfo::default().code(&shader_payload);

    match unsafe { device.raw().create_shader_module(&create_info, None) } {
        Ok(shader_module) => Ok(shader_module),
        Err(err) => Err(RtError::CreateShaderModule(err.into())),
    }
}
