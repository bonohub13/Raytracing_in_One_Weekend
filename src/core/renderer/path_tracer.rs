use crate::core::{
    buffers::Buffers,
    error::{RtError, RtResult},
    objects::Camera,
    renderer::{Command, RenderPass, Swapchain, Sync},
    shader::{ComputeShader, ComputeShaderDescriptor, RenderShader, RenderShaderDescriptor},
    surface::Surface,
};
use ash::{Device, Instance, vk};
use std::sync::Arc;
use winit::window::Window;

pub struct PathTracerDescriptor<'vk, 'entry> {
    pub window: Arc<Window>,
    pub instance: &'vk Instance,
    pub surface: &'vk Surface,
    pub physical_device: &'vk vk::PhysicalDevice,
    pub device: &'vk Device,
    pub render_shader_desc: &'vk RenderShaderDescriptor<'entry>,
    pub compute_shader_desc: &'vk ComputeShaderDescriptor<'entry>,
    pub camera: &'vk Camera,
}

pub struct PathTracer {
    swapchain: Swapchain,
    render_pass: RenderPass,
    buffers: Buffers,
    layout: vk::PipelineLayout,
    compute_pipeline: vk::Pipeline,
    graphics_pipeline: vk::Pipeline,
    command: Command,
    sync_object: Sync,
    current_image_index: usize,
}

impl PathTracer {
    pub fn new(desc: &PathTracerDescriptor) -> RtResult<Self> {
        let swapchain = Swapchain::new(
            desc.window.clone(),
            desc.instance,
            desc.surface,
            desc.physical_device,
            desc.device,
        )?;
        let render_pass = RenderPass::new(desc.device, &swapchain)?;
        let buffers = Buffers::new(
            desc.window.clone(),
            desc.instance,
            desc.device,
            desc.physical_device,
            desc.camera,
        )?;
        let layout = Self::create_layout(desc.device, buffers.descriptor_set_layouts())?;
        let compute_pipeline =
            Self::create_compute_pipeline(desc.device, &layout, desc.compute_shader_desc)?;
        let graphics_pipeline = Self::create_graphics_pipeline(
            desc.device,
            swapchain.extent(),
            &layout,
            &render_pass,
            desc.render_shader_desc,
        )?;
        let command = Command::new(
            desc.instance,
            desc.surface,
            desc.physical_device,
            desc.device,
        )?;
        let sync_object = Sync::new(desc.device)?;

        Ok(Self {
            swapchain,
            render_pass,
            buffers,
            layout,
            compute_pipeline,
            graphics_pipeline,
            command,
            sync_object,
            current_image_index: 0,
        })
    }

    #[inline]
    pub fn swapchain(&self) -> &Swapchain {
        &self.swapchain
    }

    #[inline]
    pub fn sync_object(&self) -> &Sync {
        &self.sync_object
    }

    #[inline]
    pub fn command(&self) -> &Command {
        &self.command
    }

    pub fn compute_frame(&mut self, device: &ash::Device, current_frame: usize) -> RtResult<()> {
        self.sync_object
            .wait_for_compute_fence(device, current_frame)?;
        self.sync_object
            .reset_compute_fence(device, current_frame)?;
        self.command
            .reset_compute_command_buffer(device, current_frame)?;
        self.record_compute_command_buffer(device, current_frame)
    }

