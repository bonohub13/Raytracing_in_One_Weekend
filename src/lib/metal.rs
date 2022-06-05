use crate::Material;
use crate::{dot, reflect, unit_vector};
use crate::{Color, HitRecord, Ray};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal());
        let scattered = Ray::new(rec.p(), reflected);
        let attenuation = self.albedo;

        if dot(&scattered.direction(), &rec.normal()) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
