use crate::dot;
use crate::hittable::*;
use crate::Material;
use crate::Point3;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, mat: Rc<dyn Material>) -> Sphere {
        Self {
            center: cen,
            radius: r,
            mat: mat.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(&r.at(rec.t()));
        let outward_normal = (rec.p() - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
