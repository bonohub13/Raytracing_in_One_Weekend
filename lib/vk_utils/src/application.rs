pub struct App {
    instance: crate::Instance,
    debug_utils_messenger: crate::DebugUtilsMessenger,
    surface: crate::Surface,
}

impl App {
    pub fn new(
        appbase: &crate::AppBase,
        window: &crate::window::Window,
        enable_validation_layers: bool,
    ) -> Result<Self, String> {
        use ash::vk;
        use std::ffi::CString;

        let validation_layers: Vec<CString> = if enable_validation_layers {
            vec!["VK_LAYER_KHRONOS_validation"]
                .iter()
                .map(|&layer| CString::new(layer).unwrap())
                .collect()
        } else {
            vec![]
        };
        let raw_validation_layers = validation_layers
            .iter()
            .map(|layer| layer.as_ptr())
            .collect();

        let instance = crate::Instance::new(
            appbase,
            window,
            &raw_validation_layers,
            ash::vk::API_VERSION_1_2,
        )?;
        let debug_utils_messenger = if enable_validation_layers {
            crate::DebugUtilsMessenger::new(
                &appbase.entry,
                &instance,
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )?
        } else {
            crate::DebugUtilsMessenger::null(&appbase.entry, &instance)
        };

        let surface = crate::Surface::new(appbase, &instance, window)?;

        Ok(Self {
            instance,
            debug_utils_messenger,
            surface,
        })
    }

    pub fn extensions(&self) -> Vec<ash::vk::ExtensionProperties> {
        self.instance.extensions()
    }

    pub fn layers(&self) -> Vec<ash::vk::LayerProperties> {
        self.instance.layers()
    }

    pub fn physical_devices(&self) -> Vec<ash::vk::PhysicalDevice> {
        self.instance.physical_devices()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        crate::Surface::cleanup(&mut self.surface);
        crate::debug::DebugUtilsMessenger::cleanup(&mut self.debug_utils_messenger);
        crate::Instance::cleanup(&mut self.instance);
    }
}
