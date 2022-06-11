use crate::random_unit_vector;
use crate::{Color, HitRecord, Ray};
use crate::{Material, Texture};

#[derive(Clone, Copy)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    #[inline]
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn emitted(&self, _u: f64, _v: f64, _p: &Color) -> Color {
        Color::default()
    }
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }

        let scatter = Ray::new(rec.p(), scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u(), rec.v(), &rec.p());

        Some((scatter, attenuation))
    }
}
