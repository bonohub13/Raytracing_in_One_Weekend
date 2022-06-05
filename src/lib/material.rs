use crate::{Color, HitRecord, Ray};

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}
