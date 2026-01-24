use crate::{
    core::{RtLabel, Surface, renderer::Camera},
    utils,
};
use glam::Vec3;
use wgpu::{Buffer, Device, Sampler, TextureView, util::DeviceExt};

#[derive(Debug, Clone)]
pub struct PathTracerBufferDescriptor<'data, 'label> {
    pub label: Option<&'label str>,
    pub camera: Camera,
    pub objects: &'data [HitObject],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HitObject {
    center: Vec3,
    radius: f32,
    _pad: [f32; 3],
    id: u32,
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

impl HitObject {
    const SPHERE: u32 = 1;

    pub const fn create_sphere(center: Vec3, radius: f32) -> Self {
        Self {
            center,
            radius,
            id: Self::SPHERE,
            _pad: [0f32; 3],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathTracerBuffer {
    camera_buffer: Buffer,
    object_buffer: Buffer,
    rand_data_buffer: Buffer,
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
        let rand_data_buffer = desc
            .camera
            .create_rand_data_buffer(device, Some("Rand Storage"));
        let object_buffer = Self::create_object_buffer(device, desc);
        let texture = Self::create_texture(device, surface, desc);
        let sampler = Self::create_sampler(device, desc);

        Self {
            camera_buffer,
            object_buffer,
            rand_data_buffer,
            texture,
            sampler,
        }
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
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
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
                    resource: self.object_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.rand_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
            ],
        });

        BindGroup { layout, bind_group }
    }

    fn create_object_buffer(device: &Device, desc: &PathTracerBufferDescriptor) -> Buffer {
        let label = if let Some(label) = desc.label {
            format!("{label} Storage")
        } else {
            "Storage".to_string()
        };
        let contents = unsafe { utils::data_into_bytes(desc.objects) };

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&RtLabel::Buffer(Some(&label)).to_string()),
            usage: wgpu::BufferUsages::STORAGE,
            contents,
        })
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

impl Drop for PathTracerBuffer {
    fn drop(&mut self) {
        self.camera_buffer.destroy();
        self.object_buffer.destroy();
    }
}
