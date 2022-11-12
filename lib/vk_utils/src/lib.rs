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

pub use debug::{vulkan_debug_callback, DebugUtilsMessenger};
pub use instance::Instance;
