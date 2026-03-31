// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use rtiow::VkState;
use std::{ffi::CStr, sync::Arc};
use winit::{
    application::ApplicationHandler, event::WindowEvent, keyboard::KeyCode, window::Window,
};

#[derive(Default)]
pub struct RtApp {
    window: Option<Arc<Window>>,
    state: Option<VkState>,
}

impl RtApp {
    const WINDOW_TITLE: &str = "Ray Tracing in One Weekend";
    const WINDOW_TITLE_CSTR: &CStr = c"Ray Tracing in One Weekend";
    const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

    fn handle_keyboard(event_loop: &winit::event_loop::ActiveEventLoop, key: KeyCode) {
        #[allow(clippy::single_match)]
        match key {
            KeyCode::Escape => event_loop.exit(),
            _ => (),
        }
    }
}

impl ApplicationHandler for RtApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            self.window = if let Some(monitor) = event_loop.primary_monitor() {
                let attr = winit::window::WindowAttributes::default()
                    .with_title(Self::WINDOW_TITLE)
                    .with_inner_size(monitor.size())
                    .with_transparent(false)
                    .with_resizable(false);

                match event_loop.create_window(attr) {
                    Ok(window) => Some(Arc::new(window)),
                    Err(err) => {
                        eprintln!("{err}");

                        None
                    }
                }
            } else {
                None
            };

            if let Some(window) = self.window.clone()
                && let Ok(app_version) = rtiow::util::parse_version_from_str(Self::APP_VERSION)
                    .map_err(|err| {
                        eprintln!("{err:?}");
                        err
                    })
            {
                self.state = match VkState::new(&rtiow::VkStateDesc {
                    window,
                    app_name: Self::WINDOW_TITLE_CSTR,
                    app_version,
                }) {
                    Ok(state) => Some(state),
                    Err(err) => {
                        eprintln!("{err}");
                        None
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        window_event: winit::event::WindowEvent,
    ) {
        if let Some(_window) = self.window.as_ref() {
            match window_event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            state: winit::event::ElementState::Pressed,
                            physical_key: winit::keyboard::PhysicalKey::Code(key),
                            ..
                        },
                    ..
                } => Self::handle_keyboard(event_loop, key),
                WindowEvent::Resized(_physical_size) => {}
                WindowEvent::RedrawRequested => {}
                _ => (),
            }
        }
    }
}
