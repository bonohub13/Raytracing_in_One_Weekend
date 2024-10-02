use crate::{
    buffer::ImageBuffer,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utils,
    vec3::{self, Color, Point3, Vec3},
    INFINITY,
};
use anyhow::Result;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Camera {
    image_size: [i32; 2],
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta: [Vec3; 2],
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
}

impl Camera {
    const EMPTY_SPACES: &'static str = "          ";

    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
    ) -> Self {
        let image_height = {
            let image_height = (image_width as f64 / aspect_ratio) as i32;

            if image_height < 1 {
                1
            } else {
                image_height
            }
        };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = Point3::zeroes();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta = [
            viewport_u / image_width as f64,
            viewport_v / image_height as f64,
        ];

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta.iter().sum::<Point3>());

        Self {
            image_size: [image_width, image_height],
            center,
            pixel_delta,
            pixel00_loc,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn Hittable, output_file: &str) -> Result<()> {
        let mut writer = ImageBuffer::new(output_file);
        let buffer = (0..(self.image_size[0] * self.image_size[1]))
            .into_par_iter()
            .map(|ij| {
                let j = ij / self.image_size[0];
                let i = ij % self.image_size[0];
                let pixel_color = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| {
                        let r = self.get_ray(i, j);

                        Self::ray_color(&r, self.max_depth, world)
                    })
                    .sum::<Color>();

                vec3::write_color(&(self.pixel_samples_scale * pixel_color))
            })
            .collect::<Vec<[i32; 3]>>();

        writer.set_buffer(&buffer);
        writer.write([self.image_size[0] as usize, self.image_size[1] as usize])?;

        eprintln!("\r\n\nDone!{}", Self::EMPTY_SPACES.repeat(10));

        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta[0])
            + ((j as f64 + offset.y()) * self.pixel_delta[1]);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(utils::random() - 0.5, utils::random() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::zeroes();
        }

        if let Some(rec) = world.hit(r, &Interval::new(0.001, INFINITY)) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }

            return Color::zeroes();
        }

        let unit_direction = vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
