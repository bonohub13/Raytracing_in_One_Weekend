pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use camera::Camera;
pub use color::{write_color, write_color_spp};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::*;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;
pub const RAND_MAX: i32 = 1073741823;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (2.0 * a);
    }
}

pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let rec = &mut HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, INFINITY, rec) {
        let target: Point3 = rec.p() + rec.normal() + random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p(), target - rec.p()), world, depth - 1);
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn random_f64() -> f64 {
    rand::random()
}

pub fn rand_f64_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
}
