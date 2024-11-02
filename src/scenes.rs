use anyhow::Result;
use rtiow::{
    camera::Camera,
    hittable::{
        BvhNode, ConstantMedium, Dielectric, DiffuseLight, Hittable, HittableList, Lambertian,
        Material, Metal, Quad, RotateY, Sphere, Translate,
    },
    interval::Interval,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture},
    utils,
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

pub fn cornell_box() -> Result<()> {
    let mut world = HittableList::new();
    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(15_f64, 15_f64, 15_f64)));
    let cam = Camera::new(
        Some(1_f64),
        Some(600),
        Some(200),
        Some(50),
        40_f64,
        &Point3::new(278_f64, 278_f64, -800_f64),
        &Point3::new(278_f64, 278_f64, 0_f64),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::zeroes(),
    );

    world.add(Arc::new(Quad::new(
        Point3::new(555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 555_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 555_f64),
        &green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 555_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 555_f64),
        &red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343_f64, 554_f64, 332_f64),
        Vec3::new(-130_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, -105_f64),
        &light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0_f64, 0_f64, 0_f64),
        Vec3::new(555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 555_f64),
        &white,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555_f64, 555_f64, 555_f64),
        Vec3::new(-555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, -555_f64),
        &white,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0_f64, 0_f64, 555_f64),
        Vec3::new(555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 555_f64, 0_f64),
        &white,
    )));
    let mut box1: Arc<dyn Hittable> = Arc::new(Quad::create_box(
        Point3::zeroes(),
        Point3::new(165_f64, 330_f64, 165_f64),
        &white,
    ));
    let mut box2: Arc<dyn Hittable> = Arc::new(Quad::create_box(
        Point3::zeroes(),
        Point3::new(165_f64, 165_f64, 165_f64),
        &white,
    ));

    box1 = Arc::new(RotateY::new(&box1, 15_f64));
    box1 = Arc::new(Translate::new(&box1, Vec3::new(265_f64, 0_f64, 295_f64)));
    box2 = Arc::new(RotateY::new(&box2, -18_f64));
    box2 = Arc::new(Translate::new(&box2, Vec3::new(130_f64, 0_f64, 65_f64)));
    world.add(box1);
    world.add(box2);

    cam.render_png(&world, "images/standard_cornell_box.png")
}

pub fn cornell_smoke() -> Result<()> {
    let mut world = HittableList::new();
    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(7_f64, 7_f64, 7_f64)));
    let cam = Camera::new(
        Some(1_f64),
        Some(600),
        Some(200),
        Some(50),
        40_f64,
        &Point3::new(278_f64, 278_f64, -800_f64),
        &Point3::new(278_f64, 278_f64, 0_f64),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::zeroes(),
    );

    world.add(Arc::new(Quad::new(
        Point3::new(555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 555_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 555_f64),
        &green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 555_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 555_f64),
        &red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113_f64, 554_f64, 127_f64),
        Vec3::new(330_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 305_f64),
        &light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0_f64, 0_f64, 0_f64),
        Vec3::new(555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 555_f64),
        &white,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555_f64, 555_f64, 555_f64),
        Vec3::new(-555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, -555_f64),
        &white,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0_f64, 0_f64, 555_f64),
        Vec3::new(555_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 555_f64, 0_f64),
        &white,
    )));
    let mut box1: Arc<dyn Hittable> = Arc::new(Quad::create_box(
        Point3::zeroes(),
        Point3::new(165_f64, 330_f64, 165_f64),
        &white,
    ));
    let mut box2: Arc<dyn Hittable> = Arc::new(Quad::create_box(
        Point3::zeroes(),
        Point3::new(165_f64, 165_f64, 165_f64),
        &white,
    ));

    box1 = Arc::new(RotateY::new(&box1, 15_f64));
    box1 = Arc::new(Translate::new(&box1, Vec3::new(265_f64, 0_f64, 295_f64)));
    box2 = Arc::new(RotateY::new(&box2, -18_f64));
    box2 = Arc::new(Translate::new(&box2, Vec3::new(130_f64, 0_f64, 65_f64)));
    world.add(Arc::new(ConstantMedium::new(&box1, 0.01, Color::zeroes())));
    world.add(Arc::new(ConstantMedium::new(
        &box2,
        0.01,
        Color::new(1_f64, 1_f64, 1_f64),
    )));

    cam.render_png(&world, "images/cornell_box_with_blocks_of_smoke.png")
}

