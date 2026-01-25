use crate::core::{RtError, State, StateDescriptor};
use anyhow::Result;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};

#[derive(Debug, Clone)]
pub struct Config {
    pub fps: u32,
}

#[derive(Debug)]
pub struct AppBase<'window> {
    window: Arc<Window>,
    state: State<'window>,
    next_frame: Instant,
    fps_limit: Duration,
    fps_update: Instant,
    fps_update_limit: Duration,
}

impl<'window> AppBase<'window> {
    const WINDOW_TITLE: &'static str = "Ray Tracing in One Weekend";
    pub fn new(event_loop: &ActiveEventLoop, config: &Config) -> Result<Self> {
        let window = {
            let monitor_size = if let Some(monitor) = event_loop.primary_monitor() {
                monitor.size()
            } else {
                PhysicalSize::new(1920, 1080)
            };
            let window_size = PhysicalSize::new(
                (monitor_size.width as f64 * 0.75).floor() as u32,
                (monitor_size.height as f64 * 0.75).floor() as u32,
            );
            let attribute = Window::default_attributes()
                .with_title(Self::WINDOW_TITLE)
                .with_transparent(false)
                .with_inner_size(window_size)
                .with_resizable(false)
                .with_resizable(true);
            let window = event_loop.create_window(attribute)?;

            Arc::new(window)
        };
        let desc = StateDescriptor {
            window: window.clone(),
        };
        let state = pollster::block_on(State::new(&desc))?;

        Ok(Self {
            window,
            state,
            next_frame: Instant::now(),
            fps_limit: Duration::from_secs_f64(1f64 / config.fps as f64),
            fps_update: Instant::now(),
            fps_update_limit: Duration::from_secs_f64(1f64),
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.state.resize(size.width, size.height);
    }

    pub fn redraw_frame(&mut self) {
        self.update();
        self.render();
        self.wait_frame();
    }

    fn update(&mut self) {
        self.state.update();
    }

    fn render(&mut self) {
        match self.state.render() {
            Ok(_) => (),
            Err(RtError::GetCurrentTexture(e)) => match e {
                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                    let size = self.window.inner_size();

                    self.state.resize(size.width, size.height);
                }
                _ => {
                    log::error!("Unable to render {e}");
                }
            },
            Err(e) => log::error!("Unknown error ({e})"),
        }
    }

    fn wait_frame(&mut self) {
        let frame_time = Instant::now();

        self.next_frame += self.fps_limit;
        if (self.fps_update - frame_time) > self.fps_update_limit {
            self.fps_update = frame_time;
        }

        std::thread::sleep(self.next_frame - frame_time);
    }
}
