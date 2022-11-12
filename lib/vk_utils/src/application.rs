pub struct App {
    entry: ash::Entry,
    instance: crate::Instance,
    debug_utils_messenger: crate::DebugUtilsMessenger,
}

impl App {
    pub fn new(
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

        let entry = ash::Entry::linked();
        let instance = crate::Instance::new(
            &entry,
            window,
            &raw_validation_layers,
            ash::vk::API_VERSION_1_2,
        )?;
        let debug_utils_messenger = if enable_validation_layers {
            crate::DebugUtilsMessenger::new(
                &entry,
                &instance,
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )?
        } else {
            crate::DebugUtilsMessenger::null(&entry, &instance)
        };

        Ok(Self {
            entry,
            instance,
            debug_utils_messenger,
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
        unsafe {
            self.debug_utils_messenger
                .debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_utils_messenger.debug_callback, None);
            self.instance.instance.destroy_instance(None);
        }
    }
}
