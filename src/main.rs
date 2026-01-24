#![windows_subsystem = "windows"]

mod app;
mod base;
mod core;
mod utils;

use anyhow::Result;
use app::App;
use base::Config;
use winit::event_loop::{ControlFlow, EventLoop};

#[cfg(target_os = "windows")]
use winapi::um::wincon::{ATTACH_PARENT_PROCESS, AttachConsole};

fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    let event_loop = EventLoop::new()?;
    let config = Config { fps: 30 };
    let mut app = App::default();

    app.set_config(&config);
    event_loop.set_control_flow(ControlFlow::Poll);
    match event_loop.run_app(&mut app) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{e}");

            Err(e.into())
        }
    }
}
