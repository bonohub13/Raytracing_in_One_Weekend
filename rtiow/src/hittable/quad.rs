use super::{Aabb, HitRecord, Hittable, HittableList, Material};
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

    pub fn create_box(a: Point3, b: Point3, mat: &Arc<dyn Material>) -> HittableList {
        let mut sides = HittableList::new();
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));
        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Arc::new(Self::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat,
        )));
        sides.add(Arc::new(Self::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            mat,
        )));
        sides.add(Arc::new(Self::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            mat,
        )));
        sides.add(Arc::new(Self::new(min, dz, dy, mat)));
        sides.add(Arc::new(Self::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            mat,
        )));
        sides.add(Arc::new(Self::new(min, dx, dz, mat)));

        sides
    }

    pub fn set_bounding_box(&mut self) {
        let bbox_diagonal = [
            Aabb::new(self.q, self.q + self.uvw[0] + self.uvw[1]),
            Aabb::new(self.q + self.uvw[0], self.q + self.uvw[1]),
        ];

        self.bbox = Aabb::surrounding_box(&bbox_diagonal[0], &bbox_diagonal[1]);
    }

    pub fn is_interior(&self, a: f64, b: f64) -> Option<(f64, f64)> {
        const UNIT_INTERVAL: Interval = Interval { min: 0.0, max: 1.0 };

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
