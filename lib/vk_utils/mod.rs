mod common;

pub mod constants;
pub mod debug;
pub mod tools;

pub use common::create_instance;

pub struct VkValidationInfo {
    pub is_enable: bool,
    pub required_validation_layers: [&'static str; 1],
}
