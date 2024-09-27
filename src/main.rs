use anyhow::Result;
use rtiow::{
    buffer::ImageBuffer,
    vec3::{self, Color},
};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;
const EMPTY_SPACES: &'static str = "          ";

fn main() -> Result<()> {
    let mut buffer = vec![[0; 3]; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize];
    let mut writer = ImageBuffer::new("images/image.ppm");

    for j in 0..IMAGE_HEIGHT {
        eprint!(
            "\rScanlines remaining: {}{}",
            IMAGE_HEIGHT - j,
            EMPTY_SPACES
        );
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );

            buffer[(j * IMAGE_WIDTH + i) as usize] = vec3::write_color(&pixel_color);
        }
    }

    writer.set_buffer(&buffer);
    writer.write([IMAGE_WIDTH as usize, IMAGE_HEIGHT as usize])?;

    eprintln!("\r\n\nDone!{}", EMPTY_SPACES.repeat(10));

    Ok(())
}
