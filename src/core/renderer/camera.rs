use crate::{core::RtLabel, utils};
use glam::{Vec2, Vec3A, Vec4};
use wgpu::{Buffer, Device, util::DeviceExt};

#[derive(Debug, Clone, Copy)]
pub struct CameraDescriptor {
    pub resolution: Vec2,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub look_from: Vec3A,
    pub look_at: Vec3A,
    pub vup: Vec3A,
    pub vfov: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
}

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
    defocus_disk_u: Vec4,
    defocus_disk_v: Vec4,
}

impl Default for CameraDescriptor {
    fn default() -> Self {
        Self {
            resolution: glam::vec2(1280f32, 800f32),
            samples_per_pixel: 10,
            max_depth: 10,
            look_from: glam::Vec3A::ZERO,
            look_at: glam::vec3a(0f32, 0f32, -1f32),
            vup: glam::vec3a(0f32, 1f32, 0f32),
            vfov: 90f32,
            defocus_angle: 0f32,
            focus_distance: 10f32,
        }
    }
}

impl Camera {
    pub fn new(desc: &CameraDescriptor) -> Self {
        let center = desc.look_from;
        let theta = utils::degrees_to_radians(desc.vfov);
        let height = (0.5 * theta).tan();
        let viewport_height = 2f32 * height * desc.focus_distance;
        let viewport_width = viewport_height * (desc.resolution.x / desc.resolution.y);
        let w = (desc.look_from - desc.look_at).normalize();
        let u = desc.vup.cross(w).normalize();
        let v = w.cross(u);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / desc.resolution.x;
        let pixel_delta_v = viewport_v / desc.resolution.y;

        let viewport_upper_left =
            center - ((desc.focus_distance * w) + (viewport_u * 0.5) + (viewport_v * 0.5));
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            desc.focus_distance * utils::degrees_to_radians(desc.defocus_angle * 0.5).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            resolution: desc.resolution,
            samples_per_pixel: desc.samples_per_pixel,
            max_depth: desc.max_depth,
            center: desc.look_from,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u: glam::vec4(
                defocus_disk_u.x,
                defocus_disk_u.y,
                defocus_disk_u.z,
                desc.defocus_angle,
            ),
            defocus_disk_v: glam::vec4(
                defocus_disk_v.x,
                defocus_disk_v.y,
                defocus_disk_v.z,
                desc.defocus_angle,
            ),
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
}
