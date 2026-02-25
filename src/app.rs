// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    core::{self, State},
    frame_limiter::FrameLimiter,
};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
pub struct PathTracer {
    window: Option<Arc<Window>>,
    frame_limit: Option<FrameLimiter>,
    state: Option<State>,
}

impl PathTracer {
    fn handle_keypress(key: KeyCode, event_loop: &ActiveEventLoop) {
        match key {
            KeyCode::Escape | KeyCode::KeyQ => event_loop.exit(),
            _ => (),
        }
    }
}

impl ApplicationHandler for PathTracer {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        const WINDOW_TITLE: &str = "Ray Tracing in One Weekend (rust+ash)";
        const INITIAL_WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(1280, 800);

        if self.window.is_none() && self.frame_limit.is_none() && self.state.is_none() {
            let attr = winit::window::WindowAttributes::default()
                .with_title(WINDOW_TITLE)
                .with_inner_size(INITIAL_WINDOW_SIZE);
            let window = Arc::new(
                event_loop
                    .create_window(attr)
                    .expect("Failed to create window"),
            );
            let frame_limit = FrameLimiter::new(Some(30));
            let state = State::new(&core::StateDescriptor {
                window: window.clone(),
            })
            .expect("Failed to create Vulkan State object");

            self.window = Some(window);
            self.frame_limit = Some(frame_limit);
            self.state = Some(state);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let (Some(window), Some(vk_state), Some(limiter)) = (
            self.window.as_ref(),
            self.state.as_mut(),
            self.frame_limit.as_mut(),
        ) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(key),
                            ..
                        },
                    ..
                } => Self::handle_keypress(key, event_loop),
                WindowEvent::Resized(_) => {
                    if let Err(err) = vk_state.resize() {
                        eprintln!("{err:?}");
                        event_loop.exit();
                    }
                }
                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                    if let Err(err) = vk_state.draw_frame() {
                        eprintln!("{err:?}");
                        event_loop.exit();
                    }
                    limiter.wait_frame();
                }
                _ => (),
            }
        }
    }
}
