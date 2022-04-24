use std::io::{stdout, Write};
use std::rc::Rc;

fn main() {
    // Original code
    // const IMAGE_WIDTH: u32 = 256;
    // const IMAGE_HEIGHT: u32 = 256;

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXELS: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = rt_utils::HittableList::default();
    let material_ground = rt_utils::Lambertian::new(&rt_utils::Color::new(0.8, 0.8, 0.0));
    let material_center = rt_utils::Lambertian::new(&rt_utils::Color::new(0.7, 0.3, 0.3));
    let material_left = rt_utils::Lambertian::new(&rt_utils::Color::new(0.8, 0.8, 0.8));
    let material_right = rt_utils::Lambertian::new(&rt_utils::Color::new(0.8, 0.6, 0.2));

    let sphere_ground = rt_utils::Sphere::new(
        rt_utils::Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    );
    let sphere_center = rt_utils::Sphere::new(
        rt_utils::Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::new(material_center),
    );
    let sphere_left = rt_utils::Sphere::new(
        rt_utils::Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    );
    let sphere_right = rt_utils::Sphere::new(
        rt_utils::Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    );

    world.add(&sphere_ground);
    world.add(&sphere_center);
    world.add(&sphere_left);
    world.add(&sphere_right);

    // Camera
    let cam = rt_utils::Camera::default();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stdout().flush().ok().expect("Could not flush stdout");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = rt_utils::Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXELS {
                let u = ((i as f64) + rt_utils::random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = ((j as f64) + rt_utils::random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += rt_utils::ray_color(&r, &world, MAX_DEPTH);
            }
            rt_utils::write_color_spp(pixel_color, SAMPLES_PER_PIXELS as i32);
        }
    }

    eprintln!("\nDone!");
}
