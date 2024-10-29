use anyhow::Result;
use rtiow::{
    camera::Camera,
    hittable::{Dielectric, DiffuseLight, HittableList, Lambertian, Material, Metal, Quad, Sphere},
    interval::Interval,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture},
    vec3::{Color, Point3, Vec3},
};
use std::sync::Arc;

pub fn bouncing_spheres() -> Result<()> {
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
        let center_1 = Point3::new(
            a + 0.9 * rtiow::utils::random(),
            0.2,
            b + 0.9 * rtiow::utils::random(),
        );
        let center_2 = center_1
            + Vec3::new(
                0_f64,
                rtiow::utils::random_in_range(&Interval::new(0_f64, 0.5)),
                0_f64,
            );

        if (center_1 - sphere_range).length() > 0.9 {
            if choose_mat < 0.8 {
                let albedo = Color::random() * Color::random();
                let material = Arc::new(Lambertian::new(albedo));

                world.add(Arc::new(Sphere::new_moving(
                    center_1, center_2, 0.2, material,
                )));
            } else if choose_mat < 0.95 {
                let albedo = Color::random_in_range(&Interval::new(0.5, 1_f64));
                let fuzz = rtiow::utils::random_in_range(&Interval::new(0_f64, 0.5));
                let material = Arc::new(Metal::new(albedo, fuzz));

                world.add(Arc::new(Sphere::new_moving(
                    center_1, center_2, 0.2, material,
                )));
            } else {
                let material = Arc::new(Dielectric::new(1.5));

                world.add(Arc::new(Sphere::new_moving(
                    center_1, center_2, 0.2, material,
                )));
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
        Some(16_f64 / 9_f64),
        Some(1200),
        Some(500),
        Some(50),
        20_f64,
        &Point3::new(13_f64, 2_f64, 3_f64),
        &Point3::zeroes(),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0.6,
        1e1,
        Color::new(0.7, 0.8, 1_f64),
    );

    cam.render_png(&world, "images/checkered_ground.png")
}

pub fn checkered_spheres() -> Result<()> {
    let mut world = HittableList::new();

    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::new(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, -1e1, 0_f64),
        1e1,
        Arc::new(Lambertian::from(&checker)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 1e1, 0_f64),
        1e1,
        Arc::new(Lambertian::from(&checker)),
    )));

    let cam = Camera::new(
        Some(16_f64 / 9_f64),
        Some(400),
        Some(100),
        Some(50),
        20_f64,
        &Point3::new(13_f64, 2_f64, 3_f64),
        &Point3::zeroes(),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::new(0.7, 0.8, 1_f64),
    );

    cam.render_png(&world, "images/checkered_spheres.png")
}

pub fn earth() -> Result<()> {
    let earth_texture: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.jpg")?);
    let earth_surface = Arc::new(Lambertian::from(&earth_texture));
    let globe = Sphere::new(Point3::zeroes(), 2_f64, earth_surface);
    let cam = Camera::new(
        Some(16_f64 / 9_f64),
        Some(400),
        Some(100),
        Some(50),
        20_f64,
        &Point3::new(0_f64, 0_f64, 12_f64),
        &Point3::zeroes(),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::new(0.7, 0.8, 1_f64),
    );

    cam.render_png(&globe, "images/earthmap.png")
}

pub fn perlin_spheres() -> Result<()> {
    let mut world = HittableList::new();
    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4_f64));
    let cam = Camera::new(
        Some(16_f64 / 9_f64),
        Some(400),
        Some(100),
        Some(50),
        20_f64,
        &Point3::new(13_f64, 2_f64, 3_f64),
        &Point3::zeroes(),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::new(0.7, 0.8, 1_f64),
    );

    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, -1e3, 0_f64),
        1e3,
        Arc::new(Lambertian::from(&pertext)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 2_f64, 0_f64),
        2_f64,
        Arc::new(Lambertian::from(&pertext)),
    )));

    cam.render_png(&world, "images/hashed_random_texture.png")
}

pub fn quads() -> Result<()> {
    let mut world = HittableList::new();
    let cam = Camera::new(
        Some(1_f64),
        Some(400),
        Some(100),
        Some(50),
        80_f64,
        &Point3::new(0_f64, 0_f64, 9_f64),
        &Point3::zeroes(),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::new(0.7, 0.8, 1_f64),
    );
    let left_red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1_f64, 0.2, 0.2)));
    let back_green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 1_f64, 0.2)));
    let right_blue: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1_f64)));
    let upper_orange: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(1_f64, 0.5, 0_f64)));
    let lower_teal: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3_f64, -2_f64, 5_f64),
        Vec3::new(0_f64, 0_f64, -4_f64),
        Vec3::new(0_f64, 4_f64, 0_f64),
        &left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2_f64, -2_f64, 0_f64),
        Vec3::new(4_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 4_f64, 0_f64),
        &back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3_f64, -2_f64, 1_f64),
        Vec3::new(0_f64, 0_f64, 4_f64),
        Vec3::new(0_f64, 4_f64, 0_f64),
        &right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2_f64, 3_f64, 1_f64),
        Vec3::new(4_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 4_f64),
        &upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2_f64, -3_f64, 5_f64),
        Vec3::new(4_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, -4_f64),
        &lower_teal,
    )));

    cam.render_png(&world, "images/quads.png")
}

pub fn simple_light() -> Result<()> {
    let mut world = HittableList::new();
    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(4_f64));
    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(4_f64, 4_f64, 4_f64)));
    let cam = Camera::new(
        Some(16_f64 / 9_f64),
        Some(400),
        Some(100),
        Some(50),
        20_f64,
        &Point3::new(26_f64, 3_f64, 6_f64),
        &Point3::new(0_f64, 2_f64, 0_f64),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::zeroes(),
    );

    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, -1e3, 0_f64),
        1e3,
        Arc::new(Lambertian::from(&pertext)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 2_f64, 0_f64),
        2_f64,
        Arc::new(Lambertian::from(&pertext)),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3_f64, 1_f64, -2_f64),
        Vec3::new(2_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 2_f64, 0_f64),
        &difflight,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 7_f64, 0_f64),
        2_f64,
        difflight,
    )));

    cam.render_png(&world, "images/scene_with_rectangle_light_source.png")
}
