use crate::random_f64;
use crate::INFINITY;
use crate::{HitRecord, Isotropic, Vec3};
use crate::{Hittable, Material, Texture};

pub struct ConstantMedium<H: Hittable> {
    pub boundary: H,
    pub phase_function: Box<dyn Material>,
    negative_inverted_density: f64,
}

impl<H: Hittable> ConstantMedium<H> {
    pub fn new<T>(boundary: H, texture: T, density: f64) -> Self
    where
        T: 'static + Texture,
    {
        Self {
            boundary,
            phase_function: Box::new(Isotropic::new(texture)),
            negative_inverted_density: -1. / density,
        }
    }
}

impl<H: Hittable> Hittable for ConstantMedium<H> {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && random_f64() < 0.00001;

        if let Some(mut hit1) = self.boundary.hit(r, -INFINITY, INFINITY) {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t() + 0.0001, INFINITY) {
                if debugging {
                    eprintln!("\nt_min={}, t_max={}", hit1.t(), hit2.t());
                }

                if hit1.t() < t_min {
                    hit1.set_t(t_min);
                }
                if t_max < hit2.t() {
                    hit2.set_t(t_max);
                }
                if hit2.t() > hit1.t() {
                    if hit1.t() < 0. {
                        hit1.set_t(0.);
                    }
                    let ray_length = r.direction().length();
                    let distance_inside_boundary = (hit2.t() - hit1.t()) * ray_length;
                    let hit_distance = self.negative_inverted_density * random_f64().log10();

                    if distance_inside_boundary >= hit_distance {
                        let t = hit1.t() + hit_distance / ray_length;
                        let p = r.at(t);
                        let normal = Vec3::new(1., 0., 0.);
                        let front_face = true;
                        let material = &self.phase_function;

                        if debugging {
                            eprintln!(
                                "hit_distance = {}\nhit.t = {}\nhit.p = {}",
                                hit_distance, t, p
                            );
                        }

                        Some(HitRecord::new(
                            p,
                            normal,
                            t,
                            0.,
                            0.,
                            Some(front_face),
                            material.as_ref(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
