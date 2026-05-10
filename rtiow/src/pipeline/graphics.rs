// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    Device, RtErr, RtError, Swapchain, VkState,
    pipeline::{PipelineLayout, ShaderModule},
};
use ash::vk;
use std::{ffi::CStr, path::Path, sync::Arc};

pub struct GraphicsPipeline {
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    device: Arc<Device>,
}

impl GraphicsPipeline {
    const VERT_SHADER_PATH: &str = "shaders/spv/vertex.spv";
    const FRAG_SHADER_PATH: &str = "shaders/spv/fragment.spv";
    const DYNAMIC_STATES: [vk::DynamicState; 2] =
        [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

    pub fn new(state: &VkState, swapchain: &Swapchain, layout: &PipelineLayout) -> RtErr<Self> {
        let vert_shader =
            ShaderModule::new(state.device.clone(), Path::new(Self::VERT_SHADER_PATH))?;
        let frag_shader =
            ShaderModule::new(state.device.clone(), Path::new(Self::FRAG_SHADER_PATH))?;
        let pipeline = Self::create_pipeline(
            state.device.clone(),
            layout,
            swapchain,
            &vert_shader,
            &frag_shader,
        )?;

        Ok(Self {
            pipeline,
            layout: layout.layout,
            device: state.device.clone(),
        })
    }

    pub fn bind_pipeline(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device.device().cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            )
        }
    }

    pub fn set_viewport(&self, command_buffer: vk::CommandBuffer, extent: &vk::Extent2D) {
        unsafe {
            self.device.device().cmd_set_viewport(
                command_buffer,
                0,
                std::slice::from_ref(&Self::viewport(extent)),
            )
        }
    }

    pub fn set_scissor(&self, command_buffer: vk::CommandBuffer, extent: &vk::Extent2D) {
        unsafe {
            self.device.device().cmd_set_scissor(
                command_buffer,
                0,
                std::slice::from_ref(&Self::scissor(extent)),
            )
        }
    }

    pub fn draw(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.device.device().cmd_draw(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            );
        }
    }

    fn shader_stages<'info>(
        vert_shader: &'info ShaderModule,
        frag_shader: &'info ShaderModule,
    ) -> [vk::PipelineShaderStageCreateInfo<'info>; 2] {
        const ENTRY_NAME: &CStr = c"main";

        [
            // Vertex Shader
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_shader.shader())
                .name(ENTRY_NAME),
            // Fragment Shader
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_shader.shader())
                .name(ENTRY_NAME),
        ]
    }

    const fn viewport(extent: &vk::Extent2D) -> vk::Viewport {
        vk::Viewport {
            x: 0f32,
            y: 0f32,
            min_depth: 0f32,
            max_depth: 1f32,
            width: extent.width as f32,
            height: extent.height as f32,
        }
    }

    const fn scissor(extent: &vk::Extent2D) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: *extent,
        }
    }

    fn create_pipeline(
        device: Arc<Device>,
        pipeline_layout: &PipelineLayout,
        swapchain: &Swapchain,
        vert_shader: &ShaderModule,
        frag_shader: &ShaderModule,
    ) -> RtErr<vk::Pipeline> {
        static BLEND_CONSTANTS: [f32; 4] = [0f32, 0f32, 0f32, 1f32];
        static COLOR_FORMATS: [vk::Format; 1] = [vk::Format::B8G8R8A8_SRGB];

        let shader_stages = Self::shader_stages(vert_shader, frag_shader);
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&Self::DYNAMIC_STATES);
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
        let viewport = Self::viewport(swapchain.extent());
        let scissor = Self::scissor(swapchain.extent());
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));
        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1f32)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false);
        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(true)
            .min_sample_shading(0.2)
            .rasterization_samples(swapchain.samples_count);
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD);
        let depth_attachment = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::GREATER_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);
        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(true)
            .attachments(std::slice::from_ref(&color_blend_attachment))
            .blend_constants(BLEND_CONSTANTS);
        let mut rendering_pipeline_info = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&COLOR_FORMATS)
            .depth_attachment_format(vk::Format::D32_SFLOAT);

        let create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .depth_stencil_state(&depth_attachment)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout.layout)
            .push_next(&mut rendering_pipeline_info);

        unsafe {
            device.device().create_graphics_pipelines(
                vk::PipelineCache::null(),
                std::slice::from_ref(&create_info),
                None,
            )
        }
        .map(|pipelines| pipelines[0])
        .map_err(|(_, err)| RtError::CreatePipeline(err.into()))
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            self.device.device().destroy_pipeline(self.pipeline, None);
        }
    }
}
