use crate::dot;
use crate::{HitRecord, Point3, Ray};
use crate::{Hittable, Material};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(cen: Point3, r: f64, material: M) -> Self {
        Self {
            center: cen,
            radius: r,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant >= 0. {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root >= t_min && t_max >= root {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                let mut rec = HitRecord::new(p, normal, root, &self.material);

                rec.set_face_normal(r, &normal);

                return Some(rec);
            }

            root = (-half_b + sqrtd) / a;

            if root >= t_min && t_max >= root {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                let mut rec = HitRecord::new(p, normal, root, &self.material);

                rec.set_face_normal(r, &normal);

                return Some(rec);
            }
        }

        None
    }
}
