use crate::Texture;
use crate::{Color, Point3};

#[derive(Clone, Copy)]
pub struct CheckerTexture<T: Texture> {
    pub odd: T,
    pub even: T,
}

impl<T: Texture> CheckerTexture<T> {
    pub fn new(odd: T, even: T) -> Self {
        Self { odd, even }
    }
}

impl<T: Texture> Texture for CheckerTexture<T> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
