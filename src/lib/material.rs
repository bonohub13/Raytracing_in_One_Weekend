use crate::{dot, random_unit_vector, reflect, unit_vector};
use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

#[derive(Clone)]
pub struct MaterialDef {}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        mut _attenuation: &mut Color,
        mut _scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        *_attenuation = self.albedo.clone();
        *_scattered = Ray::new(rec.p(), scatter_direction);

        true
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        mut _attenuation: &mut Color,
        mut _scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal());
        *_attenuation = self.albedo.clone();
        *_scattered = Ray::new(rec.p(), reflected);

        dot(&_scattered.direction(), &rec.normal()) > 0.0
    }
}

impl Material for MaterialDef {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
