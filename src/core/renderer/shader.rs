// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

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

pub struct RayTracingShadersDescriptor<'shader> {
    pub device: Arc<Device>,
    pub ray_generation_shader_path: &'shader str,
    pub ray_generation_main: &'shader CStr,
    pub miss_shader_path: &'shader str,
    pub miss_main: &'shader CStr,
    pub triangle_closest_hit_shader_path: &'shader str,
    pub triangle_closest_hit_main: &'shader CStr,
    pub intersection_shader_path: &'shader str,
    pub intersection_main: &'shader CStr,
    pub procedural_closest_hit_shader_path: &'shader str,
    pub procedural_closest_hit_main: &'shader CStr,
}

pub struct RenderShaders {
    device: Arc<Device>,
    vertex: vk::ShaderModule,
    vertex_entry: Box<CStr>,
    fragment: vk::ShaderModule,
    fragment_entry: Box<CStr>,
}

pub struct RayTracingShaders {
    device: Arc<Device>,
    ray_generation: vk::ShaderModule,
    ray_generation_entry: Box<CStr>,
    miss: vk::ShaderModule,
    miss_entry: Box<CStr>,
    triangle_closest_hit: vk::ShaderModule,
    triangle_closest_hit_entry: Box<CStr>,
    intersection: vk::ShaderModule,
    intersection_entry: Box<CStr>,
    procedural_closest_hit: vk::ShaderModule,
    procedural_closest_hit_entry: Box<CStr>,
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

impl RayTracingShaders {
    pub fn new(desc: &RayTracingShadersDescriptor) -> RtResult<Self> {
        let ray_generation =
            load_shader_module(desc.device.clone(), desc.ray_generation_shader_path)?;
        let miss = load_shader_module(desc.device.clone(), desc.miss_shader_path)?;
        let procedural_closest_hit =
            load_shader_module(desc.device.clone(), desc.procedural_closest_hit_shader_path)?;
        let intersection = load_shader_module(desc.device.clone(), desc.intersection_shader_path)?;
        let triangle_closest_hit =
            load_shader_module(desc.device.clone(), desc.triangle_closest_hit_shader_path)?;

        Ok(Self {
            device: desc.device.clone(),
            ray_generation,
            ray_generation_entry: desc.ray_generation_main.into(),
            miss,
            miss_entry: desc.miss_main.into(),
            procedural_closest_hit,
            procedural_closest_hit_entry: desc.procedural_closest_hit_main.into(),
            intersection,
            intersection_entry: desc.intersection_main.into(),
            triangle_closest_hit,
            triangle_closest_hit_entry: desc.triangle_closest_hit_main.into(),
        })
    }

    pub fn shader_stages<'shader>(
        &'shader self,
    ) -> [vk::PipelineShaderStageCreateInfo<'shader>; 5] {
        [
            // Ray Generation
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::RAYGEN_KHR)
                .module(self.ray_generation)
                .name(&self.ray_generation_entry),
            // Miss
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::MISS_KHR)
                .module(self.miss)
                .name(&self.miss_entry),
            // Closest Hit (Triangle mesh)
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::CLOSEST_HIT_KHR)
                .module(self.triangle_closest_hit)
                .name(&self.triangle_closest_hit_entry),
            // Intersection
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::INTERSECTION_KHR)
                .module(self.intersection)
                .name(&self.intersection_entry),
            // Closest Hit (Procedural)
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::CLOSEST_HIT_KHR)
                .module(self.procedural_closest_hit)
                .name(&self.procedural_closest_hit_entry),
        ]
    }
}

impl Drop for RenderShaders {
    fn drop(&mut self) {
        let device = self.device.raw();

        unsafe {
            device.destroy_shader_module(self.vertex, None);
            device.destroy_shader_module(self.fragment, None);
        }
    }
}

impl Drop for RayTracingShaders {
    fn drop(&mut self) {
        let device = self.device.raw();

        unsafe {
            device.destroy_shader_module(self.ray_generation, None);
            device.destroy_shader_module(self.miss, None);
            device.destroy_shader_module(self.triangle_closest_hit, None);
            device.destroy_shader_module(self.intersection, None);
            device.destroy_shader_module(self.procedural_closest_hit, None);
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

    unsafe { device.raw().create_shader_module(&create_info, None) }
        .map_err(|err| RtError::CreateShaderModule(err.into()))
}
