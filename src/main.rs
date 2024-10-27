mod scenes;

use anyhow::{bail, Result};
use scenes::*;

fn main() -> Result<()> {
    match 5 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        val => bail!("Option is not available. ({})", val),
    }
}
