use crate::{core::RtLabel, utils};
use glam::{Vec2, Vec3A};
use wgpu::{Buffer, Device, util::DeviceExt};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    pub resolution: Vec2,
    pub _pad0: [f32; 2],

    pub center: Vec3A,
    pub pixel00_loc: Vec3A,
    pub pixel_delta_u: Vec3A,
    pub pixel_delta_v: Vec3A,
}

impl Camera {
    pub fn new(resolution: Vec2) -> Self {
        const FOCAL_LENGTH: f32 = 1f32;
        const VIEWPORT_HEIGHT: f32 = 2f32;
        const VIEWPORT_V: Vec3A = glam::vec3a(0f32, -VIEWPORT_HEIGHT, 0f32);
        const CAMERA_CENTER: glam::Vec3A = glam::Vec3A::ZERO;

        let viewport_width = VIEWPORT_HEIGHT * (resolution.x / resolution.y);
        let viewport_u = glam::vec3a(viewport_width, 0f32, 0f32);

        let pixel_delta_u = viewport_u / resolution.x;
        let pixel_delta_v = VIEWPORT_V / resolution.y;

        let viewport_upper_left = CAMERA_CENTER
            - (glam::vec3a(0f32, 0f32, FOCAL_LENGTH) + viewport_u / 2f32 + VIEWPORT_V / 2f32);
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            resolution,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            ..Default::default()
        }
    }
    pub fn create_uniform_buffer(&self, device: &Device, label: Option<&str>) -> Buffer {
        let label = if label.is_some() {
            label
        } else {
            Some("Camera Uniform")
        };
        let contents = unsafe { utils::data_into_bytes(std::slice::from_ref(self)) };

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&RtLabel::Buffer(label).to_string()),
            contents,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }
}
