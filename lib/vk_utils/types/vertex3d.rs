/// A simple 3 dimentional vertex
/// pos: 3D Position with w as perspective
/// color: RGB value (max 1.0), alpha is currently set to 1.0 statically
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex3D {
    pub pos: [f32; 4],
    pub color: [f32; 3],
}

impl Vertex3D {
    /// Creates a new instance of Vertex3D object
    ///
    /// Params:
    ///     pos: 3D coordinates + w for perspective
    ///     color: RGB value (max 1.0)
    ///
    /// Return:
    ///     Vertex3D
    #[inline]
    pub fn new(pos: [f32; 4], color: [f32; 3]) -> Self {
        Self { pos, color }
    }
}

impl PartialEq for Vertex3D {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.color == other.color
    }
}

impl crate::attributes::Vertex for Vertex3D {
    fn get_binding_description() -> Vec<ash::vk::VertexInputBindingDescription> {
        use ash::vk;
        use std::mem::size_of;

        vec![vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()]
    }

    fn get_attribute_descriptions() -> Vec<ash::vk::VertexInputAttributeDescription> {
        use ash::vk;
        use memoffset::offset_of;

        vec![
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, pos) as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Self, color) as u32)
                .build(),
        ]
    }
}

impl crate::attributes::Pipeline for Vertex3D {
    fn create_graphics_pipeline(
        device: &ash::Device,
        msaa_samples: ash::vk::SampleCountFlags,
        swapchain_extent: ash::vk::Extent2D,
        render_pass: ash::vk::RenderPass,
        descriptor_set_layout: ash::vk::DescriptorSetLayout,
        vertex_shader_path: &std::path::Path,
        fragment_shader_path: &std::path::Path,
    ) -> Result<(ash::vk::Pipeline, ash::vk::PipelineLayout), String> {
        use crate::{
            attributes::Vertex,
            tools::{create_shader_module, read_shader_code},
        };
        use ash::vk;
        use std::ffi::CString;

        let vert_shader_code_res = read_shader_code(vertex_shader_path);
        let frag_shader_code_res = read_shader_code(fragment_shader_path);

        if vert_shader_code_res.is_err() {
            return Err(vert_shader_code_res.err().unwrap());
        } else if frag_shader_code_res.is_err() {
            return Err(frag_shader_code_res.err().unwrap());
        }

        let vert_shader_code = vert_shader_code_res.unwrap();
        let frag_shader_code = frag_shader_code_res.unwrap();

        let vert_shader_module_res = create_shader_module(device, &vert_shader_code);
        let frag_shader_module_res = create_shader_module(device, &frag_shader_code);

        if vert_shader_module_res.is_err() {
            return Err(vert_shader_module_res.err().unwrap());
        } else if frag_shader_module_res.is_err() {
            return Err(frag_shader_module_res.err().unwrap());
        }

        let vert_shader_module = vert_shader_module_res.unwrap();
        let frag_shader_module = frag_shader_module_res.unwrap();

        let result = CString::new("main");

        if result.is_err() {
            return Err(String::from("failed to convert \"main\" to CString"));
        }

        let main_function_name = result.unwrap();

        let shader_stages = [
            // Vertex shader
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_shader_module)
                .name(&main_function_name)
                .build(),
            // Fragment shader
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_shader_module)
                .name(&main_function_name)
                .build(),
        ];

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state =
            vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_states);

        let binding_descriptions = Self::get_binding_description();
        let attribute_descriptions = Self::get_attribute_descriptions();

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewports = [vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(swapchain_extent.width as f32)
            .height(swapchain_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build()];

        let scissors = [vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(swapchain_extent)
            .build()];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(msaa_samples)
            .min_sample_shading(0.2)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false);

        let color_blend_attachments = [vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD)
            .build()];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let set_layouts = [descriptor_set_layout];

        let pipeline_layout_info =
            vk::PipelineLayoutCreateInfo::builder().set_layouts(&set_layouts);
        let result = unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) };

        if result.is_err() {
            return Err(String::from("failed to create pipeline layout!"));
        }

        let pipeline_layout = result.unwrap();

        let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false);

        let pipeline_infos = [vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .depth_stencil_state(&depth_stencil)
            .build()];
        let result = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_infos, None)
        };

        if result.is_err() {
            return Err(String::from("failed to create graphics pipeline!"));
        }

        let graphics_pipeline = result.unwrap()[0];

        unsafe {
            device.destroy_shader_module(vert_shader_module, None);
            device.destroy_shader_module(frag_shader_module, None);
        }

        Ok((graphics_pipeline, pipeline_layout))
    }
}
