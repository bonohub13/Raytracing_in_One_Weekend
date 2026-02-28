// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{
    RtError, RtResult,
    device::Device,
    instance::Instance,
    params,
    renderer::{
        buffer::{self, AccelerationBuffer, DescriptorSet, GraphicsBuffer},
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
    graphics_command: Command,
    acceleration_command: Command,
    graphics_pipeline: Pipeline,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set: DescriptorSet,
    graphics_buffer: GraphicsBuffer,
    acceleration_buffer: AccelerationBuffer,
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
    const SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    };

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
        let acceleration_buffer = AccelerationBuffer::new(&buffer::AccelerationBufferDescriptor {
            device: desc.device.clone(),
            camera_desc: buffer::CameraDescriptor {},
        })?;
        let descriptor_set = DescriptorSet::new(&buffer::DescriptorSetDescriptor {
            device: desc.device.clone(),
            graphics_bindings: &GraphicsBuffer::layout_bindings(),
            acceleration_bindings: &AccelerationBuffer::layout_binding(),
        })?;
        let pipeline_layout = Self::create_pipeline_layout(
            desc.device.clone(),
            &[
                descriptor_set.graphics_layout(),
                descriptor_set.acceleration_layout(),
            ],
        )?;
        let graphics_pipeline = Self::create_graphics_pipeline(&swapchain, pipeline_layout, desc)?;
        let graphics_command = Command::new(&command::CommandDescriptor {
            device: desc.device.clone(),
        })?;
        let acceleration_command = Command::new(&command::CommandDescriptor {
            device: desc.device.clone(),
        })?;
        let sync_object = SyncObject::new(&sync::SyncObjectDescriptor {
            device: desc.device.clone(),
        })?;

        // Update descriptor sets for graphics buffer (initialization)
        graphics_buffer.write_descriptor_sets(&descriptor_set);

        Ok(Self {
            device: desc.device.clone(),
            swapchain,
            graphics_buffer,
            acceleration_buffer,
            descriptor_set,
            pipeline_layout,
            graphics_pipeline,
            graphics_command,
            acceleration_command,
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
        self.acceleration_buffer.update_camera()?;
        self.sync_object.reset_fences(self.current_frame)?;
        if let Some((image_index, _is_suboptimal)) = self
            .swapchain
            .acquire_next_image(&self.sync_object, self.current_frame)?
        {
            self.image_index = image_index;
        }
        self.graphics_command
            .reset_command_buffer(self.current_frame)?;
        self.record_graphics_command_buffer()?;
        self.sync_object
            .graphics_queue_submit(&self.graphics_command, self.current_frame)?;
        let _ = self.sync_object.present_queue(
            &self.swapchain,
            self.image_index as u32,
            self.current_frame,
        )?;

        self.current_frame = (self.current_frame + 1) % params::MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    fn record_graphics_command_buffer(&self) -> RtResult<()> {
        let device = self.device.raw();

        self.graphics_command
            .begin_command_buffer(self.current_frame)?;
        self.transition_texture_to_read_only()?;
        self.transition_surface_to_color_attachment()?;
        self.begin_rendering();
        self.graphics_command
            .bind_pipeline(self.graphics_pipeline, self.current_frame);
        self.set_viewport_and_scissor();
        unsafe {
            device.cmd_bind_descriptor_sets(
                self.graphics_command.buffers()[self.current_frame],
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                std::slice::from_ref(&self.descriptor_set.graphics_sets()[self.current_frame]),
                &[],
            );
            device.cmd_draw(
                self.graphics_command.buffers()[self.current_frame],
                3,
                1,
                0,
                0,
            );
        }
        self.end_rendering();
        self.transition_surface_to_present()?;
        self.graphics_command.end_command_buffer(self.current_frame)
    }

    fn record_acceleration_command_buffer(&self) -> RtResult<()> {
        self.acceleration_command
            .begin_command_buffer(self.current_frame)?;
        self.transition_texture_to_rt_write()?;
        self.acceleration_command
            .end_command_buffer(self.current_frame)
    }

    fn begin_rendering(&self) {
        const BLACK: vk::ClearValue = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0f32, 0f32, 0f32, 1f32],
            },
        };

        let attachment_info = vk::RenderingAttachmentInfo::default()
            .image_view(self.swapchain.image_view()[self.image_index])
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(BLACK);
        let rendering_info = vk::RenderingInfo::default()
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent(),
            })
            .layer_count(1)
            .color_attachments(std::slice::from_ref(&attachment_info));

        unsafe {
            self.device.raw().cmd_begin_rendering(
                self.graphics_command.buffers()[self.current_frame],
                &rendering_info,
            )
        };
    }

    fn end_rendering(&self) {
        unsafe {
            self.device
                .raw()
                .cmd_end_rendering(self.graphics_command.buffers()[self.current_frame])
        }
    }

    fn transition_texture_to_rt_write(&self) -> RtResult<()> {
        let barrier = self
            .graphics_buffer
            .write_transition_barrier(self.current_frame);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device.raw().cmd_pipeline_barrier2(
                self.graphics_command.buffers()[self.current_frame],
                &dependency_info,
            )
        };

        Ok(())
    }

    fn transition_texture_to_read_only(&self) -> RtResult<()> {
        let barrier = self
            .graphics_buffer
            .render_transition_barrier(self.current_frame);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device.raw().cmd_pipeline_barrier2(
                self.graphics_command.buffers()[self.current_frame],
                &dependency_info,
            )
        };

        Ok(())
    }

    fn transition_surface_to_color_attachment(&self) -> RtResult<()> {
        let barrier = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::NONE)
            .src_access_mask(vk::AccessFlags2::NONE)
            .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(
                vk::AccessFlags2::COLOR_ATTACHMENT_WRITE | vk::AccessFlags2::COLOR_ATTACHMENT_READ,
            )
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .image(self.swapchain.images()[self.image_index])
            .subresource_range(Self::SUBRESOURCE_RANGE);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device.raw().cmd_pipeline_barrier2(
                self.graphics_command.buffers()[self.current_frame],
                &dependency_info,
            )
        };

        Ok(())
    }

    fn transition_surface_to_present(&self) -> RtResult<()> {
        let barrier = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::NONE)
            .dst_access_mask(vk::AccessFlags2::NONE)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .image(self.swapchain.images()[self.image_index])
            .subresource_range(Self::SUBRESOURCE_RANGE);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device.raw().cmd_pipeline_barrier2(
                self.graphics_command.buffers()[self.current_frame],
                &dependency_info,
            )
        };

        Ok(())
    }

    fn set_viewport_and_scissor(&self) {
        let swapchain_extent = self.swapchain.extent();
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

        unsafe {
            self.device.raw().cmd_set_viewport(
                self.graphics_command.buffers()[self.current_frame],
                0,
                std::slice::from_ref(&viewport),
            );
            self.device.raw().cmd_set_scissor(
                self.graphics_command.buffers()[self.current_frame],
                0,
                std::slice::from_ref(&scissor),
            );
        }
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
