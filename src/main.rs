use anyhow::Result;
use rtiow::{
    camera::Camera,
    hittable::{Dielectric, HittableList, Lambertian, Metal, Sphere},
    vec3::{Color, Point3},
};
use std::sync::Arc;

fn main() -> Result<()> {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0_f64)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1_f64 / 1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1_f64));

    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, -100.5, -1_f64),
        100_f64,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 0_f64, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1_f64, 0_f64, -1_f64),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1_f64, 0_f64, -1_f64),
        0.4,
        material_bubble,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1_f64, 0_f64, -1_f64),
        0.5,
        material_right,
    )));

    let cam = Camera::new(16_f64 / 9_f64, 400, 100, 50);

    cam.render(&world, "images/hollow_glass_sphere.ppm")?;

    Ok(())
}
