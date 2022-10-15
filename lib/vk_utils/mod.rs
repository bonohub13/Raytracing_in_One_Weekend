mod common;

pub mod constants;
pub mod debug;
pub mod tools;

pub mod device;
pub mod image;
pub mod queue;
pub mod surface;
pub mod swapchain;

pub use common::create_instance;

pub struct VkValidationInfo {
    pub is_enable: bool,
    pub required_validation_layers: [&'static str; 1],
}

pub struct VkDeviceExtension {
    pub names: [&'static str; 1],
}
