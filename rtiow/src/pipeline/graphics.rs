use crate::{
    Device, RtErr, RtError, VkState,
    pipeline::{Encoder, PipelineLayout, ShaderModule},
};
use ash::vk;
use std::{ffi::CStr, path::Path, sync::Arc};

pub struct GraphicsPipeline {
    layout: vk::PipelineLayout,
    device: Arc<Device>,
}

impl GraphicsPipeline {
    const VERT_SHADER_PATH: &str = "shaders/spv/vertex.spv";
    const FRAG_SHADER_PATH: &str = "shaders/spv/fragment.spv";
    const DYNAMIC_STATES: [vk::DynamicState; 2] =
        [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

    pub fn new(state: &VkState, layout: &PipelineLayout) -> RtErr<Self> {
        let vert_shader =
            ShaderModule::new(state.device.clone(), Path::new(Self::VERT_SHADER_PATH))?;
        let frag_shader =
            ShaderModule::new(state.device.clone(), Path::new(Self::FRAG_SHADER_PATH))?;

        Ok(Self {
            device: state.device.clone(),
            layout: layout.layout,
        })
    }

    pub fn render(
        &self,
        state: &VkState,
        encoder: &Encoder,
        image_index: usize,
        current_frame: usize,
    ) {
        static CLEAR_VALUE: vk::ClearValue = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0f32, 0f32, 0f32, 1f32],
            },
        };
        static OFFSET: vk::Offset2D = vk::Offset2D { x: 0, y: 0 };

        let device = self.device.device();
        let command_buffer = encoder.command_buffers()[current_frame];
        let color_attachment = vk::RenderingAttachmentInfo::default()
            .image_view(state.swapchain.image_views()[image_index])
            .image_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(CLEAR_VALUE);
        let rendering_info = vk::RenderingInfo::default()
            .render_area(vk::Rect2D {
                offset: OFFSET,
                extent: *state.swapchain.extent(),
            })
            .layer_count(1)
            .color_attachments(std::slice::from_ref(&color_attachment));

        unsafe {
            device.cmd_begin_rendering(command_buffer, &rendering_info);

            // Draw calls

            device.cmd_end_rendering(command_buffer);
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

    fn create_pipeline(device: Arc<Device>, extent: &vk::Extent2D) -> RtErr<vk::Pipeline> {
        static BLEND_CONSTANTS: [f32; 4] = [0f32, 0f32, 0f32, 1f32];
        static COLOR_FORMATS: [vk::Format; 1] = [vk::Format::B8G8R8A8_SRGB];

        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&Self::DYNAMIC_STATES);
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(std::slice::from_ref(&Self::viewport(extent)))
            .scissors(std::slice::from_ref(&Self::scissor(extent)));
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
            .attachments(std::slice::from_ref(&color_blend_attachment))
            .blend_constants(BLEND_CONSTANTS);
        let mut rendering_pipeline_info = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&COLOR_FORMATS)
            .depth_attachment_format(vk::Format::D32_SFLOAT);

        let create_info =
            vk::GraphicsPipelineCreateInfo::default().push_next(&mut rendering_pipeline_info);

        todo!()
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {}
}
