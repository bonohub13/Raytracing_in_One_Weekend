use crate::core::{
    RtResult,
    debug::DebugUtilsMessenger,
    device::{self, Device},
    instance::{self, Instance},
    renderer::{self, PathTracer},
    surface::{self, Surface},
};
use ash::vk;
use std::{ffi::CStr, sync::Arc};
use winit::window::Window;

pub struct StateDescriptor {
    pub window: Arc<Window>,
}

#[allow(dead_code)]
pub struct State {
    renderer: PathTracer,
    device: Arc<Device>,
    surface: Arc<Surface>,
    debug_messenger: Option<DebugUtilsMessenger>,
    instance: Arc<Instance>,
    window: Arc<Window>,
}

impl State {
    const APPLICATION_NAME: &CStr = c"Ray Tracing in One Weekend (Vulkan)";
    const APPLICATION_VERSION: u32 = vk::make_api_version(0, 0, 1, 0);

    pub fn new(desc: &StateDescriptor) -> RtResult<Self> {
        let instance = Arc::new(Instance::new(&instance::InstanceDescriptor {
            application_name: Self::APPLICATION_NAME,
            application_version: Self::APPLICATION_VERSION,
            window: desc.window.clone(),
        })?);
        #[cfg(not(debug_assertions))]
        let debug_messenger = None;
        #[cfg(debug_assertions)]
        let debug_messenger = Some(DebugUtilsMessenger::new(instance.clone())?);
        let surface = Arc::new(Surface::new(&surface::SurfaceDescriptor {
            window: desc.window.clone(),
            instance: instance.clone(),
        })?);
        let device = Arc::new(Device::new(&device::DeviceDescriptor {
            instance: instance.clone(),
            surface: surface.clone(),
        })?);
        let renderer = PathTracer::new(&renderer::PathTracerDescriptor {
            instance: instance.clone(),
            device: device.clone(),
            surface: surface.clone(),
        })?;

        Ok(State {
            window: desc.window.clone(),
            instance,
            debug_messenger,
            surface,
            device,
            renderer,
        })
    }

    pub fn resize(&mut self) -> RtResult<()> {
        self.renderer.resize()
    }

    pub fn draw_frame(&mut self) -> RtResult<()> {
        self.renderer.render_frame()
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if let Err(err) = self.device.device_wait_idle() {
            eprintln!("{err:?}")
        }
        if let Some(debug_messenger) = self.debug_messenger.take() {
            drop(debug_messenger);
        }
    }
}
