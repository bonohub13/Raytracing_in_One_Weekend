use super::{HitRecord, Material};
use crate::{
    ray::Ray,
    texture::{SolidColor, Texture},
    utils,
    vec3::{self, Color},
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from(tex: Arc<dyn Texture>) -> Self {
        Self { tex: tex.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = {
            let scatter_direction = rec.normal + vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                rec.normal
            } else {
                scatter_direction
            }
        };
        let scattered = Ray::new(rec.p, scatter_direction, *r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);

        Some((attenuation, scattered))
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = {
            let reflected = vec3::reflect(r_in.direction(), &rec.normal);

            vec3::unit_vector(&reflected) + (self.fuzz * vec3::random_unit_vector())
        };
        let scattered = Ray::new(rec.p, reflected, *r_in.time());
        let attenuation = self.albedo;

        if vec3::dot(scattered.direction(), &rec.normal) > 0_f64 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1_f64 - refraction_index) / (1_f64 + refraction_index)).powi(2);

        r0 + (1_f64 - r0) * (1_f64 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1_f64, 1_f64, 1_f64);
        let refraction_index = if rec.front_face {
            1_f64 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = vec3::dot(&(-unit_direction), &rec.normal).min(1_f64);
        let sin_theta = (1_f64 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1_f64;
        let direction = if cannot_refract
            || (Dielectric::reflectance(cos_theta, refraction_index)) > utils::random()
        {
            vec3::reflect(&unit_direction, &rec.normal)
        } else {
            vec3::refract(&unit_direction, &rec.normal, refraction_index)
        };
        let scattered = Ray::new(rec.p, direction, *r_in.time());

        Some((attenuation, scattered))
    }
}
