use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{self, Point3, Vec3},
};

pub mod hittable_list;
pub mod sphere;

pub use hittable_list::*;
pub use sphere::*;

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, _r: &Ray, _ray_t: &Interval) -> Option<HitRecord> {
        None
    }
}
