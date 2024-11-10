mod scenes;

use anyhow::Result;
use scenes::*;

fn main() -> Result<()> {
    match 10 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(None),
        8 => cornell_smoke(),
        9 => ray_tracing_the_next_week(800, 10000, 40),
        10 => cornell_box(Some("images/cornell_box_revised.png")),
        11 => cornell_box_10spp(),
        _ => ray_tracing_the_next_week(400, 250, 4),
    }
}
