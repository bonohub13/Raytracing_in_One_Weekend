mod _constants {
    use crate::{VkDeviceExtension, VkValidationInfo};
    use ash::vk::make_api_version;

    pub const WINDOW_TITLE: &str = "Ray Tracing in One Weekend";
    pub const WINDOW_WIDTH: u32 = 800;
    pub const WINDOW_HEIGHT: u32 = 600;

    pub const VK_VALIDATION_LAYER_NAMES: VkValidationInfo = VkValidationInfo {
        is_enable: true,
        required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
    };
    pub const VK_DEVICE_EXTENSIONS: VkDeviceExtension = VkDeviceExtension {
        names: ["VK_KHR_swapchain"],
    };

    pub const APPLICATION_NAME: &str = "Ray Tracing in One Weekend";
    pub const APPLICATION_VERSION: u32 = make_api_version(0, 1, 0, 0);

    pub const ENGINE_NAME: &str = "NO ENGINE";
    pub const ENGINE_VERSION: u32 = make_api_version(0, 1, 0, 0);

    pub const VERT_SHADER_PATH: &str = "shaders/spv/vertex3d_vert.spv";
    pub const FRAG_SHADER_PATH: &str = "shaders/spv/vertex3d_frag.spv";
}

pub use _constants::{
    // Window stuff
    WINDOW_HEIGHT, // Window height
    WINDOW_TITLE,  // Window title
    WINDOW_WIDTH,  // Window width
};

pub use _constants::{
    // Vulkan stuff
    VK_DEVICE_EXTENSIONS,      //Vulkan Device Extensions
    VK_VALIDATION_LAYER_NAMES, // Vulkan Validation Layers
};

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

pub use _constants::{
    FRAG_SHADER_PATH, // Fragment shader
    // Shader stuff
    VERT_SHADER_PATH, // Vertex shader
};
