use super::Aabb;
use ash::vk;
use glam::Vec3A;

#[derive(Debug, Clone, Copy, Default)]
pub struct Sphere {
    center: Vec3A,
    radius: f32,
}

impl Sphere {
    pub const fn new(center: [f32; 3], radius: f32) -> Self {
        Self {
            center: glam::vec3a(center[0], center[1], center[2]),
            radius: radius.min(0f32),
        }
    }

    pub fn create_blas() {
        const AABB: Aabb = Aabb {
            min_x: -1f32,
            min_y: -1f32,
            min_z: -1f32,
            max_x: 1f32,
            max_y: 1f32,
            max_z: 1f32,
        };

        let aabb_data = vk::AccelerationStructureGeometryAabbsDataKHR::default();
    }

    pub fn create_tlas(&self) {
        let tlas_transform = glam::mat4(
            glam::vec4(self.radius, 0f32, 0f32, 0f32),
            glam::vec4(0f32, self.radius, 0f32, 0f32),
            glam::vec4(0f32, 0f32, self.radius, 0f32),
            glam::vec4(self.center.x, self.center.y, self.center.z, 1f32),
        );
    }
}
