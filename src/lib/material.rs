use crate::{dot, random_unit_vector, reflect, unit_vector};
use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter<M>(
        &self,
        r_in: &Ray,
        rec: &HitRecord<M>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool
    where
        M: Material,
    {
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
pub struct DefaultObject {}

impl Material for Lambertian {
    fn scatter<M>(
        &self,
        r_in: &Ray,
        rec: &HitRecord<M>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool
    where
        M: Material,
    {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }

        scattered = &mut Ray::new(rec.p(), scatter_direction);
        attenuation = &mut self.albedo.clone();

        true
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

impl Material for Metal {
    fn scatter<M>(
        &self,
        r_in: &Ray,
        rec: &HitRecord<M>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool
    where
        M: Material,
    {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal());
        scattered = &mut Ray::new(rec.p(), reflected);
        attenuation = &mut self.albedo.clone();

        dot(&scattered.direction(), &rec.normal()) > 0.0
    }
}

impl Material for DefaultObject {
    fn scatter<M>(
        &self,
        r_in: &Ray,
        rec: &HitRecord<M>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool
    where
        M: Material,
    {
        false
    }
}
