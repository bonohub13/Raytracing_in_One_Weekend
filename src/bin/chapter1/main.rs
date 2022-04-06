use std::io::stdout;
use std::io::Write;

fn main() {
    // Original code
    // const IMAGE_WIDTH: u32 = 256;
    // const IMAGE_HEIGHT: u32 = 256;

    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = 1080;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..IMAGE_WIDTH {
            // Rust loses digits when converted to f64
            // e.g. 1/2 as f64 returns 0 instead of 0.5
            // Need to create a variable to temporarily hold converted f64 values
            let (fi, fimage_width) = (i as f64, (IMAGE_WIDTH - 1) as f64);
            let (fj, fimage_height) = (j as f64, (IMAGE_HEIGHT - 1) as f64);
            let pixel_color = rt_utils::Color::new(fi / fimage_width, fj / fimage_height, 0.25);

            rt_utils::write_color(pixel_color);
        }
    }

    eprintln!("\nDone!");
}
