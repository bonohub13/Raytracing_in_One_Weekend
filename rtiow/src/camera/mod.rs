use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utils,
    vec3::{self, Color, Point3, Vec3},
    writer::PpmWriter,
    INFINITY, PI,
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
    sqrt_spp: i32,
    pixel_samples_scale: f64,
    recip_sqrt_spp: f64,
    max_depth: i32,
    defocus_angle: f64,
    defocus_disk: [Vec3; 2],
    background: Color,
}

impl Camera {
    const EMPTY_SPACES: &'static str = "          ";

    pub fn new(
        aspect_ratio: Option<f64>,
        image_width: Option<i32>,
        samples_per_pixel: Option<i32>,
        max_depth: Option<i32>,
        vfov: f64,
        look_from: &Point3,
        look_at: &Point3,
        vup: &Vec3,
        defocus_angle: f64,
        focus_distance: f64,
        background: Color,
    ) -> Self {
        // Parameters with default values
        let aspect_ratio = aspect_ratio.unwrap_or(1.0);
        let image_width = image_width.unwrap_or(100);
        let samples_per_pixel = samples_per_pixel.unwrap_or(10);
        let max_depth = max_depth.unwrap_or(10);

        let image_height = {
            let image_height = (image_width as f64 / aspect_ratio) as i32;

            if image_height < 1 {
                1
            } else {
                image_height
            }
        };

        let sqrt_spp = (samples_per_pixel as f64).sqrt() as i32;
        let pixel_samples_scale = 1.0 / sqrt_spp.pow(2) as f64;
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

        let center = *look_from;

        let theta = utils::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
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

        let viewport_upper_left = center - (focus_distance * w) - (viewport_u + viewport_v) / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta.iter().sum::<Point3>());

        let defocus_radius = focus_distance * utils::degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk = [u * defocus_radius, v * defocus_radius];

        Self {
            image_size: [image_width, image_height],
            center,
            pixel_delta,
            pixel00_loc,
            sqrt_spp,
            recip_sqrt_spp,
            pixel_samples_scale,
            max_depth,
            defocus_angle,
            defocus_disk,
            background,
        }
    }

    pub fn render_ppm(&self, world: &dyn Hittable, output_file: &str) -> Result<()> {
        let mut writer = PpmWriter::new(output_file);
        let buffer = self.render(world)?;

        writer.set_buffer(&buffer);
        writer.write([self.image_size[0] as usize, self.image_size[1] as usize])?;

        eprintln!("\r\n\nDone!{}", Self::EMPTY_SPACES.repeat(10));

        Ok(())
    }

    pub fn render_png(&self, world: &dyn Hittable, output_file: &str) -> Result<()> {
        let image_size = [self.image_size[0] as u32, self.image_size[1] as u32];
        let mut buffer: RgbImage = ImageBuffer::new(image_size[0], image_size[1]);
        let pixels = self.render(world)?;

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

    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta[0])
            + ((j as f64 + offset.y()) * self.pixel_delta[1]);
        let ray_origin = if self.defocus_angle <= 0.0 {
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

    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px = ((s_i as f64 + utils::random()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + utils::random()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.0)
    }

    #[allow(dead_code)]
    fn sample_square() -> Vec3 {
        Vec3::new(utils::random() - 0.5, utils::random() - 0.5, 0.0)
    }

    fn render(&self, world: &dyn Hittable) -> Result<Vec<[i32; 3]>> {
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
                let pixel_color = (0..self.sqrt_spp)
                    .into_par_iter()
                    .map(|s_j| {
                        (0..self.sqrt_spp)
                            .into_par_iter()
                            .map(|s_i| {
                                let r = self.get_ray(i, j, s_i, s_j);

                                self.ray_color(&r, self.max_depth, world)
                            })
                            .sum::<Color>()
                    })
                    .sum::<Color>();

                vec3::write_color(&(self.pixel_samples_scale * pixel_color))
            })
            .collect::<Vec<[i32; 3]>>();

        Ok(pixels)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::zeroes();
        }

        if let Some(rec) = world.hit(r, &Interval::new(0.001, INFINITY)) {
            let color_from_emission = rec.mat.emitted(rec.u, rec.v, &rec.p);

            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                let scattering_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);
                let pdf_value = 1.0 / (2.0 * PI);
                let color_from_scatter =
                    (attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world))
                        / pdf_value;

                return color_from_emission + color_from_scatter;
            }

            return color_from_emission;
        }

        self.background
    }
}
