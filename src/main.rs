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

    let window_config = vk_utils::window::WindowConfig {
        width: 1920,
        height: 1080,
        resizable: false,
    };
    let window = vk_utils::window::Window::new(&app_base, &window_config)?;

    let mut engine = vk_utils::Engine::new(&app_base, &window)?;

    app_base.run(&mut engine, &window);

    Ok(())
}
