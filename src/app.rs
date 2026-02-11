use crate::core::{self, State};
use std::{path::PathBuf, sync::Arc};
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
        if self.window.is_none() {
            let attr = winit::window::WindowAttributes::default()
                .with_title(WINDOW_TITLE)
                .with_inner_size(INITIAL_WINDOW_SIZE);
            let window = Arc::new(
                event_loop
                    .create_window(attr)
                    .expect("Failed to create window"),
            );
            let state = match State::new(&core::StateDescriptor {
                app_name: WINDOW_TITLE,
                window: window.clone(),
                render_shader_desc: core::RenderShaderDescriptor {
                    vertex_shader_path: PathBuf::from("shaders/spv/vertex.spv"),
                    fragment_shader_path: PathBuf::from("shaders/spv/fragment.spv"),
                    vertex_shader_entrypoint: c"main",
                    fragment_shader_entrypoint: c"main",
                },
                compute_shader_desc: core::ComputeShaderDescriptor {
                    compute_shader_path: PathBuf::from("shaders/spv/path_tracer.spv"),
                    compute_shader_entrypoint: c"main",
                },
            }) {
                Ok(state) => state,
                Err(e) => panic!("{e}"),
            };

            self.window = Some(window);
            self.state = Some(state)
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let (Some(window), Some(state)) = (self.window.as_ref(), self.state.as_mut()) {
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
                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                    if let Err(err) = state.draw_frame(window.clone()) {
                        panic!("{err}");
                    }
                }
                WindowEvent::Resized(_) => {
                    if let Err(err) = state.resize(window.clone()) {
                        panic!("{err}");
                    }
                }
                _ => (),
            }
            if let Err(err) = state.device_wait_idle() {
                panic!("{err}");
            }
        }
    }
}
