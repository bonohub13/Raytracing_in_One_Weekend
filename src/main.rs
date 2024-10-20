mod scenes;

use anyhow::{bail, Result};
use scenes::*;

fn main() -> Result<()> {
    match 3 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        val => bail!("Option is not available. ({})", val),
    }
}
