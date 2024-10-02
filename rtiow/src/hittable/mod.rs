use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{self, Color, Point3, Vec3},
};
use std::fmt::Debug;

pub mod hittable_list;
pub mod material;
pub mod sphere;

pub use hittable_list::*;
pub use material::*;
pub use sphere::*;

#[derive(Debug)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
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

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}