    pub fn draw_frame(
        &mut self,
        window: Arc<Window>,
        instance: &ash::Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<usize> {
        self.sync_object
            .wait_for_render_fence(device, current_frame)?;
        match self
            .swapchain
            .acquire_next_image(&self.sync_object, current_frame)
        {
            Ok(result) => {
                if let Some((image_index, _)) = result {
                    self.current_image_index = image_index;
                } else {
                    self.resize(window.clone(), instance, surface, physical_device, device)?;

                    return Ok(self.current_image_index);
                }
            }
            Err(err) => return Err(err),
        };
        self.sync_object.reset_render_fence(device, current_frame)?;
        self.command
            .reset_render_command_buffer(device, current_frame)?;
        self.record_command_buffer(device, self.current_image_index, current_frame)?;

        Ok(self.current_image_index)
    }

    pub fn resize(
        &mut self,
        window: Arc<Window>,
        instance: &ash::Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
        device: &ash::Device,
    ) -> RtResult<()> {
        if let Err(err) = unsafe { device.device_wait_idle() } {
            return Err(RtError::DeviceWaitIdle(err.into()));
        }
        self.swapchain
            .resize(window.clone(), instance, surface, physical_device, device)?;
        self.render_pass.resize(device, &self.swapchain)?;
        self.buffers.resize(window, device)
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) -> RtResult<()> {
        unsafe {
            self.sync_object.destroy(device);
            self.command.destroy(device);
            device.destroy_pipeline(self.compute_pipeline, None);
            device.destroy_pipeline(self.graphics_pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
            self.buffers.destroy(device)?;
            self.render_pass.destroy(device);
            self.swapchain.destroy(device);
        }

        Ok(())
    }

    fn record_compute_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.command
            .begin_compute_command_buffer(device, current_frame)?;
        self.compute(device, current_frame);
        self.command
            .end_compute_command_buffer(device, current_frame)
    }

    fn record_command_buffer(
        &self,
        device: &ash::Device,
        image_index: usize,
        current_frame: usize,
    ) -> RtResult<()> {
        self.command
            .begin_render_command_buffer(device, current_frame)?;
        self.render(device, image_index, current_frame);
        self.command
            .end_render_command_buffer(device, current_frame)
    }

    fn compute(&self, device: &ash::Device, current_frame: usize) {
        let barrier = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::GENERAL)
            .dst_access_mask(vk::AccessFlags::SHADER_WRITE)
            .image(*self.buffers.texture().image())
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        unsafe {
            device.cmd_pipeline_barrier(
                *self.command.compute_buffer(current_frame),
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                std::slice::from_ref(&barrier),
            );
            device.cmd_bind_pipeline(
                *self.command.compute_buffer(current_frame),
                vk::PipelineBindPoint::COMPUTE,
                self.compute_pipeline,
            );
            device.cmd_bind_descriptor_sets(
                *self.command.compute_buffer(current_frame),
                vk::PipelineBindPoint::COMPUTE,
                self.layout,
                0,
                std::slice::from_ref(&self.buffers.compute_sets()[current_frame]),
                &[],
            );
            device.cmd_dispatch(
                *self.command.compute_buffer(current_frame),
                self.buffers.camera().width.div_ceil(16),
                self.buffers.camera().height.div_ceil(16),
                1,
            );
        }
    }

