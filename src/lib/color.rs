use crate::clamp;
use crate::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> String {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    format!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0., 0.999)) as i32,
        (256.0 * clamp(g, 0., 0.999)) as i32,
        (256.0 * clamp(b, 0., 0.999)) as i32,
    )
}
