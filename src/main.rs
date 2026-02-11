mod app;
mod core;

use anyhow::Result;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    let mut app = app::PathTracer::default();
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
