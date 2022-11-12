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

    let mut window = vk_utils::window::Window::new(&config)?;
    let vk_app = vk_utils::application::App::new(&window, true)?;

    window.run();

    Ok(())
}
