pub fn create_pipeline_layout(
    device: &ash::Device,
    descriptor_set_layout: ash::vk::DescriptorSetLayout,
) -> Result<ash::vk::PipelineLayout, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating pipeline layout");

    let layouts = if descriptor_set_layout == ash::vk::DescriptorSetLayout::null() {
        vec![]
    } else {
        vec![descriptor_set_layout]
    };
    let create_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&layouts)
        .build();

    let layout_sg = {
        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&create_info, None)
                .map_err(|_| String::from("failed to create pipeline layout"))?
        };

        guard(pipeline_layout, |layout| {
            log::warn!("pipeline layout scopeguard");

            unsafe {
                device.destroy_pipeline_layout(layout, None);
            }
        })
    };

    log::info!("created pipeline layout");

    Ok(ScopeGuard::into_inner(layout_sg))
}

pub fn create_compute_pipeline(
    device: &ash::Device,
    pipeline_layout: ash::vk::PipelineLayout,
    shader: &crate::VkShaderModule,
) -> Result<ash::vk::Pipeline, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};
    use std::ffi::CStr;

    log::info!("creating compute pipeline");

    let main_fn = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };
    let shader_info = vk::PipelineShaderStageCreateInfo::builder()
        .module(shader.shader_module)
        .name(main_fn)
        .stage(shader.stage)
        .build();
    let create_info = vk::ComputePipelineCreateInfo::builder()
        .stage(shader_info)
        .layout(pipeline_layout)
        .build();

    let compute_pipeline_sg = {
        let pipeline = unsafe {
            device
                .create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None)
                .map_err(|_| String::from("failed to create compute pipeline"))?
        }[0];

        guard(pipeline, |pipeline| {
            log::warn!("compute pipeline scopeguard");

            unsafe {
                device.destroy_pipeline(pipeline, None);
            }
        })
    };

    log::info!("created compute pipeline");

    Ok(ScopeGuard::into_inner(compute_pipeline_sg))
}

pub fn create_graphics_pipeline(
    device: &ash::Device,
    render_pass: ash::vk::RenderPass,
    pipeline_layout: ash::vk::PipelineLayout,
    extent: ash::vk::Extent2D,
    shaders: &Vec<crate::VkShaderModule>,
    binding_description: ash::vk::VertexInputBindingDescription,
    attribute_descriptions: &Vec<ash::vk::VertexInputAttributeDescription>,
) -> Result<ash::vk::Pipeline, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};
    use std::ffi::CStr;

    log::info!("creating graphics pipeline");

    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .build();

    let raster = vk::PipelineRasterizationStateCreateInfo::builder()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::CLOCKWISE)
        .line_width(1.0)
        .build();

    let blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
        .color_write_mask(vk::ColorComponentFlags::RGBA)
        .build();

    let blend = vk::PipelineColorBlendStateCreateInfo::builder()
        .attachments(&[blend_attachment])
        .logic_op_enable(false)
        .logic_op(vk::LogicOp::COPY)
        .build();

    let viewport = vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(extent.width as f32)
        .height(extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)
        .build();
    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D { x: 0, y: 0 })
        .extent(extent)
        .build();
    let viewport_create_info = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(&[viewport])
        .scissors(&[scissor])
        .build();

    let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(false)
        .depth_write_enable(false)
        .depth_compare_op(vk::CompareOp::GREATER)
        .build();

    let multisampling_create_info = vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build();

    let main_func_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };
    let shader_stages: Vec<ash::vk::PipelineShaderStageCreateInfo> = shaders
        .iter()
        .map(|shader_module| {
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(shader_module.stage)
                .module(shader_module.shader_module)
                .name(main_func_name)
                .build()
        })
        .collect();

    let vertex_create_info = vk::PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(&[binding_description])
        .vertex_attribute_descriptions(attribute_descriptions)
        .build();

    let create_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_create_info)
        .input_assembly_state(&input_assembly)
        .rasterization_state(&raster)
        .color_blend_state(&blend)
        .multisample_state(&multisampling_create_info)
        .viewport_state(&viewport_create_info)
        .render_pass(render_pass)
        .layout(pipeline_layout)
        .build();

    let pipeline_sg = {
        let graphics_pipeline = unsafe {
            device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None)
                .map_err(|_| String::from("failed to create graphics pipeline"))?
        }[0];

        guard(graphics_pipeline, |pipeline| {
            log::warn!("graphics pipeline scopeguard");

            unsafe {
                device.destroy_pipeline(pipeline, None);
            }
        })
    };

    log::info!("created graphics pipeline");

    Ok(ScopeGuard::into_inner(pipeline_sg))
}
