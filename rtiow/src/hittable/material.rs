use super::{HitRecord, Material};
use crate::{
    ray::Ray,
    vec3::{self, Color},
};

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = {
            let scatter_direction = rec.normal + vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                rec.normal
            } else {
                scatter_direction
            }
        };
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

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
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if vec3::dot(scattered.direction(), &rec.normal) > 0_f64 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
