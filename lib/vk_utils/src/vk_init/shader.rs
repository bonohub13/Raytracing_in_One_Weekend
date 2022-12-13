pub fn create_shader_module(
    device: &ash::Device,
    filename: &str,
    stage: ash::vk::ShaderStageFlags,
) -> Result<crate::VkShaderModule, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};
    use std::fs::File;
    use std::path::Path;

    log::info!("creating VkShaderModule");

    let mut spv_file = File::open(Path::new(filename))
        .map_err(|_| format!("failed to find spv file at {:?}", filename))?;
    let shader_code = ash::util::read_spv(&mut spv_file)
        .map_err(|_| format!("failed to read shader spv file ({})", filename))?;

    let create_info = vk::ShaderModuleCreateInfo::builder()
        .code(&shader_code)
        .build();

    let shader_sg = {
        let shader_module = unsafe {
            device
                .create_shader_module(&create_info, None)
                .map_err(|_| String::from("failed to create shader module"))?
        };

        guard(shader_module, |module| {
            log::warn!("shader module scopeguard");

            unsafe {
                device.destroy_shader_module(module, None);
            }
        })
    };

    log::info!("created VkShaderModule");

    Ok(crate::VkShaderModule {
        shader_module: ScopeGuard::into_inner(shader_sg),
        stage,
    })
}
