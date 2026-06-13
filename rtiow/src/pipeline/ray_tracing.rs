// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    Allocator, Device, PipelineLayout, RtErr, RtError, SbtLayout, ShaderBindingTable, ShaderModule,
    VkState,
};
use ash::vk;
use std::{
    mem::ManuallyDrop,
    path::Path,
    sync::{Arc, Mutex},
};

pub struct RayTracingPipeline {
    pipeline: vk::Pipeline,
    layout: Arc<PipelineLayout>,
    shader_binding_table: ManuallyDrop<ShaderBindingTable>,
    device: Arc<Device>,
}

impl RayTracingPipeline {
    const RAYGEN_SHADER_PATH: &str = "shaders/spv/raygen.spv";
    const MISS_SHADER_PATH: &str = "shaders/spv/miss.spv";
    const MESH_CLOSEST_HIT_SHADER_PATH: &str = "shaders/spv/mesh_closest_hit.spv";
    const AABB_CLOSEST_HIT_SHADER_PATH: &str = "shaders/spv/aabb_closest_hit.spv";
    const AABB_INTERSECTION_SHADER_PATH: &str = "shaders/spv/aabb_intersection.spv";

    pub fn new(
        state: &VkState,
        layout: Arc<PipelineLayout>,
        allocator: Arc<Mutex<Allocator>>,
    ) -> RtErr<Self> {
        let raygen_shader =
            ShaderModule::new(state.device.clone(), Path::new(Self::RAYGEN_SHADER_PATH))?;
        let miss_shader =
            ShaderModule::new(state.device.clone(), Path::new(Self::MISS_SHADER_PATH))?;
        let mesh_hit_shader = ShaderModule::new(
            state.device.clone(),
            Path::new(Self::MESH_CLOSEST_HIT_SHADER_PATH),
        )?;
        let aabb_hit_shader = ShaderModule::new(
            state.device.clone(),
            Path::new(Self::AABB_CLOSEST_HIT_SHADER_PATH),
        )?;
        let aabb_intersection_shader = ShaderModule::new(
            state.device.clone(),
            Path::new(Self::AABB_INTERSECTION_SHADER_PATH),
        )?;
        let pipeline = Self::create_pipeline(
            layout.clone(),
            &raygen_shader,
            &miss_shader,
            &mesh_hit_shader,
            Some(&aabb_hit_shader),
            Some(&aabb_intersection_shader),
        )?;
        let sbt_layout = ShaderBindingTable::calculate_sbt_layout(state);
        let handle_data = Self::handle_data(
            layout.clone(),
            pipeline,
            &sbt_layout,
            ShaderBindingTable::GROUP_COUNT,
        )?;
        let shader_binding_table = ManuallyDrop::new(ShaderBindingTable::new(
            state,
            allocator.clone(),
            &sbt_layout,
            &handle_data,
        )?);

        Ok(Self {
            pipeline,
            layout,
            shader_binding_table,
            device: state.device.clone(),
        })
    }
    pub fn bind_pipeline(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device.device().cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::RAY_TRACING_KHR,
                self.pipeline,
            )
        }
    }

    pub fn trace_rays(
        &self,
        command_buffer: vk::CommandBuffer,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            self.layout.rt_loader.cmd_trace_rays(
                command_buffer,
                self.shader_binding_table.raygen_region(),
                self.shader_binding_table.miss_region(),
                self.shader_binding_table.hit_region(),
                self.shader_binding_table.callable_region(),
                width,
                height,
                depth,
            )
        }
    }

    fn create_pipeline(
        pipeline_layout: Arc<PipelineLayout>,
        raygen_shader: &ShaderModule,
        miss_shader: &ShaderModule,
        mesh_closest_hit_shader: &ShaderModule,
        aabb_closest_hit_shader: Option<&ShaderModule>,
        aabb_intersection_shader: Option<&ShaderModule>,
    ) -> RtErr<vk::Pipeline> {
        const RAYGEN_INDEX: u32 = 0;
        const MISS_INDEX: u32 = 1;
        const MESH_CLOSEST_HIT_INDEX: u32 = 2;
        const AABB_CLOSEST_HIT_INDEX: u32 = 3;
        const AABB_INTERSECTION_INDEX: u32 = 4;
        const MAX_PIPELINE_RAY_RECURSION_DEPTH: u32 = 2;

        #[cfg(debug_assertions)]
        assert!(aabb_closest_hit_shader.is_some() == aabb_intersection_shader.is_some());

        let shader_stages = if let Some(closest_hit) = aabb_closest_hit_shader
            && let Some(intersection) = aabb_intersection_shader
        {
            vec![
                // 0: Ray Gen shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::RAYGEN_KHR)
                    .module(raygen_shader.shader())
                    .name(c"main"),
                // 1: Miss shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::MISS_KHR)
                    .module(miss_shader.shader())
                    .name(c"main"),
                // 2: Mesh Closest Hit shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::CLOSEST_HIT_KHR)
                    .module(mesh_closest_hit_shader.shader())
                    .name(c"main"),
                // 2: AABB Closest Hit shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::CLOSEST_HIT_KHR)
                    .module(closest_hit.shader())
                    .name(c"main"),
                // 4: AABB Intersection shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::INTERSECTION_KHR)
                    .module(intersection.shader())
                    .name(c"main"),
            ]
        } else {
            vec![
                // 0: Ray Gen shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::RAYGEN_KHR)
                    .module(raygen_shader.shader())
                    .name(c"main"),
                // 1: Miss shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::MISS_KHR)
                    .module(miss_shader.shader())
                    .name(c"main"),
                // 2: Mesh Closest Hit shader
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlags::CLOSEST_HIT_KHR)
                    .module(mesh_closest_hit_shader.shader())
                    .name(c"main"),
            ]
        };
        let shader_groups =
            if aabb_closest_hit_shader.is_some() && aabb_intersection_shader.is_some() {
                vec![
                    // Group 0: Ray Generation Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::GENERAL)
                        .general_shader(RAYGEN_INDEX)
                        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(vk::SHADER_UNUSED_KHR),
                    // Group 1: Miss Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::GENERAL)
                        .general_shader(MISS_INDEX)
                        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(vk::SHADER_UNUSED_KHR),
                    // Group 2: Mesh Closest Hit Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP)
                        .general_shader(vk::SHADER_UNUSED_KHR)
                        .closest_hit_shader(MESH_CLOSEST_HIT_INDEX)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(vk::SHADER_UNUSED_KHR),
                    // Group 3: AABB Closest Hit Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::PROCEDURAL_HIT_GROUP)
                        .general_shader(vk::SHADER_UNUSED_KHR)
                        .closest_hit_shader(AABB_CLOSEST_HIT_INDEX)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(AABB_INTERSECTION_INDEX),
                ]
            } else {
                vec![
                    // Group 0: Ray Generation Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::GENERAL)
                        .general_shader(RAYGEN_INDEX)
                        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(vk::SHADER_UNUSED_KHR),
                    // Group 1: Miss Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::GENERAL)
                        .general_shader(MISS_INDEX)
                        .closest_hit_shader(vk::SHADER_UNUSED_KHR)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(vk::SHADER_UNUSED_KHR),
                    // Group 2: Mesh Closest Hit Group
                    vk::RayTracingShaderGroupCreateInfoKHR::default()
                        .ty(vk::RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP)
                        .general_shader(vk::SHADER_UNUSED_KHR)
                        .closest_hit_shader(MESH_CLOSEST_HIT_INDEX)
                        .any_hit_shader(vk::SHADER_UNUSED_KHR)
                        .intersection_shader(vk::SHADER_UNUSED_KHR),
                ]
            };
        let create_info = vk::RayTracingPipelineCreateInfoKHR::default()
            .stages(&shader_stages)
            .groups(&shader_groups)
            .max_pipeline_ray_recursion_depth(MAX_PIPELINE_RAY_RECURSION_DEPTH)
            .layout(pipeline_layout.layout);

        unsafe {
            pipeline_layout.rt_loader.create_ray_tracing_pipelines(
                vk::DeferredOperationKHR::null(),
                vk::PipelineCache::null(),
                std::slice::from_ref(&create_info),
                None,
            )
        }
        .map(|pipelines| pipelines[0])
        .map_err(|(_, err)| RtError::CreatePipeline(err.into()))
    }

    pub(crate) fn handle_data(
        layout: Arc<PipelineLayout>,
        pipeline: vk::Pipeline,
        sbt_layout: &SbtLayout,
        group_count: usize,
    ) -> RtErr<Vec<u8>> {
        unsafe {
            layout.rt_loader.get_ray_tracing_shader_group_handles(
                pipeline,
                0,
                group_count as u32,
                sbt_layout.handle_size as usize * group_count,
            )
        }
        .map_err(|err| RtError::GetRayTracingShaderGroupHandles(err.into()))
    }
}

impl Drop for RayTracingPipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.device().destroy_pipeline(self.pipeline, None);
            ManuallyDrop::drop(&mut self.shader_binding_table);
        }
    }
}
