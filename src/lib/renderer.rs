pub fn ppm_p3(image_width: i64, image_height: i64) {
    println!("P3\n{} {}\n255", image_width, image_height);
}

pub fn render(image_width: i64, image_height: i64) {
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = (i as f64) / ((image_width as f64) - 1.0);
            let g = (j as f64) / ((image_height as f64) - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
