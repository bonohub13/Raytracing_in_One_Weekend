use crate::{Aabb, HitRecord, Point3, Vec3};
use crate::{Hittable, Material};

pub struct RectXZ<M: Material> {
    pub material: M,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<M: Material> RectXZ<M> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for RectXZ<M> {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();

        if t > t_min && t_max > t {
            let x = r.origin().x() + t * r.direction().x();
            let z = r.origin().z() + t * r.direction().z();

            if x > self.x0 && self.x1 > x && z > self.z0 && self.z1 > z {
                let u = (x - self.x0) / (self.x1 - self.x0);
                let v = (z - self.z0) / (self.z0 - self.z1);
                let outward_normal = Vec3::new(0., 1., 0.);
                let mut hit = HitRecord::new(r.at(t), outward_normal, t, u, v, &self.material);

                hit.set_face_normal(r, &outward_normal);

                Some(hit)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
