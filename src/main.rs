// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod app;
mod frame_limiter;
mod renderer;

use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = app::RtApp::default();

    event_loop.set_control_flow(ControlFlow::Poll);
    if let Err(err) = event_loop.run_app(&mut app) {
        Err(err.into())
    } else {
        Ok(())
    }
}
