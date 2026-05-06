use crate::{Device, RtErr, RtError};
use ash::vk;
use std::{fs, io::Read, path::Path, sync::Arc};

pub struct ShaderModule {
    code: Box<[u32]>,
    shader: vk::ShaderModule,
    device: Arc<Device>,
}

impl ShaderModule {
    pub fn new(device: Arc<Device>, path: &Path) -> RtErr<Self> {
        let code = read_shader(path)?;
        let shader = Self::create_shader_module(device.clone(), &code)?;

        Ok(Self {
            device,
            code,
            shader,
        })
    }

    #[inline]
    pub(crate) fn shader(&self) -> vk::ShaderModule {
        self.shader
    }

    fn create_shader_module(device: Arc<Device>, code: &[u32]) -> RtErr<vk::ShaderModule> {
        let create_info = vk::ShaderModuleCreateInfo::default().code(code);

        unsafe { device.device().create_shader_module(&create_info, None) }
            .map_err(|err| RtError::CreateShaderModule(err.into()))
    }
}

fn read_shader(path: &Path) -> RtErr<Box<[u32]>> {
    const BYTE_ALIGNMENT: usize = 4;
    const BYTE_ALIGNMENT_MASK: usize = 0b11;

    let mut reader = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|err| RtError::ReadFile(err.into()))?;
    let file_size = reader
        .metadata()
        .map_err(|err| RtError::ReadFile(err.into()))?
        .len() as usize;
    let padding_size = BYTE_ALIGNMENT - (file_size & BYTE_ALIGNMENT_MASK);
    let mut buffer: Vec<u8> = Vec::with_capacity(file_size + padding_size);

    reader
        .read_to_end(&mut buffer)
        .map_err(|err| RtError::ReadFile(err.into()))?;
    if !buffer.len().is_multiple_of(BYTE_ALIGNMENT) {
        buffer.extend(std::iter::repeat_n(0, padding_size))
    }

    Ok(Box::from(bytemuck::cast_slice(&buffer)))
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.device
                .device()
                .destroy_shader_module(self.shader, None)
        }
    }
}
