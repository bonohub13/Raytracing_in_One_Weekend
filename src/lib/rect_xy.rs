use crate::{Aabb, HitRecord, Point3, Vec3};
use crate::{Hittable, Material};

pub struct RectXY<M: Material> {
    pub material: M,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl<M: Material> RectXY<M> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: M) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl<M: Material> Hittable for RectXY<M> {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();

        if t >= t_min && t_max >= t {
            let x = r.origin().x() + t * r.direction().x();
            let y = r.origin().y() + t * r.direction().y();

            if x >= self.x0 && self.x1 >= x && y >= self.y0 && self.y1 >= y {
                let outward_normal = Vec3::new(0., 0., 1.);
                let mut rec = HitRecord::new(
                    r.at(t),
                    outward_normal,
                    t,
                    (x - self.x0) / (self.x1 - self.x0),
                    (y - self.y0) / (self.y1 - self.y0),
                    None,
                    &self.material,
                );
                rec.set_face_normal(r, &outward_normal);

                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
