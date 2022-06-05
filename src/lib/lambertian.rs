use crate::random_unit_vector;
use crate::Material;
use crate::{Color, HitRecord, Ray};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let scatter = Ray::new(rec.p(), scatter_direction);
        let attenuation = self.albedo;

        Some((scatter, attenuation))
    }
}
