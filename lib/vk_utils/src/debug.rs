pub unsafe extern "system" fn vulkan_debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> ash::vk::Bool32 {
    use ash::vk;
    use std::ffi::CStr;

    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[VERBOSE]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[INFO]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[WARNING]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[ERROR]",
        _ => "[UNKNOWN]",
    };

    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[GENERAL]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[VALIDATION]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[PERFORMANCE]",
        _ => "[UNKNOWN]",
    };

    let message = CStr::from_ptr((*p_callback_data).p_message);

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log::info!("{}{}{:?}", severity, types, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log::warn!("{}{}{:?}", severity, types, message);
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log::error!("{}{}{:?}", severity, types, message)
        }
        _ => log::info!("{}{}{:?}", severity, types, message),
    }

    vk::FALSE
}

pub struct DebugUtilsMessenger {
    pub debug_utils_loader: ash::extensions::ext::DebugUtils,
    pub debug_callback: ash::vk::DebugUtilsMessengerEXT,
}

impl DebugUtilsMessenger {
    pub fn new(
        entry: &ash::Entry,
        instance: &crate::Instance,
        severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    ) -> Result<Self, String> {
        use ash::{extensions::ext::DebugUtils, vk};
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating DebugUtilsMessenger");

        if instance.validation_layers().is_empty() {
            return Err(String::from("no available validation layers"));
        }

        let debug_utils_loader = DebugUtils::new(entry, &instance.instance);

        log::info!("created DebugUtils loader");

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(severity)
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(vulkan_debug_callback))
            .build();

        log::info!("creating debug callback");

        let debug_callback_sg = {
            let debug_callback = unsafe {
                debug_utils_loader
                    .create_debug_utils_messenger(&create_info, None)
                    .map_err(|_| String::from("failed to set up debug messenger"))?
            };

            guard(debug_callback, |callback| {
                log::info!("debug callback scopeguard");

                unsafe {
                    debug_utils_loader.destroy_debug_utils_messenger(callback, None);
                }
            })
        };

        log::info!("created debug callback");

        Ok(Self {
            debug_callback: ScopeGuard::into_inner(debug_callback_sg),
            debug_utils_loader,
        })
    }

    pub fn null(entry: &ash::Entry, instance: &crate::Instance) -> Self {
        use ash::{extensions::ext::DebugUtils, vk};

        let debug_utils_loader = DebugUtils::new(entry, &instance.instance);

        Self {
            debug_utils_loader,
            debug_callback: vk::DebugUtilsMessengerEXT::null(),
        }
    }

    pub fn cleanup(debug_utils: &mut Self) {
        log::info!("performing cleanup for DebugUtilsMessenger");

        unsafe {
            debug_utils
                .debug_utils_loader
                .destroy_debug_utils_messenger(debug_utils.debug_callback, None);
        }
    }
}
