// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use rtiow::{GraphicsPipeline, PipelineLayout, VkState};
use std::{ffi::CStr, sync::Arc};
use winit::{
    application::ApplicationHandler, event::WindowEvent, keyboard::KeyCode, window::Window,
};

#[derive(Default)]
pub struct RtApp {
    graphics_pipeline: Option<GraphicsPipeline>,
    pipeline_layout: Option<PipelineLayout>,
    state: Option<VkState>,
    window: Option<Arc<Window>>,
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
        if self.window.is_none() && self.state.is_none() && self.graphics_pipeline.is_none() {
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

            if let Some(state) = self.state.as_ref() {
                self.pipeline_layout = match PipelineLayout::new(state) {
                    Ok(layout) => Some(layout),
                    Err(err) => {
                        eprintln!("{err}");
                        None
                    }
                }
            }

            if let (Some(state), Some(layout)) =
                (self.state.as_ref(), self.pipeline_layout.as_ref())
            {
                self.graphics_pipeline = match GraphicsPipeline::new(state, layout) {
                    Ok(pipeline) => Some(pipeline),
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

        if let (Some(_window), Some(state)) = (self.window.as_ref(), self.state.as_mut()) {
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
                WindowEvent::Resized(_physical_size) => {}
                WindowEvent::RedrawRequested => {}
                _ => (),
            }
        }
    }
}
