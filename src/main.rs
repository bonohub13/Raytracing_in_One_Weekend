mod scenes;

use anyhow::{bail, Result};
use scenes::*;

fn main() -> Result<()> {
    match 4 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        val => bail!("Option is not available. ({})", val),
    }
}
