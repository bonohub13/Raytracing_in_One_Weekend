use super::{Aabb, HitRecord, Hittable, Material};
use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{self, Point3, Vec3},
    PI,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::zeroes(), 0_f64),
            radius: 0_f64.max(radius),
            mat,
        }
    }

    pub fn new_moving(
        center_1: Point3,
        center_2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center_1, center_2 - center_1, 0_f64);

        Self {
            center,
            radius: 0_f64.max(radius),
            mat,
        }
    }

    fn get_sphere_uv(p: &Point3) -> [f64; 2] {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        let u = phi / (2_f64 * PI);
        let v = theta / PI;

        [u, v]
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(*r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0_f64 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - current_center) / self.radius;
        let uv = Self::get_sphere_uv(&outward_normal);
        let mut hit = HitRecord {
            p,
            normal: outward_normal,
            t: root,
            u: uv[0],
            v: uv[1],
            front_face: false,
            mat: self.mat.as_ref(),
        };

        hit.set_face_normal(r, &outward_normal);

        Some(hit)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let rvec = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = Aabb::new(self.center.at(0_f64) - rvec, self.center.at(0_f64) + rvec);
        let box1 = Aabb::new(self.center.at(1_f64) - rvec, self.center.at(1_f64) + rvec);

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
