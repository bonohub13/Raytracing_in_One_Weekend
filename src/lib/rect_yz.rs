use crate::{Aabb, HitRecord, Point3, Vec3};
use crate::{Hittable, Material};

pub struct RectYZ<M: Material> {
    pub material: M,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl<M: Material> RectYZ<M> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: M) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl<M: Material> Hittable for RectYZ<M> {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();

        if t > t_min && t_max > t {
            let y = r.origin().y() + t * r.direction().y();
            let z = r.origin().z() + t * r.direction().z();

            if y > self.y0 && self.y1 > y && z > self.z0 && self.z1 > z {
                let u = (y - self.y0) / (self.y1 - self.y0);
                let v = (z - self.z0) / (self.z0 - self.z1);
                let outward_normal = Vec3::new(1., 0., 0.);
                let mut hit =
                    HitRecord::new(r.at(t), outward_normal, t, u, v, None, &self.material);

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
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
