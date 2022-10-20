mod window;

pub use window::Window;

#[derive(Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub resizable: bool,
}

impl WindowConfig {
    pub fn new(title: &str, width: u32, height: u32, fullscreen: bool, resizable: bool) -> Self {
        Self {
            title: String::from(title),
            width,
            height,
            fullscreen,
            resizable,
        }
    }
}
