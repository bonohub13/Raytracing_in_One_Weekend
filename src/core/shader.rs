use crate::core::error::{RtError, RtResult};
use ash::{util, vk};
use std::{
    ffi::CStr,
    fs::OpenOptions,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct RenderShaderDescriptor<'entry> {
    pub vertex_shader_path: PathBuf,
    pub vertex_shader_entrypoint: &'entry CStr,
    pub fragment_shader_path: PathBuf,
    pub fragment_shader_entrypoint: &'entry CStr,
}

#[derive(Debug, Clone)]
pub struct ComputeShaderDescriptor<'entry> {
    pub compute_shader_path: PathBuf,
    pub compute_shader_entrypoint: &'entry CStr,
}

pub struct RenderShader {
    vertex: vk::ShaderModule,
    fragment: vk::ShaderModule,
}

pub struct ComputeShader {
    compute: vk::ShaderModule,
}

impl RenderShader {
    pub fn new(device: &ash::Device, desc: &RenderShaderDescriptor) -> RtResult<Self> {
        let vertex = create_shader(device, &desc.vertex_shader_path)?;
        let fragment = create_shader(device, &desc.fragment_shader_path)?;

        Ok(Self { vertex, fragment })
    }

    #[inline]
    pub fn vertex_shader(&self) -> &vk::ShaderModule {
        &self.vertex
    }

    #[inline]
    pub fn fragment_shader(&self) -> &vk::ShaderModule {
        &self.fragment
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_shader_module(self.vertex, None);
            device.destroy_shader_module(self.fragment, None);
        }
    }
}

impl ComputeShader {
    pub fn new(device: &ash::Device, desc: &ComputeShaderDescriptor) -> RtResult<Self> {
        let compute = create_shader(device, &desc.compute_shader_path)?;

        Ok(Self { compute })
    }

    #[inline]
    pub const fn compute_shader(&self) -> &vk::ShaderModule {
        &self.compute
    }

    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        unsafe { device.destroy_shader_module(self.compute, None) }
    }
}

fn create_shader(device: &ash::Device, path: &Path) -> RtResult<vk::ShaderModule> {
    let mut reader = match OpenOptions::new().read(true).open(path) {
        Ok(file) => Ok(file),
        Err(err) => Err(RtError::ReadFile(err.into())),
    }?;
    let code = match util::read_spv(&mut reader) {
        Ok(spv_code) => Ok(spv_code),
        Err(err) => Err(RtError::ReadFile(err.into())),
    }?;
    let create_info = vk::ShaderModuleCreateInfo::default().code(&code);

    match unsafe { device.create_shader_module(&create_info, None) } {
        Ok(shader_module) => Ok(shader_module),
        Err(err) => Err(RtError::CreateShaderModule(err.into())),
    }
}