pub fn ray_tracing_the_next_week(
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
) -> Result<()> {
    let mut boxes1 = HittableList::new();
    let mut boxes2 = HittableList::new();
    let mut world = HittableList::new();
    let earthmap: Arc<dyn Texture> = Arc::new(ImageTexture::new("earthmap.jpg")?);
    let pertext: Arc<dyn Texture> = Arc::new(NoiseTexture::new(0.2));
    let ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    let light: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(7_f64, 7_f64, 7_f64)));
    let sphere_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    let emat: Arc<dyn Material> = Arc::new(Lambertian::from(&earthmap));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let boundaries: [Arc<dyn Hittable>; 2] = [
        Arc::new(Sphere::new(
            Point3::new(360_f64, 150_f64, 145_f64),
            70_f64,
            Arc::new(Dielectric::new(1.5)),
        )),
        Arc::new(Sphere::new(
            Point3::zeroes(),
            5e3,
            Arc::new(Dielectric::new(1.5)),
        )),
    ];
    let center1 = Point3::new(400_f64, 400_f64, 200_f64);
    let center2 = center1 + Vec3::new(30_f64, 0_f64, 0_f64);
    let boxes_per_sides: i32 = 20;
    let ns = 1000;
    let cam = Camera::new(
        Some(1_f64),
        Some(image_width),
        Some(samples_per_pixel),
        Some(max_depth),
        40_f64,
        &Point3::new(478_f64, 278_f64, -600_f64),
        &Point3::new(278_f64, 278_f64, 0_f64),
        &Vec3::new(0_f64, 1_f64, 0_f64),
        0_f64,
        1e1,
        Color::zeroes(),
    );

    // Floor
    (0..boxes_per_sides.pow(2)).into_iter().for_each(|ij| {
        let i = ij / boxes_per_sides;
        let j = ij % boxes_per_sides;
        let w = 1e2;
        let xyz_0 = [-1e3 + i as f64 * w, 0_f64, -1e3 + j as f64 * w];
        let xyz_1 = [
            xyz_0[0] + w,
            utils::random_in_range(&Interval::new(1_f64, 101_f64)),
            xyz_0[2] + w,
        ];

        boxes1.add(Arc::new(Quad::create_box(
            Point3::new(xyz_0[0], xyz_0[1], xyz_0[2]),
            Point3::new(xyz_1[0], xyz_1[1], xyz_1[2]),
            &ground,
        )))
    });
    world.add(Arc::new(BvhNode::from(&boxes1)?));
    world.add(Arc::new(Quad::new(
        Point3::new(123_f64, 554_f64, 147_f64),
        Vec3::new(300_f64, 0_f64, 0_f64),
        Vec3::new(0_f64, 0_f64, 265_f64),
        &light,
    )));

    // Cube filled with spheres
    (0..ns).into_iter().for_each(|_| {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_in_range(&Interval::new(0_f64, 165_f64)),
            1e1,
            white.clone(),
        )));
    });
    let cube: Arc<dyn Hittable> = Arc::new(BvhNode::from(&boxes2)?);
    let cube: Arc<dyn Hittable> = Arc::new(RotateY::new(&cube, 15_f64));
    world.add(Arc::new(Translate::new(
        &cube,
        Vec3::new(-1e2, 27e1, 395_f64),
    )));

    // Moving sphere
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50_f64,
        sphere_material,
    )));

    // Glass sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(26e1, 15e1, 145_f64),
        5e1,
        Arc::new(Dielectric::new(1.5)),
    )));

    // Metal sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(0_f64, 15e1, 145_f64),
        5e1,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1e0)),
    )));

    // Boundaries
    world.add(boundaries[0].clone());
    world.add(Arc::new(ConstantMedium::new(
        &boundaries[0],
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    world.add(Arc::new(ConstantMedium::new(
        &boundaries[1],
        0.0001,
        Color::new(1_f64, 1_f64, 1_f64),
    )));

    // Earth sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(400_f64, 200_f64, 400_f64),
        1e2,
        emat,
    )));

    // Perlin texture sphere
    world.add(Arc::new(Sphere::new(
        Point3::new(220_f64, 280_f64, 300_f64),
        8e1,
        Arc::new(Lambertian::from(&pertext)),
    )));

    cam.render_png(&world, "images/RayTracingTheNextWeek-final_scene.png")
}
