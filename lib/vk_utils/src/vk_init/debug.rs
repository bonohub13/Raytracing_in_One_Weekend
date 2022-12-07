pub struct DebugUtils {
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: ash::vk::DebugUtilsMessengerEXT,
}

impl DebugUtils {
    pub fn cleanup(debug: &mut Self) {
        unsafe {
            debug
                .debug_utils_loader
                .destroy_debug_utils_messenger(debug.debug_messenger, None);
        }
    }

    pub unsafe extern "system" fn debug_callback(
        message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut std::os::raw::c_void,
    ) -> ash::vk::Bool32 {
        use ash::vk;
        use std::ffi::CStr;

        let severity = match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[INFO]",
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[VERBOSE]",
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[WARNING]",
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[ERROR]",
            _ => "[UNKNOWN]",
        };

        let type_ = match message_type {
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[GENERAL]",
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[VALIDATION]",
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[PERFORMANCE]",
            _ => "[UNKNOWN]",
        };

        let message = CStr::from_ptr((*p_callback_data).p_message);

        match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
                log::error!("{}{}{:?}", severity, type_, message);
            }
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
                log::warn!("{}{}{:?}", severity, type_, message);
            }
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | _ => {
                log::info!("{}{}{:?}", severity, type_, message);
            }
        }

        vk::FALSE
    }

    pub fn debug_create_info() -> ash::vk::DebugUtilsMessengerCreateInfoEXT {
        use ash::vk::{self, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT};

        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_type(
                DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | DebugUtilsMessageTypeFlagsEXT::GENERAL,
            )
            .message_severity(
                DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | DebugUtilsMessageSeverityFlagsEXT::INFO
                    | DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
            )
            .pfn_user_callback(Some(Self::debug_callback))
            .build()
    }
}
