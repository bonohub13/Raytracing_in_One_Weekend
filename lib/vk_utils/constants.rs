mod _constants {
    use crate::VkValidationInfo;
    use ash::vk::make_api_version;

    pub const VK_VALIDATION_LAYER_NAMES: VkValidationInfo = VkValidationInfo {
        is_enable: true,
        required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
    };

    pub const APPLICATION_NAME: &str = "Ray Tracing in One Weekend";
    pub const APPLICATION_VERSION: u32 = make_api_version(0, 1, 0, 0);

    pub const ENGINE_NAME: &str = "NO ENGINE";
    pub const ENGINE_VERSION: u32 = make_api_version(0, 1, 0, 0);
}

pub use _constants::VK_VALIDATION_LAYER_NAMES; // Vulkan Validation Layers

pub use _constants::{
    // Application stuff
    APPLICATION_NAME,    // Application name
    APPLICATION_VERSION, // Application version
};

pub use _constants::{
    // Engine stuff
    ENGINE_NAME,    //Engine name
    ENGINE_VERSION, // Engine version
};
