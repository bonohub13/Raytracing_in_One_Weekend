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
