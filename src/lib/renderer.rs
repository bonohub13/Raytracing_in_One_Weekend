use crate::{random_f64, ray_color, write_color};
use crate::{Camera, Color, HittableList};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

pub fn ppm_p3(image_width: i32, image_height: i32) {
    println!("P3\n{} {}\n255", image_width, image_height);
}

pub fn render(
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    depth: i32,
    world: &HittableList,
    cam: &Camera,
) {
    eprintln!(":: Starting render ::");

    let bar = ProgressBar::new(image_height as u64).with_style(
        ProgressStyle::default_bar()
            .template("Rendering: [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:} scanlines"),
    );

    let pixels = (0..image_height)
        .into_par_iter()
        .rev()
        .progress_with(bar)
        .map(|j| {
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut col = Color::default();

                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + random_f64()) / (image_width as f64 - 1.0);
                        let v = (j as f64 + random_f64()) / (image_height as f64 - 1.0);
                        let r = cam.get_ray(u, v);

                        col += ray_color(&r, world, depth);
                    }

                    write_color(col, samples_per_pixel)
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{}", pixels);

    eprintln!("\n:: Rendering done! ::");
}
