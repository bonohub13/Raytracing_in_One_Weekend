mod camera;
mod sphere;

pub use sphere::Sphere;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Aabb {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}
