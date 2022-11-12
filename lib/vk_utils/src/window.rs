#[derive(Copy, Clone)]
pub struct WindowConfig {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub resizable: bool,
}

pub struct Window {
    config: WindowConfig,
    event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
}

impl Window {
    pub fn new(config: &WindowConfig) -> Result<Self, String> {
        use winit::{
            dpi::LogicalSize,
            event_loop::EventLoop,
            window::{Fullscreen, WindowBuilder},
        };

        log::info!("creating window");

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_fullscreen(if config.fullscreen {
                // Fullscreen to primary monitor
                Some(Fullscreen::Borderless(None))
            } else {
                None
            })
            .with_resizable(config.resizable)
            .build(&event_loop)
            .map_err(|_| String::from("failed to create window"))?;

        log::info!("created window");

        Ok(Self {
            config: config.clone(),
            event_loop,
            window,
        })
    }

    pub fn window_config(&self) -> WindowConfig {
        self.config
    }

    pub fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        use raw_window_handle::HasRawDisplayHandle;

        self.window.raw_display_handle()
    }

    pub fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        use raw_window_handle::HasRawWindowHandle;

        self.window.raw_window_handle()
    }

    pub fn framebuffer_size(&self) -> ash::vk::Extent2D {
        use ash::vk;

        // inner size only contains where you can draw
        // no title bars
        let framebuffer_size = self.window.inner_size();

        vk::Extent2D {
            width: framebuffer_size.width,
            height: framebuffer_size.height,
        }
    }

    pub fn window_size(&self) -> ash::vk::Extent2D {
        use ash::vk;

        // outer_size contains title bars and everything
        let window_size = self.window.outer_size();

        vk::Extent2D {
            width: window_size.width,
            height: window_size.height,
        }
    }

    pub fn is_minimized(&self) -> bool {
        let window_size = self.window_size();

        window_size.width == 0 && window_size.height == 0
    }

    pub fn run(&mut self) {
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
                    if self.window.inner_size().width == 0 && self.window.inner_size().height == 0 {
                        return;
                    }
                    // Draw here
                }
                _ => {}
            }
        });
    }
}
