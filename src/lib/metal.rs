use crate::Material;
use crate::{dot, random_in_unit_sphere, reflect, unit_vector};
use crate::{Color, HitRecord, Ray};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    #[inline]
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    #[inline]
    pub fn albedo(&self) -> Color {
        self.albedo
    }

    #[inline]
    pub fn fuzz(&self) -> f64 {
        self.fuzz
    }
}

impl Material for Metal {
    fn emitted(&self, _u: f64, _v: f64, _p: &Color) -> Color {
        Color::default()
    }
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal());
        let scattered = Ray::new(
            rec.p(),
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );
        let attenuation = self.albedo;

        if dot(&scattered.direction(), &rec.normal()) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
