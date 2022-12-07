use chrono::prelude::Local;
use logger::Logger;
use std::path::Path;

fn main() -> Result<(), String> {
    let mut logger = Logger::new();
    let now = Local::now();
    let filename = format!("./log/rtweekend_{}.log", now.format("%Y-%m-%d_%H-%M-%S"));

    logger.create_logfile(Path::new(filename.as_str()))?;
    logger.init().unwrap();

    let mut app_base = vk_utils::AppBase::new();

    let settings = vk_utils::VkSettings {
        window_width: 800,
        window_height: 600,
        compute_shader_file: "",
        compute_shader_group_size_x: 800,
        compute_shader_group_size_y: 600,
    };
    let window_config = vk_utils::window::WindowConfig {
        width: 800,
        height: 600,
        resizable: false,
    };
    let window = vk_utils::window::Window::new(&app_base, &window_config)?;

    let engine = vk_utils::Engine::new(&settings, &app_base, &window)?;

    app_base.run();

    Ok(())
}
