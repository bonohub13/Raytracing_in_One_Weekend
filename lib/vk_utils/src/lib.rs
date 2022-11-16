pub mod constants {
    // Application stuff
    pub const APPLICATION_NAME: &str = "Ray Tracing in One Weekend";
    pub const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);

    // Engine stuff
    pub const ENGINE_NAME: &str = "No Engine";
    pub const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);
}

pub mod application;
pub mod utils;
pub mod window;

mod debug;
mod instance;
mod surface;

pub use debug::{vulkan_debug_callback, DebugUtilsMessenger};
pub use instance::Instance;
pub use surface::Surface;

pub struct AppBase {
    pub entry: ash::Entry,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl AppBase {
    pub fn new() -> Self {
        use winit::event_loop::EventLoop;

        Self {
            entry: ash::Entry::linked(),
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(&mut self, window: &window::Window) {
        use winit::platform::run_return::EventLoopExtRunReturn;

        self.event_loop.run_return(|event, _, control_flow| {
            use winit::event::{Event, WindowEvent};

            control_flow.set_poll();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => {}
                },
                Event::MainEventsCleared => {
                    // Do not do anything if window is minimized
                    // Could not use self.is_minimized() due to event_loop
                    // being borrowed once as mutable
                    if window.is_minimized() {
                        return;
                    }
                    // Draw here
                }
                _ => {}
            }
        });
    }
}
