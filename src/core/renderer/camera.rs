use crate::{core::RtLabel, utils};
use glam::{Vec2, Vec3A};
use wgpu::{Buffer, Device, util::DeviceExt};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    pub resolution: Vec2,
    samples_per_pixel: u32,
    max_depth: u32,
    center: Vec3A,
    pixel00_loc: Vec3A,
    pixel_delta_u: Vec3A,
    pixel_delta_v: Vec3A,
    pixel_samples_scale: Vec3A,
}

impl Camera {
    pub fn new(resolution: Vec2, samples_per_pixel: u32, max_depth: u32) -> Self {
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
            samples_per_pixel,
            max_depth,
            center: CAMERA_CENTER,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: glam::vec3a(1f32 / samples_per_pixel as f32, 0f32, 0f32),
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
            usage: wgpu::BufferUsages::UNIFORM,
        })
    }

    pub fn create_rand_data_buffer(&self, device: &Device, label: Option<&str>) -> Buffer {
        let rand_data: Vec<glam::Vec3A> = (0..(self.samples_per_pixel * self.samples_per_pixel))
            .map(|_| glam::vec3a(utils::random(), utils::random(), utils::random()))
            .collect();
        let contents = unsafe { utils::data_into_bytes(&rand_data) };

        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&RtLabel::Buffer(label).to_string()),
            contents,
            usage: wgpu::BufferUsages::STORAGE,
        })
    }
}
