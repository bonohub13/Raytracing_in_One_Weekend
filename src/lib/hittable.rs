use crate::dot;
use crate::Material;
use crate::{Aabb, Point3, Ray, Vec3};

pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    t: f64,
    u: f64,
    v: f64,
    front_face: bool,
    pub material: &'a dyn Material,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

impl<'a> HitRecord<'a> {
    #[inline]
    pub fn new(
        point: Point3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        material: &'a dyn Material,
    ) -> Self {
        Self {
            p: point,
            normal,
            t,
            u,
            v,
            front_face: false,
            material,
        }
    }

    #[inline]
    pub fn p(&self) -> Point3 {
        self.p
    }

    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    #[inline]
    pub fn t(&self) -> f64 {
        self.t
    }

    #[inline]
    pub fn u(&self) -> f64 {
        self.u
    }

    #[inline]
    pub fn v(&self) -> f64 {
        self.v
    }

    #[inline]
    pub fn front_face(&self) -> bool {
        self.front_face
    }

    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
