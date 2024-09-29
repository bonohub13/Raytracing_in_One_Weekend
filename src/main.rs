use anyhow::Result;
use rtiow::{
    camera::Camera,
    hittable::{HittableList, Sphere},
    vec3::Point3,
};
use std::rc::Rc;

fn main() -> Result<()> {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0 / 9.0, 400, 100, 50);

    cam.render(&world, "images/lambertian_sphere-gamma_corrected.ppm")?;

    Ok(())
}
