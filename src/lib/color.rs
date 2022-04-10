use crate::clamp;

pub fn write_color(pixel_color: crate::Color) {
    let pc = pixel_color * 255.999;
    let x = pc.x() as i64;
    let y = pc.y() as i64;
    let z = pc.z() as i64;

    println!("{} {} {}", x, y, z);
}

pub fn write_color_spp(pixel_color: crate::Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    let scale = 1.0 / samples_per_pixel as f64;

    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i64,
        (256.0 * clamp(g, 0.0, 0.999)) as i64,
        (256.0 * clamp(b, 0.0, 0.999)) as i64
    );
}
