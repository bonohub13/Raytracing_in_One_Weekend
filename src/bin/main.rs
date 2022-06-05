// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let mut world = rt_utils::HittableList::default();

    let material_ground = rt_utils::Lambertian::new(rt_utils::Color::new(0.8, 0.8, 0.0));
    let material_center = rt_utils::Lambertian::new(rt_utils::Color::new(0.7, 0.3, 0.3));
    let material_left = rt_utils::Metal::new(rt_utils::Color::new(0.8, 0.8, 0.8));
    let material_right = rt_utils::Metal::new(rt_utils::Color::new(0.8, 0.6, 0.2));

    world.push(rt_utils::Sphere::new(
        rt_utils::Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    ));
    world.push(rt_utils::Sphere::new(
        rt_utils::Point3::new(0., 0., -1.),
        0.5,
        material_center,
    ));
    world.push(rt_utils::Sphere::new(
        rt_utils::Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    ));
    world.push(rt_utils::Sphere::new(
        rt_utils::Point3::new(1., 0., -1.),
        0.5,
        material_right,
    ));

    // Camera
    let cam = rt_utils::Camera::new();

    rt_utils::ppm_p3(IMAGE_WIDTH, IMAGE_HEIGHT);
    rt_utils::render(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        &world,
        &cam,
    );
}