    fn render(&self, device: &ash::Device, image_index: usize, current_frame: usize) {
        const RENDER_AREA_OFFSET: vk::Offset2D = vk::Offset2D { x: 0, y: 0 };
        const CLEAR_COLORS: [vk::ClearValue; 1] = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0f32, 0f32, 0f32, 1f32],
            },
        }];

        let scissor = vk::Rect2D {
            offset: RENDER_AREA_OFFSET,
            extent: *self.swapchain.extent(),
        };
        let begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(*self.render_pass.render_pass())
            .framebuffer(self.render_pass.framebuffers()[image_index])
            .render_area(scissor)
            .clear_values(&CLEAR_COLORS);
        let viewport = vk::Viewport::default()
            .x(0f32)
            .y(0f32)
            .width(self.swapchain.extent().width as f32)
            .height(self.swapchain.extent().height as f32)
            .min_depth(0f32)
            .max_depth(1f32);
        let barrier = vk::ImageMemoryBarrier::default()
            .old_layout(vk::ImageLayout::GENERAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::SHADER_READ)
            .image(*self.buffers.texture().image())
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        unsafe {
            device.cmd_pipeline_barrier(
                *self.command.buffer(current_frame),
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::FRAGMENT_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                std::slice::from_ref(&barrier),
            );
            device.cmd_begin_render_pass(
                *self.command.buffer(current_frame),
                &begin_info,
                vk::SubpassContents::INLINE,
            );
            device.cmd_bind_pipeline(
                *self.command.buffer(current_frame),
                vk::PipelineBindPoint::GRAPHICS,
                self.graphics_pipeline,
            );
            device.cmd_bind_descriptor_sets(
                *self.command.buffer(current_frame),
                vk::PipelineBindPoint::GRAPHICS,
                self.layout,
                1,
                std::slice::from_ref(&self.buffers.graphics_sets()[0]),
                &[],
            );
            device.cmd_set_viewport(
                *self.command.buffer(current_frame),
                0,
                std::slice::from_ref(&viewport),
            );
            device.cmd_set_scissor(
                *self.command.buffer(current_frame),
                0,
                std::slice::from_ref(&scissor),
            );
            device.cmd_draw(*self.command.buffer(current_frame), 3, 1, 0, 0);
            device.cmd_end_render_pass(*self.command.buffer(current_frame));
        }
    }

    fn create_layout(
        device: &ash::Device,
        layouts: &[vk::DescriptorSetLayout],
    ) -> RtResult<vk::PipelineLayout> {
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default().set_layouts(layouts);

        match unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) } {
            Ok(layout) => Ok(layout),
            Err(err) => Err(RtError::CreatePipelineLayout(err.into())),
        }
    }

    fn create_graphics_pipeline(
        device: &ash::Device,
        swapchain_extent: &vk::Extent2D,
        layout: &vk::PipelineLayout,
        render_pass: &RenderPass,
        desc: &RenderShaderDescriptor,
    ) -> RtResult<vk::Pipeline> {
        const DYNAMIC_STATES: [vk::DynamicState; 2] =
            [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

        let mut render_shader = RenderShader::new(device, desc)?;
        let shader_stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(*render_shader.vertex_shader())
                .name(desc.vertex_shader_entrypoint),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(*render_shader.fragment_shader())
                .name(desc.fragment_shader_entrypoint),
        ];
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&DYNAMIC_STATES);
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
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
            extent: *swapchain_extent,
        };
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&viewport))
            .scissors(std::slice::from_ref(&scissor));
        let rasterizer = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1f32)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);
        let multisampling = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD);
        let color_blending = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(std::slice::from_ref(&color_blend_attachment));
        let create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(*layout)
            .render_pass(*render_pass.render_pass())
            .subpass(0);
        let graphics_pipeline = match unsafe {
            device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                std::slice::from_ref(&create_info),
                None,
            )
        } {
            Ok(pipeline) => {
                if let Some(pipeline) = pipeline.first() {
                    Ok(*pipeline)
                } else {
                    Err(RtError::CreateGraphicsPipeline(None))
                }
            }
            Err((_, err)) => Err(RtError::CreateGraphicsPipeline(Some(err.into()))),
        }?;

        unsafe { render_shader.destroy(device) };

        Ok(graphics_pipeline)
    }

    fn create_compute_pipeline(
        device: &ash::Device,
        layout: &vk::PipelineLayout,
        desc: &ComputeShaderDescriptor,
    ) -> RtResult<vk::Pipeline> {
        let mut compute_shader = ComputeShader::new(device, desc)?;
        let shader_stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(*compute_shader.compute_shader())
            .name(desc.compute_shader_entrypoint);
        let create_info = vk::ComputePipelineCreateInfo::default()
            .layout(*layout)
            .stage(shader_stage);
        let compute_pipeline = match unsafe {
            device.create_compute_pipelines(
                vk::PipelineCache::null(),
                std::slice::from_ref(&create_info),
                None,
            )
        } {
            Ok(pipelines) => {
                if let Some(pipeline) = pipelines.first() {
                    Ok(*pipeline)
                } else {
                    Err(RtError::CreateComputePipeline(None))
                }
            }
            Err((_, err)) => Err(RtError::CreateComputePipeline(Some(err.into()))),
        }?;

        unsafe { compute_shader.destroy(device) };

        Ok(compute_pipeline)
    }
}
