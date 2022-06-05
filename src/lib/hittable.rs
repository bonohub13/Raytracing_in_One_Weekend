use crate::dot;
use crate::{Material, Point3, Ray, Vec3};

pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    pub material: &'a dyn Material,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Point3, normal: Vec3, t: f64, material: &'a dyn Material) -> Self {
        Self {
            p: point,
            normal,
            t,
            front_face: false,
            material,
        }
    }

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

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
