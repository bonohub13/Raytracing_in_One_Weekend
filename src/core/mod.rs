mod error;
mod label;
pub mod renderer;

use crate::core::renderer::{HitObject, PathTracer};
pub use error::{RtError, RtResult};
pub use label::RtLabel;

use std::{path::PathBuf, sync::Arc};
use wgpu::{Adapter, Device, DeviceType, Instance, Queue, SurfaceConfiguration, SurfaceTexture};
use winit::window::Window;

#[derive(Debug, Clone)]
pub struct StateDescriptor<'obj> {
    pub window: Arc<Window>,
    pub objects: &'obj [HitObject],
}

#[derive(Debug)]
pub struct Surface<'window> {
    surface: wgpu::Surface<'window>,
    config: SurfaceConfiguration,
    is_configured: bool,
}

#[derive(Debug)]
pub struct State<'window> {
    window: Arc<Window>,
    surface: Surface<'window>,
    device: Device,
    queue: Queue,
    path_tracer: PathTracer,
}

impl<'window> Surface<'window> {
    fn resize(&mut self, device: &Device, width: u32, height: u32) {
        if (width > 0) && (height > 0) {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(device, &self.config);
            self.is_configured = true;
        }
    }

    fn get_current_texture(&mut self) -> RtResult<SurfaceTexture> {
        match self.surface.get_current_texture() {
            Ok(texture) => Ok(texture),
            Err(e) => Err(RtError::GetCurrentTexture(e)),
        }
    }

    #[inline]
    pub fn config(&self) -> &SurfaceConfiguration {
        &self.config
    }
}

impl<'window> State<'window> {
    pub async fn new<'data>(desc: &StateDescriptor<'data>) -> RtResult<Self> {
        let instance = {
            let desc = wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                flags: wgpu::InstanceFlags::default(),
                ..Default::default()
            };

            wgpu::Instance::new(&desc)
        };
        let (surface, adapter) = Self::create_surface(&instance, desc.window.clone()).await?;
        let (device, queue) = Self::request_device(&adapter).await?;
        let path_tracer = Self::create_pipeline(&device, &surface, desc)?;

        Ok(Self {
            window: desc.window.clone(),
            surface,
            device,
            queue,
            path_tracer,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(&self.device, width, height);
    }

    pub fn update(&mut self) {
        self.path_tracer.update(&self.queue);
    }

    pub fn render(&mut self) -> RtResult<()> {
        self.window.request_redraw();
        if !self.surface.is_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&RtLabel::RenderEncoder(None).to_string()),
            });
        {
            let mut compute_pass = encoder.begin_compute_pass(&Default::default());

            self.path_tracer.compute(&mut compute_pass);
        }
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&RtLabel::RenderPass(None).to_string()),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            self.path_tracer.render(&mut render_pass);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    #[inline]
    async fn create_surface(
        instance: &Instance,
        window: Arc<Window>,
    ) -> RtResult<(Surface<'window>, Adapter)> {
        let size = window.inner_size();
        let surface = match instance.create_surface(window.clone()) {
            Ok(surface) => Ok(surface),
            Err(e) => Err(RtError::CreateSurface(e)),
        }?;
        let adapter = Self::request_adapter(instance, &surface).await?;
        let capabilties = surface.get_capabilities(&adapter);
        let format = capabilties
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilties.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: capabilties.present_modes[0],
            alpha_mode: capabilties.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok((
            Surface {
                surface,
                config,
                is_configured: false,
            },
            adapter,
        ))
    }

    #[inline]
    async fn request_adapter(
        instance: &Instance,
        surface: &wgpu::Surface<'window>,
    ) -> RtResult<wgpu::Adapter> {
        let adapters = {
            let mut adapters = instance.enumerate_adapters(wgpu::Backends::VULKAN).await;

            adapters
                .sort_by_key(|adapter| Self::device_type_priority(adapter.get_info().device_type));

            adapters
        };
        let adapter = adapters.iter().find(|adapter| {
            let features = adapter.features();

            adapter.is_surface_supported(surface)
                && features
                    .features_wgpu
                    .contains(wgpu::FeaturesWGPU::EXPERIMENTAL_PASSTHROUGH_SHADERS)
        });

        match adapter {
            Some(adapter) => Ok(adapter.clone()),
            None => Err(RtError::RequestAdapter),
        }
    }

    #[inline]
    async fn request_device(adapter: &Adapter) -> RtResult<(Device, Queue)> {
        let desc = wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::EXPERIMENTAL_PASSTHROUGH_SHADERS,
            experimental_features: unsafe { wgpu::ExperimentalFeatures::enabled() },
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        };

        match adapter.request_device(&desc).await {
            Ok(device) => Ok(device),
            Err(e) => Err(RtError::RequestDevice(e)),
        }
    }

    #[inline]
    fn create_pipeline(
        device: &Device,
        surface: &Surface,
        desc: &StateDescriptor,
    ) -> RtResult<PathTracer> {
        let config = surface.config();
        let resolution = glam::vec2(config.width as f32, config.height as f32);

        let desc = renderer::PathTracerDescriptor {
            render_pipeline_label: RtLabel::RenderPipeline(Some("Path Tracer")),
            render_shader_desc: renderer::RenderShaderDescriptor {
                vertex_path: PathBuf::from("shaders/spv/vs_main.spv"),
                vertex_entry_point: "vs_main",
                vertex_label: RtLabel::Shader(Some("Vertex")),
                fragment_path: PathBuf::from("shaders/spv/fs_main.spv"),
                fragment_entry_point: "fs_main",
                fragment_label: RtLabel::Shader(Some("Fragment")),
            },
            compute_pipeline_label: RtLabel::ComputePipeline(Some("Path Tracer")),
            compute_shader_desc: renderer::ComputeShaderDescriptor {
                compute_path: PathBuf::from("shaders/spv/path_tracer.spv"),
                compute_entry_point: "main",
                compute_label: RtLabel::Shader(Some("Path Tracer")),
            },
            buffer_desc: renderer::PathTracerBufferDescriptor {
                label: Some("Path Tracer"),
                camera: renderer::Camera::new(&renderer::CameraDescriptor {
                    resolution,
                    samples_per_pixel: 500,
                    max_depth: 50,
                    vfov: 20f32,
                    look_from: glam::vec3a(13f32, 2f32, 3f32),
                    look_at: glam::Vec3A::ZERO,
                    vup: glam::vec3a(0f32, 1f32, 0f32),
                    defocus_angle: 0.6,
                    focus_distance: 10f32,
                }),
                objects: desc.objects,
            },
        };

        PathTracer::new(device, surface, &desc)
    }

    fn device_type_priority(device_type: DeviceType) -> u32 {
        match device_type {
            DeviceType::DiscreteGpu => 0,
            DeviceType::IntegratedGpu => 1,
            DeviceType::VirtualGpu => 2,
            DeviceType::Cpu => 3,
            DeviceType::Other => 4,
        }
    }
}

impl Drop for State<'_> {
    fn drop(&mut self) {
        self.device.destroy();
    }
}
