use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utils,
    vec3::{self, Color, Point3, Vec3},
    writer::PpmWriter,
    INFINITY,
};
use anyhow::Result;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
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
    defocus_angle: f64,
    defocus_disk: [Vec3; 2],
}

impl Camera {
    const EMPTY_SPACES: &'static str = "          ";

    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: &Point3,
        look_at: &Point3,
        vup: &Vec3,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        let image_height = {
            let image_height = (image_width as f64 / aspect_ratio) as i32;

            if image_height < 1 {
                1
            } else {
                image_height
            }
        };

        let pixel_samples_scale = 1_f64 / samples_per_pixel as f64;

        let center = *look_from;

        let theta = utils::degrees_to_radians(vfov);
        let h = (theta / 2_f64).tan();
        let viewport_height = 2_f64 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = vec3::unit_vector(&(*look_from - look_at));
        let u = vec3::unit_vector(&vec3::cross(vup, &w));
        let v = vec3::cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta = [
            viewport_u / image_width as f64,
            viewport_v / image_height as f64,
        ];

        let viewport_upper_left = center - (focus_distance * w) - (viewport_u + viewport_v) / 2_f64;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta.iter().sum::<Point3>());

        let defocus_radius =
            focus_distance * utils::degrees_to_radians(defocus_angle / 2_f64).tan();
        let defocus_disk = [u * defocus_radius, v * defocus_radius];

        Self {
            image_size: [image_width, image_height],
            center,
            pixel_delta,
            pixel00_loc,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            defocus_angle,
            defocus_disk,
        }
    }

    pub fn render_ppm(&self, world: &dyn Hittable, output_file: &str) -> Result<()> {
        let mut writer = PpmWriter::new(output_file);
        let bar = ProgressBar::new((self.image_size[0] * self.image_size[1]) as u64).with_style(
            ProgressStyle::default_bar().template(
                "Rendering: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:} scanlines",
            )?,
        );
        let buffer = (0..(self.image_size[0] * self.image_size[1]))
            .into_par_iter()
            .progress_with(bar)
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

    pub fn render_png(&self, world: &dyn Hittable, output_file: &str) -> Result<()> {
        let image_size = [self.image_size[0] as u32, self.image_size[1] as u32];
        let mut buffer: RgbImage = ImageBuffer::new(image_size[0], image_size[1]);
        let bar = ProgressBar::new((self.image_size[0] * self.image_size[1]) as u64).with_style(
            ProgressStyle::default_bar().template(
                "Rendering: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:} scanlines",
            )?,
        );
        let pixels = (0..(self.image_size[0] * self.image_size[1]))
            .into_par_iter()
            .progress_with(bar)
            .map(|ij| {
                let j = ij / self.image_size[0];
                let i = ij % self.image_size[0];
                let pixel_color = (0..self.samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        let r = self.get_ray(i, j);

                        Self::ray_color(&r, self.max_depth, world)
                    })
                    .sum::<Color>();

                vec3::write_color(&(self.pixel_samples_scale * pixel_color))
            })
            .collect::<Vec<[i32; 3]>>();

        buffer.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let rgb = pixels[(y * image_size[0] + x) as usize];

            *pixel = Rgb([rgb[0] as u8, rgb[1] as u8, rgb[2] as u8]);
        });

        match buffer.save(output_file) {
            Ok(()) => {
                eprintln!("Done!");

                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to write file: {}", e);
                Err(e.into())
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta[0])
            + ((j as f64 + offset.y()) * self.pixel_delta[1]);
        let ray_origin = if self.defocus_angle <= 0_f64 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = utils::random();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = vec3::random_in_unit_disk();

        self.center + (p[0] * self.defocus_disk[0]) + (p[1] * self.defocus_disk[1])
    }

    fn sample_square() -> Vec3 {
        Vec3::new(utils::random() - 0.5, utils::random() - 0.5, 0_f64)
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
        let a = 0.5 * (unit_direction.y() + 1_f64);

        (1_f64 - a) * Color::new(1_f64, 1_f64, 1_f64) + a * Color::new(0.5, 0.7, 1_f64)
    }
}
