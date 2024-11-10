use super::{HitRecord, Material, Onb};
use crate::{
    ray::Ray,
    texture::{SolidColor, Texture},
    utils,
    vec3::{self, Color, Point3},
    PI,
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

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

#[derive(Debug, Clone)]
pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from(tex: &Arc<dyn Texture>) -> Self {
        Self { tex: tex.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let uvw = Onb::new(&rec.normal);
        let scatter_direction = uvw.transform(&vec3::random_cosine_direction());
        let scattered = Ray::new(rec.p, vec3::unit_vector(&scatter_direction), *r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf = vec3::dot(uvw.w(), scattered.direction()) / PI;

        Some((attenuation, scattered, pdf))
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * PI)
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let reflected = {
            let reflected = vec3::reflect(r_in.direction(), &rec.normal);

            vec3::unit_vector(&reflected) + (self.fuzz * vec3::random_unit_vector())
        };
        let scattered = Ray::new(rec.p, reflected, *r_in.time());
        let attenuation = self.albedo;

        if vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered, 0.0))
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
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_index = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = vec3::dot(&(-unit_direction), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.0;
        let direction = if cannot_refract
            || (Dielectric::reflectance(cos_theta, refraction_index)) > utils::random()
        {
            vec3::reflect(&unit_direction, &rec.normal)
        } else {
            vec3::refract(&unit_direction, &rec.normal, refraction_index)
        };
        let scattered = Ray::new(rec.p, direction, *r_in.time());

        Some((attenuation, scattered, 0.0))
    }
}

impl DiffuseLight {
    pub fn new(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }

    pub fn from(tex: &Arc<dyn Texture>) -> Self {
        Self { tex: tex.clone() }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            Color::zeroes()
        } else {
            self.tex.value(u, v, p)
        }
    }
}

impl Isotropic {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from(tex: &Arc<dyn Texture>) -> Self {
        Self { tex: tex.clone() }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray, f64)> {
        let scattered = Ray::new(rec.p, vec3::random_unit_vector(), *r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        let pdf = 1.0 / (4.0 * PI);

        Some((attenuation, scattered, pdf))
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
