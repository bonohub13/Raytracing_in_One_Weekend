mod scenes;

use anyhow::{bail, Result};
use scenes::*;

fn main() -> Result<()> {
    match 6 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        val => bail!("Option is not available. ({})", val),
    }
}
