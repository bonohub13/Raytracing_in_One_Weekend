mod functions;

pub use functions::{get_random_color, random_float, random_float_in_range};

pub enum MaterialType {
    DIFFUSE,
    METAL,
    REFRACTIVE,
}

pub enum TextureType {
    SOLID,
    CHECKERED,
}

#[repr(align(16))]
pub struct Sphere {
    pub center: cgmath::Vector3<f32>,
    pub radius: f32,
    pub material_index: u32,
}

#[repr(align(16))]
pub struct Color {
    pub color: cgmath::Vector3<f32>,
}

#[repr(align(16))]
pub struct Material {
    pub type_: u32,
    pub texture_type: u32,
    pub colors: [Color; 2],
    pub specific_attribute: f32,
}

#[repr(align(4))]
pub struct Scene {
    pub sphere_amount: u32,
    pub spheres: [Sphere; 500],
    pub materials: [Material; 500],
}
