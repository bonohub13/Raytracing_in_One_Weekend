use crate::core::{RtError, RtLabel, RtResult};
use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};
use wgpu::{Device, ShaderModule};

#[derive(Debug, Clone)]
pub struct RenderShaderDescriptor<'label> {
    pub vertex_path: PathBuf,
    pub vertex_entry_point: &'label str,
    pub vertex_label: RtLabel<'label>,
    pub fragment_path: PathBuf,
    pub fragment_entry_point: &'label str,
    pub fragment_label: RtLabel<'label>,
}

#[derive(Debug, Clone)]
pub struct ComputeShaderDescriptor<'label> {
    pub compute_path: PathBuf,
    pub compute_entry_point: &'label str,
    pub compute_label: RtLabel<'label>,
}

#[derive(Debug, Clone)]
pub struct RenderShader {
    vertex: ShaderModule,
    fragment: ShaderModule,
}

#[derive(Debug, Clone)]
pub struct ComputeShader {
    compute: ShaderModule,
}

impl RenderShader {
    pub fn new(device: &Device, desc: &RenderShaderDescriptor) -> RtResult<Self> {
        let vert_shader_code = read_shader(&desc.vertex_path)?;
        let frag_shader_code = read_shader(&desc.fragment_path)?;
        let vertex = unsafe {
            device.create_shader_module_passthrough(
                wgpu::wgt::CreateShaderModuleDescriptorPassthrough {
                    entry_point: desc.vertex_entry_point.to_string(),
                    label: Some(&desc.vertex_label.to_string()),
                    spirv: Some(wgpu::util::make_spirv_raw(&vert_shader_code)),
                    runtime_checks: wgpu::ShaderRuntimeChecks::checked(),
                    ..Default::default()
                },
            )
        };
        let fragment = unsafe {
            device.create_shader_module_passthrough(
                wgpu::wgt::CreateShaderModuleDescriptorPassthrough {
                    entry_point: desc.fragment_entry_point.to_string(),
                    label: Some(&desc.fragment_label.to_string()),
                    spirv: Some(wgpu::util::make_spirv_raw(&frag_shader_code)),
                    runtime_checks: wgpu::ShaderRuntimeChecks::checked(),
                    ..Default::default()
                },
            )
        };

        Ok(Self { vertex, fragment })
    }

    #[inline]
    pub fn vertex(&self) -> &ShaderModule {
        &self.vertex
    }

    #[inline]
    pub fn fragment(&self) -> &ShaderModule {
        &self.fragment
    }
}

impl ComputeShader {
    pub fn new(device: &Device, desc: &ComputeShaderDescriptor) -> RtResult<Self> {
        let compute_shader_code = read_shader(&desc.compute_path)?;
        let compute = unsafe {
            device.create_shader_module_passthrough(
                wgpu::wgt::CreateShaderModuleDescriptorPassthrough {
                    entry_point: desc.compute_entry_point.to_string(),
                    label: Some(&desc.compute_label.to_string()),
                    spirv: Some(wgpu::util::make_spirv_raw(&compute_shader_code)),
                    runtime_checks: wgpu::ShaderRuntimeChecks::checked(),
                    ..Default::default()
                },
            )
        };

        Ok(Self { compute })
    }

    #[inline]
    pub fn compute(&self) -> &ShaderModule {
        &self.compute
    }
}

fn read_shader(path: &Path) -> RtResult<Box<[u8]>> {
    const BYTE_ALIGNMENT: usize = 4;

    let mut reader = match OpenOptions::new().read(true).open(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(RtError::ReadFile(e.into())),
    }?;
    let mut buffer = vec![];

    match reader.read_to_end(&mut buffer) {
        Ok(_) => Ok(()),
        Err(e) => Err(RtError::ReadFile(e.into())),
    }?;
    if buffer.len().is_multiple_of(BYTE_ALIGNMENT) {
        buffer.extend(std::iter::repeat_n(0, buffer.len() % BYTE_ALIGNMENT));
    }

    Ok(Box::from_iter(buffer))
}
