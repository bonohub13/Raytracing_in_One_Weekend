// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{
    RtError, RtResult,
    device::Device,
    instance::Instance,
    params,
    renderer::{
        buffer::{self, AccelerationBuffer, DescriptorSet, GraphicsBuffer, ShaderBindingTable},
        command::{self, Command},
        shader,
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
    RayTracing(vk::Pipeline),
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
    sbt: ShaderBindingTable,
    graphics_pipeline: Pipeline,
    rt_pipeline: Pipeline,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set: DescriptorSet,
    graphics_buffer: GraphicsBuffer,
    rt_buffer: AccelerationBuffer,
    swapchain: Swapchain,
    device: Arc<Device>,
}

impl Pipeline {
    pub fn inner(&self) -> vk::Pipeline {
        match self {
            Self::Graphics(pipeline) | Self::RayTracing(pipeline) => *pipeline,
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
    const SBT_NAME: &str = "Shader binding table allocation";

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
        let sync_object = SyncObject::new(&sync::SyncObjectDescriptor {
            device: desc.device.clone(),
        })?;
        let command = Command::new(&command::CommandDescriptor {
            device: desc.device.clone(),
        })?;
        let rt_buffer = AccelerationBuffer::new(&buffer::AccelerationBufferDescriptor {
            device: desc.device.clone(),
            sync_object: &sync_object,
            command: &command,
            camera_desc: buffer::CameraDescriptor {},
        })?;
        let descriptor_set = DescriptorSet::new(&buffer::DescriptorSetDescriptor {
            device: desc.device.clone(),
            acceleration_bindings: &AccelerationBuffer::layout_binding(),
            graphics_bindings: &GraphicsBuffer::layout_bindings(),
        })?;
        let pipeline_layout = Self::create_pipeline_layout(
            desc.device.clone(),
            &[
                descriptor_set.acceleration_layout(),
                descriptor_set.graphics_layout(),
            ],
        )?;
        let graphics_pipeline = Self::create_graphics_pipeline(&swapchain, pipeline_layout, desc)?;
        let rt_pipeline = Self::create_ray_tracing_pipeline(desc.device.clone(), pipeline_layout)?;
        let sbt = ShaderBindingTable::new(&buffer::ShaderBindingTableDescriptor {
            device: desc.device.clone(),
            rt_pipeline,
            name: Self::SBT_NAME,
        })?;

        // Update descriptor sets for graphics buffer (initialization)
        graphics_buffer.write_descriptor_sets(&descriptor_set);

        Ok(Self {
            device: desc.device.clone(),
            swapchain,
            graphics_buffer,
            rt_buffer,
            descriptor_set,
            pipeline_layout,
            graphics_pipeline,
            rt_pipeline,
            sbt,
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
        let command_buffer = self.command.buffers()[self.current_frame];

        self.sync_object.wait_for_fences(self.current_frame)?;
        self.rt_buffer.update_camera()?;
        self.sync_object.reset_fences(self.current_frame)?;
        if let Some((image_index, _is_suboptimal)) = self
            .swapchain
            .acquire_next_image(&self.sync_object, self.current_frame)?
        {
            self.image_index = image_index;
        }
        self.command.reset_command_buffer(command_buffer)?;
        self.record_command_buffer(command_buffer)?;
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

    fn record_command_buffer(&self, command_buffer: vk::CommandBuffer) -> RtResult<()> {
        let device = self.device.raw();

        self.command.begin_command_buffer(command_buffer)?;
        // Ray tracing pipeline
        // TODO: Record TLAS build
        self.transition_to_trace(command_buffer);
        self.trace_rays(command_buffer);
        self.command.bind_pipeline(command_buffer, self.rt_pipeline);
        unsafe {
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::RAY_TRACING_KHR,
                self.pipeline_layout,
                0,
                &[
                    self.descriptor_set.acceleration_sets()[self.current_frame],
                    self.descriptor_set.graphics_sets()[self.current_frame],
                ],
                &[],
            );
        }
        self.transition_texture_to_rt_write(command_buffer);
        // Graphics pipeline
        self.transition_texture_to_read_only(command_buffer);
        self.transition_surface_to_color_attachment(command_buffer);
        self.begin_rendering(command_buffer);
        self.command
            .bind_pipeline(command_buffer, self.graphics_pipeline);
        self.set_viewport_and_scissor(command_buffer);
        unsafe {
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[
                    self.descriptor_set.acceleration_sets()[self.current_frame],
                    self.descriptor_set.graphics_sets()[self.current_frame],
                ],
                &[],
            );
            device.cmd_draw(command_buffer, 3, 1, 0, 0);
        }
        self.end_rendering(command_buffer);
        self.transition_surface_to_present(command_buffer);
        self.command.end_command_buffer(command_buffer)
    }

    fn trace_rays(&self, command_buffer: vk::CommandBuffer) {
        let extent = self.swapchain.extent();

        unsafe {
            self.device.rt_loader().cmd_trace_rays(
                command_buffer,
                self.sbt.ray_generation_region(),
                self.sbt.miss_region(),
                self.sbt.hit_region(),
                self.sbt.call_region(),
                extent.width,
                extent.height,
                1,
            )
        }
    }

    fn begin_rendering(&self, command_buffer: vk::CommandBuffer) {
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
            self.device
                .raw()
                .cmd_begin_rendering(command_buffer, &rendering_info)
        };
    }

    fn end_rendering(&self, command_buffer: vk::CommandBuffer) {
        unsafe { self.device.raw().cmd_end_rendering(command_buffer) }
    }

    fn transition_to_trace(&self, command_buffer: vk::CommandBuffer) {
        let barrier = vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR)
            .src_access_mask(vk::AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR)
            .dst_stage_mask(vk::PipelineStageFlags2::RAY_TRACING_SHADER_KHR)
            .dst_access_mask(vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR);
        let dependency =
            vk::DependencyInfo::default().memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .raw()
                .cmd_pipeline_barrier2(command_buffer, &dependency)
        }
    }

    fn transition_texture_to_rt_write(&self, command_buffer: vk::CommandBuffer) {
        let barrier = self
            .graphics_buffer
            .write_transition_barrier(self.current_frame);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .raw()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info)
        }
    }

    fn transition_texture_to_read_only(&self, command_buffer: vk::CommandBuffer) {
        let barrier = self
            .graphics_buffer
            .render_transition_barrier(self.current_frame);
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .raw()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info)
        }
    }

    fn transition_surface_to_color_attachment(&self, command_buffer: vk::CommandBuffer) {
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
            self.device
                .raw()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info)
        }
    }

    fn transition_surface_to_present(&self, command_buffer: vk::CommandBuffer) {
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
            self.device
                .raw()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info)
        }
    }

    fn set_viewport_and_scissor(&self, command_buffer: vk::CommandBuffer) {
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
            self.device
                .raw()
                .cmd_set_viewport(command_buffer, 0, std::slice::from_ref(&viewport));
            self.device
                .raw()
                .cmd_set_scissor(command_buffer, 0, std::slice::from_ref(&scissor));
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

    fn create_ray_tracing_pipeline(
        device: Arc<Device>,
        pipeline_layout: vk::PipelineLayout,
    ) -> RtResult<Pipeline> {
        const RAY_GENERATION_PATH: &str = "shaders/spv/raygen.spv";
        const RAY_GENERATION_MAIN: &CStr = c"main";
        const MISS_PATH: &str = "shaders/spv/miss.spv";
        const MISS_MAIN: &CStr = c"main";
        const TRIANGLE_CLOSEST_HIT_PATH: &str = "shaders/spv/mesh_closest_hit.spv";
        const TRIANGLE_CLOSEST_HIT_MAIN: &CStr = c"main";
        const INTERSECTION_PATH: &str = "shaders/spv/intersection.spv";
        const INTERSECTION_MAIN: &CStr = c"main";
        const PROCEDURAL_CLOSEST_HIT_PATH: &str = "shaders/spv/proc_closest_hit.spv";
        const PROCEDURAL_CLOSEST_HIT_MAIN: &CStr = c"main";

        let shader_module = shader::RayTracingShaders::new(&shader::RayTracingShadersDescriptor {
            device: device.clone(),
            ray_generation_shader_path: RAY_GENERATION_PATH,
            ray_generation_main: RAY_GENERATION_MAIN,
            miss_shader_path: MISS_PATH,
            miss_main: MISS_MAIN,
            triangle_closest_hit_shader_path: TRIANGLE_CLOSEST_HIT_PATH,
            triangle_closest_hit_main: TRIANGLE_CLOSEST_HIT_MAIN,
            intersection_shader_path: INTERSECTION_PATH,
            intersection_main: INTERSECTION_MAIN,
            procedural_closest_hit_shader_path: PROCEDURAL_CLOSEST_HIT_PATH,
            procedural_closest_hit_main: PROCEDURAL_CLOSEST_HIT_MAIN,
        })?;
        let shader_stages = shader_module.shader_stages();
        let shader_groups = [
            // Ray Generation
            vk::RayTracingShaderGroupCreateInfoKHR::default()
                .ty(vk::RayTracingShaderGroupTypeKHR::GENERAL)
                .general_shader(0)
                .intersection_shader(vk::SHADER_UNUSED_KHR)
                .closest_hit_shader(vk::SHADER_UNUSED_KHR)
                .any_hit_shader(vk::SHADER_UNUSED_KHR),
            // Miss
            vk::RayTracingShaderGroupCreateInfoKHR::default()
                .ty(vk::RayTracingShaderGroupTypeKHR::GENERAL)
                .general_shader(1)
                .intersection_shader(vk::SHADER_UNUSED_KHR)
                .closest_hit_shader(vk::SHADER_UNUSED_KHR)
                .any_hit_shader(vk::SHADER_UNUSED_KHR),
            // Closest Hit (Triangles)
            vk::RayTracingShaderGroupCreateInfoKHR::default()
                .ty(vk::RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP)
                .general_shader(vk::SHADER_UNUSED_KHR)
                .intersection_shader(vk::SHADER_UNUSED_KHR)
                .closest_hit_shader(2)
                .any_hit_shader(vk::SHADER_UNUSED_KHR),
            // Closest Hit (AABBs)
            vk::RayTracingShaderGroupCreateInfoKHR::default()
                .ty(vk::RayTracingShaderGroupTypeKHR::PROCEDURAL_HIT_GROUP)
                .general_shader(vk::SHADER_UNUSED_KHR)
                .intersection_shader(3)
                .closest_hit_shader(4)
                .any_hit_shader(vk::SHADER_UNUSED_KHR),
        ];
        let create_info = vk::RayTracingPipelineCreateInfoKHR::default()
            .stages(&shader_stages)
            .groups(&shader_groups)
            .max_pipeline_ray_recursion_depth(params::RAY_BOUNCE_MAX_DEPTH)
            .layout(pipeline_layout);
        let rt_pipelines = unsafe {
            device.rt_loader().create_ray_tracing_pipelines(
                vk::DeferredOperationKHR::null(),
                vk::PipelineCache::null(),
                std::slice::from_ref(&create_info),
                None,
            )
        }
        .map_err(|(_, err)| RtError::CreatePipeline(Some(err.into())))?;

        if let Some(pipeline) = rt_pipelines.first() {
            Ok(Pipeline::RayTracing(*pipeline))
        } else {
            Err(RtError::CreatePipeline(None))
        }
    }
}

impl Drop for PathTracer {
    fn drop(&mut self) {
        let device = self.device.raw();

        unsafe {
            device.destroy_pipeline(self.graphics_pipeline.inner(), None);
            device.destroy_pipeline(self.rt_pipeline.inner(), None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
        };
    }
}
