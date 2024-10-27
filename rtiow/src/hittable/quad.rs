use super::{Aabb, HitRecord, Hittable, Material};
use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{self, Point3, Vec3},
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Quad {
    q: Point3,
    uvw: [Vec3; 3],
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: &Arc<dyn Material>) -> Self {
        let n = vec3::cross(&u, &v);
        let normal = vec3::unit_vector(&n);
        let d = vec3::dot(&normal, &q);
        let w = n / vec3::dot(&n, &n);
        let mut ret = Self {
            q,
            uvw: [u, v, w],
            mat: mat.clone(),
            bbox: Aabb::new(Vec3::zeroes(), Vec3::zeroes()),
            normal,
            d,
        };

        ret.set_bounding_box();

        ret
    }

    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal = [
            Aabb::new(self.q, self.q + self.uvw[0] + self.uvw[1]),
            Aabb::new(self.q + self.uvw[0], self.q + self.uvw[1]),
        ];

        self.bbox = Aabb::surrounding_box(&bbox_diagonal[0], &bbox_diagonal[1]);
    }

    pub fn is_interior(&self, a: f64, b: f64) -> Option<(f64, f64)> {
        const UNIT_INTERVAL: Interval = Interval {
            min: 0_f64,
            max: 1_f64,
        };

        if (!UNIT_INTERVAL.contains(a)) || (!UNIT_INTERVAL.contains(b)) {
            None
        } else {
            Some((a, b))
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denominator = vec3::dot(&self.normal, r.direction());

        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - vec3::dot(&self.normal, r.origin())) / denominator;

        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_httpt_vector = intersection - self.q;
        let alpha = vec3::dot(
            &self.uvw[2],
            &vec3::cross(&planar_httpt_vector, &self.uvw[1]),
        );
        let beta = vec3::dot(
            &self.uvw[2],
            &vec3::cross(&self.uvw[0], &planar_httpt_vector),
        );

        if let Some((u, v)) = self.is_interior(alpha, beta) {
            let mut rec = HitRecord {
                t,
                p: intersection,
                mat: self.mat.as_ref(),
                normal: Vec3::zeroes(),
                u,
                v,
                front_face: false,
            };

            rec.set_face_normal(r, &self.normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
