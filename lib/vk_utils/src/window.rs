pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

pub struct Window {
    window: winit::window::Window,
}

impl Window {
    const WINDOW_NAME: &'static str = "Ray Tracing in Vulkan (Compute)";

    pub fn new(app_base: &crate::AppBase, config: &WindowConfig) -> Result<Self, String> {
        use winit::dpi::LogicalSize;
        use winit::window::WindowBuilder;

        let window = WindowBuilder::new()
            .with_title(Self::WINDOW_NAME)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
            .build(app_base.event_loop())
            .map_err(|_| String::from("failed to build new window"))?;

        Ok(Self { window })
    }

    pub fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        use raw_window_handle::HasRawDisplayHandle;

        self.window.raw_display_handle()
    }

    pub fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        use raw_window_handle::HasRawWindowHandle;

        self.window.raw_window_handle()
    }

    pub fn window_size(&self) -> (u32, u32) {
        let window_size = self.window.inner_size();

        (window_size.width, window_size.height)
    }
}
