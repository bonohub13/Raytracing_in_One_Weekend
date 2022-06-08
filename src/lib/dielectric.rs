use crate::Material;
use crate::{dot, random_f64, reflect, refract, unit_vector};
use crate::{Color, HitRecord, Ray};

#[derive(Copy, Clone)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    #[inline]
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    #[inline]
    pub fn index_of_refraction(&self) -> f64 {
        self.index_of_refraction
    }

    #[inline]
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);

        r0.powi(2) + (1.0 - r0.powi(2)) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = unit_vector(&r_in.direction());
        let mut cos_theta = dot(&-unit_direction, &rec.normal());
        if cos_theta < 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            reflect(&unit_direction, &rec.normal())
        } else {
            refract(&unit_direction, &rec.normal(), refraction_ratio)
        };

        let scattered = Ray::new(rec.p(), direction, r_in.time());

        Some((scattered, attenuation))
    }
}
