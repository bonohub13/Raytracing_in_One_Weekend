use crate::random_in_unit_sphere;
use crate::{Color, HitRecord, Ray};
use crate::{Material, Texture};

pub struct Isotropic<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        Some((
            Ray::new(rec.p(), random_in_unit_sphere(), r_in.time()),
            self.albedo.value(rec.u(), rec.v(), &rec.p()),
        ))
    }
}
