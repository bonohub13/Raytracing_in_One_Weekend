use crate::Texture;
use crate::{Color, Perlin, Point3};

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1., 1., 1.)
            * 0.5
            * (1.
                + (self.scale * p.z() + 10. * self.noise.turbulence(&(self.scale * *p), None))
                    .sin())
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 0.,
        }
    }
}
