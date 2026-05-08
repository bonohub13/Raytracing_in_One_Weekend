// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{frame_limiter::FrameLimiter, renderer::Renderer};
use rtiow::VkState;
use std::{ffi::CStr, sync::Arc};
use winit::{
    application::ApplicationHandler, event::WindowEvent, keyboard::KeyCode, window::Window,
};

#[derive(Default)]
pub struct RtApp {
    renderer: Option<Renderer>,
    state: Option<VkState>,
    window: Option<Arc<Window>>,
    frame_limiter: Option<FrameLimiter>,
}

impl RtApp {
    const WINDOW_TITLE: &str = "Ray Tracing in One Weekend";
    const WINDOW_TITLE_CSTR: &CStr = c"Ray Tracing in One Weekend";
    const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

    fn handle_keyboard(
        event_loop: &winit::event_loop::ActiveEventLoop,
        key: KeyCode,
        state: &VkState,
    ) {
        #[allow(clippy::single_match)]
        match key {
            KeyCode::Escape => {
                event_loop.exit();
                while let Err(err) = state.device_wait_idle() {
                    eprintln!("{err}");
                }
            }
            _ => (),
        }
    }
}

impl ApplicationHandler for RtApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.frame_limiter.is_none()
            && self.window.is_none()
            && self.state.is_none()
            && self.renderer.is_none()
        {
            self.frame_limiter = Some(FrameLimiter::new(Some(60)));
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

            if let (Some(window), Some(state)) = (self.window.clone(), self.state.as_ref()) {
                self.renderer = match Renderer::new(window, state) {
                    Ok(renderer) => Some(renderer),
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
        if (self.window.is_some() && self.state.is_none())
            || (self.window.is_none() && self.state.is_some())
        {
            event_loop.exit();
        }

        if let (Some(frame_limiter), Some(window), Some(state), Some(renderer)) = (
            self.frame_limiter.as_mut(),
            self.window.clone(),
            self.state.as_mut(),
            self.renderer.as_mut(),
        ) {
            match window_event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                    while let Err(err) = state.device_wait_idle() {
                        eprintln!("{err}");
                    }
                }
                WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            state: winit::event::ElementState::Pressed,
                            physical_key: winit::keyboard::PhysicalKey::Code(key),
                            ..
                        },
                    ..
                } => Self::handle_keyboard(event_loop, key, state),
                WindowEvent::Resized(physical_size) => {
                    if let Err(err) = renderer.resize(window, state, physical_size) {
                        eprintln!("{err}");
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Err(err) = renderer.draw_frame(window, state) {
                        eprintln!("{err}");
                    }

                    frame_limiter.wait_frame();
                }
                _ => (),
            }
        }
    }
}
