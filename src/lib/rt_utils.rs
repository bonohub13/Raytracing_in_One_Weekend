mod camera;
mod color;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod renderer;
mod sphere;
mod vec3;
mod vec3_utils;

pub use camera::*;
pub use color::*;
pub use hittable::*;
pub use hittable_list::*;
pub use lambertian::*;
pub use material::*;
pub use metal::*;
pub use ray::*;
pub use renderer::*;
pub use sphere::*;
pub use vec3::*;
pub use vec3_utils::*;

use rand::Rng;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen()
}

pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

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

pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit) = world.hit(r, 0.001, INFINITY) {
        if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::default();
        }
    }

    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
