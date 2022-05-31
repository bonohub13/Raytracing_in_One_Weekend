use rt_utils::renderer::*;

// Image
const IMAGE_WIDTH: i64 = 256;
const IMAGE_HEIGHT: i64 = 256;

fn main() {
    ppm_p3(IMAGE_WIDTH, IMAGE_HEIGHT);

    render(IMAGE_WIDTH, IMAGE_HEIGHT);
}
