use crate::{Color, HitRecord, Point3, Ray};

pub trait Material: Send + Sync {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
