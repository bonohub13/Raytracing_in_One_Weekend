// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod app;
mod core;
mod frame_limiter;

use anyhow::Result;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    let mut app = app::PathTracer::default();
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
