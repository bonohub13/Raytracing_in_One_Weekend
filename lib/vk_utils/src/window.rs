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
    pub window: winit::window::Window,
}

impl Window {
    pub fn new(appbase: &crate::AppBase, config: &WindowConfig) -> Result<Self, String> {
        use winit::{
            dpi::LogicalSize,
            window::{Fullscreen, WindowBuilder},
        };

        log::info!("creating window");

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
            .build(&appbase.event_loop)
            .map_err(|_| String::from("failed to create window"))?;

        log::info!("created window");

        Ok(Self {
            config: config.clone(),
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
}
