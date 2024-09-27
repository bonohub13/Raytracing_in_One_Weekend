use super::Color;

pub fn write_color(pixel_color: &Color) -> [i32; 3] {
    let mut rgb = [0; 3];

    rgb.iter_mut()
        .enumerate()
        .for_each(|(i, e_i)| *e_i = (255.999 * pixel_color[i]) as i32);

    rgb
}
