mod aabb;
mod bvh;
mod camera;
mod checker_texture;
mod color;
mod dielectric;
mod diffuse_light;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod moving_sphere;
mod noise_texture;
mod perlin;
mod ray;
mod rect_xy;
mod rect_xz;
mod rect_yz;
mod renderer;
mod scenes;
mod solid_color;
mod sphere;
mod texture;
mod vec3;
mod vec3_utils;

pub use aabb::*;
pub use bvh::*;
pub use camera::*;
pub use checker_texture::*;
pub use color::*;
pub use dielectric::*;
pub use diffuse_light::*;
pub use hittable::*;
pub use hittable_list::*;
pub use lambertian::*;
pub use material::*;
pub use metal::*;
pub use moving_sphere::*;
pub use noise_texture::*;
pub use perlin::*;
pub use ray::*;
pub use rect_xy::*;
pub use rect_xz::*;
pub use rect_yz::*;
pub use renderer::*;
pub use scenes::*;
pub use solid_color::*;
pub use sphere::*;
pub use texture::*;
pub use vec3::*;
pub use vec3_utils::*;

use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

#[inline]
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

#[inline]
pub fn random_i32_in_range(min: i32, max: i32) -> i32 {
    random_f64_in_range(min as f64, max as f64 + 1.) as i32
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
fn min<T: std::cmp::PartialOrd>(val0: T, val1: T) -> T {
    if val0 < val1 {
        val0
    } else {
        val1
    }
}

#[inline]
fn max<T: std::cmp::PartialOrd>(val0: T, val1: T) -> T {
    if val0 > val1 {
        val0
    } else {
        val1
    }
}

#[inline]
pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0. {
        return -1.;
    }

    (-half_b - discriminant.sqrt()) / a
}

#[inline]
pub fn ray_color(r: &Ray, background: &Color, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit) = world.hit(r, 0.001, INFINITY) {
        let emitted = hit.material.emitted(hit.u(), hit.v(), &hit.p());

        if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        *background
    }
}
