use crate::{Color, HitRecord, Point3, Ray};
use crate::{Material, Texture};

pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Point3)> {
        None
    }
}
