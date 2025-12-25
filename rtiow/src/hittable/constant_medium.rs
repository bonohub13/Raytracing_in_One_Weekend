use super::{Aabb, HitRecord, Hittable, Isotropic, Material};
use crate::{
    interval::Interval,
    ray::Ray,
    texture::Texture,
    utils,
    vec3::{Color, Vec3},
    INFINITY,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: &Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary: boundary.clone(),
            neg_inv_density: 1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }

    pub fn from(boundary: &Arc<dyn Hittable>, density: f64, tex: &Arc<dyn Texture>) -> Self {
        Self {
            boundary: boundary.clone(),
            neg_inv_density: 1.0 / density,
            phase_function: Arc::new(Isotropic::from(tex)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        if let Some(mut rec1) = self.boundary.hit(r, &Interval::UNIVERSE) {
            if let Some(mut rec2) = self
                .boundary
                .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY))
            {
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }
                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * utils::random().ln_1p();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;

                Some(HitRecord {
                    t,
                    p: r.at(t),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    front_face: true,
                    mat: self.phase_function.as_ref(),
                    u: 0.0,
                    v: 0.0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.boundary.bounding_box()
    }
}
