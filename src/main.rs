use anyhow::Result;
use rtiow::{
    camera::Camera,
    hittable::{Dielectric, HittableList, Lambertian, Metal, Sphere},
    interval::Interval,
    vec3::{Color, Point3, Vec3},
};
use std::sync::Arc;

fn main() -> Result<()> {
    let mut world = HittableList::new();

    let sphere_range = Point3::new(4_f64, 0.2, 0_f64);
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, -1e3, 0_f64),
        1e3,
        material_ground,
    )));

    for ab in 0..484 {
        let a = (ab / 22 - 11) as f64;
        let b = (ab % 22 - 11) as f64;
        let choose_mat = rtiow::utils::random();
        let center = Point3::new(
            a + 0.9 * rtiow::utils::random(),
            0.2,
            b + 0.9 * rtiow::utils::random(),
        );

        if (center - sphere_range).length() > 0.9 {
            if choose_mat < 0.8 {
                let albedo = Color::random() * Color::random();
                let material = Arc::new(Lambertian::new(albedo));

                world.add(Arc::new(Sphere::new(center, 0.2, material)));
            } else if choose_mat < 0.95 {
                let albedo = Color::random_in_range(&Interval::new(0.5, 1_f64));
                let fuzz = rtiow::utils::random_in_range(&Interval::new(0_f64, 0.5));
                let material = Arc::new(Metal::new(albedo, fuzz));

                world.add(Arc::new(Sphere::new(center, 0.2, material)));
            } else {
                let material = Arc::new(Dielectric::new(1.5));

                world.add(Arc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0_f64));

    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 1_f64, 0_f64),
        1_f64,
        material1,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4_f64, 1_f64, 0_f64),
        1_f64,
        material2,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4_f64, 1_f64, 0_f64),
        1_f64,
        material3,
    )));

    let cam = Camera::new(
        16_f64 / 9_f64,
        1200,
        500,
        50,
        20_f64,
        &Point3::new(13_f64, 2_f64, 3_f64),
        &Point3::zeroes(),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0.6,
        1e1,
    );

    cam.render_png(&world, "images/final_scene.png")?;

    Ok(())
}
