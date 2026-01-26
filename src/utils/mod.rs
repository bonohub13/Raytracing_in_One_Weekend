use glam::Vec3A;
use rand::prelude::*;

pub(crate) const PI: f32 = std::f32::consts::PI;

pub(crate) fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180f32
}

pub(crate) fn random() -> f32 {
    rand::rng().random()
}

pub(crate) fn random_in_range(min: f32, max: f32) -> f32 {
    rand::rng().random_range(min..max)
}

pub(crate) fn random_vec3a() -> Vec3A {
    glam::vec3a(random(), random(), random())
}

pub(crate) fn random_in_range_vec3a(min: f32, max: f32) -> Vec3A {
    glam::vec3a(
        random_in_range(min, max),
        random_in_range(min, max),
        random_in_range(min, max),
    )
}

pub(crate) unsafe fn data_into_bytes<T>(data: &[T]) -> &[u8]
where
    T: Sized,
{
    let p_data = &data[0] as *const T;

    unsafe { std::slice::from_raw_parts(p_data as *const u8, size_of_val(data)) }
}
