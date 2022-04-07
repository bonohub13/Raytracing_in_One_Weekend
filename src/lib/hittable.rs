use crate::dot;
use crate::{Point3, Ray, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
    // Getters
    pub fn p(&self) -> Point3 {
        self.p
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    // Setters
    pub fn set_p(&mut self, p: &Point3) {
        self.p = *p;
    }
    pub fn set_normal(&mut self, normal: &Vec3) {
        self.normal = *normal;
    }
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }
    pub fn set_front_face(&mut self, front_face: &bool) {
        self.front_face = *front_face
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
