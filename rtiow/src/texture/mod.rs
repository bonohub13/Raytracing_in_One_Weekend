use crate::{interval::Interval, vec3::Color};
use anyhow::Result;
use std::{fmt::Debug, sync::Arc};

mod image;
mod perlin;

pub use image::RtwImage;
pub use perlin::*;

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Color) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    albedo: Color,
}

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture + 'static>,
    odd: Arc<dyn Texture + 'static>,
}

#[derive(Debug)]
pub struct ImageTexture {
    image: RtwImage,
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Color) -> Color {
        self.albedo
    }
}

impl CheckerTexture {
    pub fn new(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1_f64 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }

    pub fn from(scale: f64, even: &Arc<dyn Texture>, odd: &Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1_f64 / scale,
            even: even.clone(),
            odd: odd.clone(),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Color) -> Color {
        let is_even = [
            (self.inv_scale * p.x()).floor() as i32,
            (self.inv_scale * p.y()).floor() as i32,
            (self.inv_scale * p.z()).floor() as i32,
        ]
        .iter()
        .sum::<i32>()
            % 2
            == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

impl ImageTexture {
    pub fn new(filename: &str) -> Result<Self> {
        Ok(Self {
            image: RtwImage::new(filename)?,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Color) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0_f64, 1_f64, 1_f64);
        }

        let u = Interval::new(0_f64, 1_f64).clamp(u);
        let v = 1_f64 - Interval::new(0_f64, 1_f64).clamp(v);
        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel = self.image.pixel_data(i, j);
        let color_scale = 1_f64 / 255_f64;

        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Color) -> Color {
        // Color::new(1_f64, 1_f64, 1_f64) * 0.5 * (1_f64 + self.noise.noise(&(self.scale * p)))
        Color::new(1_f64, 1_f64, 1_f64) * self.noise.turbulance(p, 7)
    }
}
