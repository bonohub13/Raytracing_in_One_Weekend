use crate::{
    core::{RtLabel, Surface, renderer::Camera},
    utils,
};
use wgpu::{Buffer, Device, Queue, Sampler, TextureView};

#[derive(Debug, Clone)]
pub struct PathTracerBufferDescriptor<'label> {
    pub label: Option<&'label str>,
    pub camera: Camera,
}

#[derive(Debug, Clone)]
pub struct BindGroup {
    layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

#[derive(Debug, Clone)]
struct Texture {
    _texture: wgpu::Texture,
    view: TextureView,
}

#[derive(Debug, Clone)]
pub struct PathTracerBuffer {
    camera_buffer: Buffer,
    texture: Texture,
    sampler: Sampler,
}

impl BindGroup {
    #[inline]
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    #[inline]
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

impl PathTracerBuffer {
    pub fn new(device: &Device, surface: &Surface, desc: &PathTracerBufferDescriptor) -> Self {
        let camera_buffer = desc.camera.create_uniform_buffer(device, None);
        let texture = Self::create_texture(device, surface, desc);
        let sampler = Self::create_sampler(device, desc);

        Self {
            camera_buffer,
            texture,
            sampler,
        }
    }

    pub fn update_camera(&mut self, queue: &Queue, camera: &Camera) {
        let contents = unsafe { utils::data_into_bytes(std::slice::from_ref(camera)) };

        queue.write_buffer(&self.camera_buffer, 0, contents);
    }

    pub fn create_render_bind_group(&self, device: &Device, label: Option<&str>) -> BindGroup {
        let label = if label.is_some() {
            label
        } else {
            Some("Render")
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&RtLabel::BindGroupLayout(label).to_string()),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&RtLabel::BindGroup(label).to_string()),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        BindGroup { layout, bind_group }
    }

    pub fn create_compute_bind_group(&self, device: &Device, label: Option<&str>) -> BindGroup {
        let label = if label.is_some() {
            label
        } else {
            Some("Compute")
        };
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&RtLabel::BindGroupLayout(label).to_string()),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba16Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&RtLabel::BindGroup(label).to_string()),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
            ],
        });

        BindGroup { layout, bind_group }
    }

    #[inline]
    fn create_texture(
        device: &Device,
        surface: &Surface,
        desc: &PathTracerBufferDescriptor,
    ) -> Texture {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&RtLabel::Texture(desc.label).to_string()),
            size: wgpu::Extent3d {
                width: surface.config().width,
                height: surface.config().height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&Default::default());

        Texture {
            _texture: texture,
            view,
        }
    }

    #[inline]
    fn create_sampler(device: &Device, desc: &PathTracerBufferDescriptor) -> Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(&RtLabel::Sampler(desc.label).to_string()),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        })
    }
}
