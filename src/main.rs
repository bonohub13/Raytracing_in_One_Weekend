use anyhow::Result;
use rtiow::{
    buffer::ImageBuffer,
    ray::Ray,
    vec3::{self, Color},
};

const EMPTY_SPACES: &'static str = "          ";

fn ray_color(r: &Ray) -> Color {
    Color::zeroes()
}

fn main() -> Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = {
        let image_height = (image_width as f64 / aspect_ratio) as i32;

        if image_height < 1 {
            1
        } else {
            image_height
        }
    };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let mut buffer = vec![[0; 3]; (image_width * image_height) as usize];
    let mut writer = ImageBuffer::new("images/image.ppm");

    for j in 0..image_height {
        eprint!(
            "\rScanlines remaining: {}{}",
            image_height - j,
            EMPTY_SPACES
        );
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            buffer[(j * image_width + i) as usize] = vec3::write_color(&pixel_color);
        }
    }

    writer.set_buffer(&buffer);
    writer.write([IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize])?;

    eprintln!("\r\n\nDone!{}", EMPTY_SPACES.repeat(10));

    Ok(())
}
