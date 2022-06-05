// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let world = rt_utils::random_scene();

    // Camera
    let look_from = rt_utils::Point3::new(13., 2., 3.);
    let look_at = rt_utils::Point3::default();
    let vup = rt_utils::Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = rt_utils::Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
