use super::Color;
use crate::interval::Interval;

static INTENSITY: Interval = Interval {
    min: 0_f64,
    max: 0.999,
};

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0_f64 {
        linear_component.sqrt()
    } else {
        0_f64
    }
}

pub fn write_color(pixel_color: &Color) -> [i32; 3] {
    let mut rgb = [0; 3];

    rgb.iter_mut().enumerate().for_each(|(i, e_i)| {
        *e_i = (256_f64 * INTENSITY.clamp(linear_to_gamma(pixel_color[i]))) as i32
    });

    rgb
}

#[test]
fn test_write_color() {
    let pixel_color = Color::new(0.01, 0.999, 0.9991);
    let result = write_color(&pixel_color);
    let target = [25, 255, 255];

    assert_eq!(target, result)
}

#[test]
fn test_linear_to_gamma() {
    let linear_component = [0_f64, 4_f64];
    let targets = [0_f64, 2_f64];

    for i in 0..linear_component.len() {
        let gamma = linear_to_gamma(linear_component[i]);

        assert_eq!(targets[i], gamma)
    }
}
