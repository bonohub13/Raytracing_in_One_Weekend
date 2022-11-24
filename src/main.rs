use ash::vk;
use chrono::prelude::Local;
use logger::Logger;
use std::path::Path;

fn main() -> Result<(), String> {
    let mut logger = Logger::new();
    let now = Local::now();
    let filename = format!("./log/rtweekend_{}.log", now.format("%Y-%m-%d_%H-%M-%S"));

    logger.create_logfile(Path::new(filename.as_str()))?;
    logger.init().unwrap();

    let config = vk_utils::window::WindowConfig {
        title: "Vulkan Window",
        width: 800,
        height: 600,
        fullscreen: false,
        resizable: true,
    };

    let enable_physical_device_features = vk_utils::PhysicalDeviceRequiredFeatures {
        geometry_shader_support: true,
        ray_tracing_support: false,
        graphics_queue_support: true,
    };

    let mut app_base = vk_utils::AppBase::new();

    let window = vk_utils::window::Window::new(&app_base, &config)?;

    let vk_app = vk_utils::application::App::new(
        &app_base,
        &window,
        vk::PresentModeKHR::default(),
        Some(&enable_physical_device_features),
        true,
    )?;

    app_base.run(&window);

    Ok(())
}
