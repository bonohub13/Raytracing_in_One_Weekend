// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    let mode = 0;

    let mut look_from = rt_utils::Point3::new(13., 2., 3.);
    let mut look_at = rt_utils::Point3::default();
    let vfov = 20.0;
    let mut aperture = 0.;
    let mut background = rt_utils::Color::default();
    let mut samples_per_pixel = SAMPLES_PER_PIXEL;

    let world = match mode {
        1 => {
            aperture = 0.1;
            background = rt_utils::Color::new(0.7, 0.8, 1.);
            rt_utils::random_scene()
        }
        2 => {
            background = rt_utils::Color::new(0.7, 0.8, 1.);
            rt_utils::two_spheres()
        }
        3 => {
            background = rt_utils::Color::new(0.7, 0.8, 1.);
            rt_utils::two_perlin_spheres()
        }
        4 | _ => {
            samples_per_pixel = 400;
            look_from = rt_utils::Point3::new(26., 3., 6.);
            look_at = rt_utils::Point3::new(0., 2., 0.);
            rt_utils::simple_light()
        }
    };

    // Camera
    let vup = rt_utils::Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let cam = rt_utils::Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        Some(0.),
        Some(1.),
    );

    rt_utils::ppm_p3(IMAGE_WIDTH, IMAGE_HEIGHT);
    rt_utils::render(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        samples_per_pixel,
        MAX_DEPTH,
        &background,
        &world,
        &cam,
    );
}
