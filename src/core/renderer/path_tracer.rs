// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{
    RtError, RtResult,
    device::Device,
    instance::Instance,
    params,
    renderer::{
        buffer::{self, DescriptorSet, GraphicsBuffer},
        command::{self, Command},
        shader::{self},
        swapchain::{Swapchain, SwapchainDescriptor},
        sync::{self, SyncObject},
    },
    surface::Surface,
};
use ash::vk;
use std::{ffi::CStr, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub enum Pipeline {
    Graphics(vk::Pipeline),
}

pub struct PathTracerDescriptor {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
}

pub struct PathTracer {
    image_index: usize,
    current_frame: usize,
    sync_object: SyncObject,
    command: Command,
    graphics_pipeline: Pipeline,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set: DescriptorSet,
    graphics_buffer: GraphicsBuffer,
    swapchain: Swapchain,
    device: Arc<Device>,
}

impl Pipeline {
    pub fn inner(&self) -> vk::Pipeline {
        match self {
            Self::Graphics(pipeline) => *pipeline,
        }
    }
}

impl PathTracer {
    const PROJECTION_TEXTURE_NAME: &str = "Frame image texture";
    pub fn new(desc: &PathTracerDescriptor) -> RtResult<Self> {
        let swapchain = Swapchain::new(&SwapchainDescriptor {
            instance: desc.instance.clone(),
            surface: desc.surface.clone(),
            device: desc.device.clone(),
        })?;
        let graphics_buffer = GraphicsBuffer::new(&buffer::GraphicsBufferDescriptor {
            projection_texture_name: Self::PROJECTION_TEXTURE_NAME,
            surface: desc.surface.clone(),
            device: desc.device.clone(),
        })?;
        let descriptor_set = DescriptorSet::new(&buffer::DescriptorSetDescriptor {
            device: desc.device.clone(),
            bindings: &[GraphicsBuffer::layout_bindings()].concat(),
        })?;
        let pipeline_layout =
            Self::create_pipeline_layout(desc.device.clone(), &[descriptor_set.layout()])?;
        let graphics_pipeline = Self::create_graphics_pipeline(&swapchain, pipeline_layout, desc)?;
        let command = Command::new(&command::CommandDescriptor {
            device: desc.device.clone(),
        })?;
        let sync_object = SyncObject::new(&sync::SyncObjectDescriptor {
            device: desc.device.clone(),
        })?;

        Ok(Self {
            device: desc.device.clone(),
            swapchain,
            graphics_buffer,
            descriptor_set,
            pipeline_layout,
            graphics_pipeline,
            command,
            sync_object,
            image_index: 0,
            current_frame: 0,
        })
    }

    pub fn resize(&mut self) -> RtResult<()> {
        self.swapchain.recreate_swapchain()?;

        Ok(())
    }

    pub fn render_frame(&mut self) -> RtResult<()> {
        self.sync_object.wait_for_fences(self.current_frame)?;
        self.sync_object.reset_fences(self.current_frame)?;
        if let Some((image_index, _is_suboptimal)) = self
            .swapchain
            .acquire_next_image(&self.sync_object, self.current_frame)?
        {
            self.image_index = image_index;
        }
        self.command.reset_command_buffer(self.current_frame)?;
        self.command
            .record_command_buffer(&command::CommandRecordDescriptor {
                swapchain: &self.swapchain,
                descriptor_set: &self.descriptor_set,
                graphics_buffer: &self.graphics_buffer,
                pipeline_layout: self.pipeline_layout,
                pipeline: self.graphics_pipeline,
                image_index: self.image_index,
                current_frame: self.current_frame,
            })?;
        self.sync_object
            .graphics_queue_submit(&self.command, self.current_frame)?;
        let _ = self.sync_object.present_queue(
            &self.swapchain,
            self.image_index as u32,
            self.current_frame,
        )?;

        self.current_frame = (self.current_frame + 1) % params::MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    fn create_pipeline_layout(
        device: Arc<Device>,
        set_layouts: &[vk::DescriptorSetLayout],
    ) -> RtResult<vk::PipelineLayout> {
        let create_info = vk::PipelineLayoutCreateInfo::default().set_layouts(set_layouts);

        match unsafe { device.raw().create_pipeline_layout(&create_info, None) } {
            Ok(layout) => Ok(layout),
            Err(err) => Err(RtError::CreatePipelineLayout(err.into())),
        }
    }

    fn create_graphics_pipeline(
        swapchain: &Swapchain,
        pipeline_layout: vk::PipelineLayout,
        desc: &PathTracerDescriptor,
    ) -> RtResult<Pipeline> {
        const VERTEX_SHADER_PATH: &str = "shaders/spv/vertex.spv";
        const VERTEX_ENTRY: &CStr = c"main";
        const FRAGMENT_SHADER_PATH: &str = "shaders/spv/fragment.spv";
        const FRAGMENT_ENTRY: &CStr = c"main";
        const DYNAMIC_STATES: [vk::DynamicState; 2] =
            [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

        let shader_module = shader::RenderShaders::new(&shader::RenderShadersDescriptor {
            device: desc.device.clone(),
            vertex_shader_path: VERTEX_SHADER_PATH,
            vertex_main: VERTEX_ENTRY,
            fragment_shader_path: FRAGMENT_SHADER_PATH,
            fragment_main: FRAGMENT_ENTRY,
        })?;
        let shader_stages = shader_module.shader_stages();
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&DYNAMIC_STATES);
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
        let (swapchain_extent, swapchain_format) = (swapchain.extent(), swapchain.image_format());
        let viewport = vk::Viewport {
            x: 0f32,
            y: 0f32,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0f32,
            max_depth: 1f32,
        };
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        };
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .viewports(std::slice::from_ref(&viewport))
            .scissor_count(1)
            .scissors(std::slice::from_ref(&scissor));
        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_bias_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1f32)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);
        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .min_sample_shading(1f32)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD);
        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(std::slice::from_ref(&color_blend_attachment))
            .blend_constants([0f32; 4]);
        let mut pipeline_rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(std::slice::from_ref(&swapchain_format));
        let create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout)
            .render_pass(vk::RenderPass::null())
            .subpass(0)
            .push_next(&mut pipeline_rendering);

        match unsafe {
            desc.device.raw().create_graphics_pipelines(
                vk::PipelineCache::null(),
                std::slice::from_ref(&create_info),
                None,
            )
        } {
            Ok(graphics_pipeline) => {
                if let Some(pipeline) = graphics_pipeline.first() {
                    Ok(Pipeline::Graphics(*pipeline))
                } else {
                    Err(RtError::CreatePipeline(None))
                }
            }
            Err((_, err)) => Err(RtError::CreatePipeline(Some(err.into()))),
        }
    }
}

impl Drop for PathTracer {
    fn drop(&mut self) {
        unsafe {
            self.device
                .raw()
                .destroy_pipeline(self.graphics_pipeline.inner(), None);
            self.device
                .raw()
                .destroy_pipeline_layout(self.pipeline_layout, None);
        };
    }
}
