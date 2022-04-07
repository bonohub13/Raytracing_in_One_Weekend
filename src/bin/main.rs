use std::io::stdout;
use std::io::Write;

fn main() {
    // Original code
    // const IMAGE_WIDTH: u32 = 256;
    // const IMAGE_HEIGHT: u32 = 256;

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // World
    let mut world = rt_utils::HittableList::default();
    world.add(Box::new(rt_utils::Sphere::new(
        rt_utils::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(rt_utils::Sphere::new(
        rt_utils::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = rt_utils::Point3::new(0.0, 0.0, 0.0);
    let horizontal = rt_utils::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = rt_utils::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2 - vertical / 2 - rt_utils::Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..IMAGE_WIDTH {
            // Rust loses digits when converted to f64
            // e.g. 1/2 as f64 returns 0 instead of 0.5
            // Need to create a variable to temporarily hold converted f64 values
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = rt_utils::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = rt_utils::ray_color(&r, &world);

            rt_utils::write_color(pixel_color);
        }
    }

    eprintln!("\nDone!");
}
