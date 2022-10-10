mod _debug {
    use crate::{constants::VK_VALIDATION_LAYER_NAMES, tools as vk_tools};
    use ash::{
        extensions::ext::DebugUtils,
        vk::{
            self, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
            DebugUtilsMessengerCallbackDataEXT,
        },
        Entry,
    };
    use std::os::raw::c_void;

    pub unsafe extern "system" fn vulkan_debug_callback(
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_type: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut c_void,
    ) -> vk::Bool32 {
        use std::ffi::CStr;

        let severity = match message_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
            _ => "[Unknown]",
        };
        let types = match message_type {
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
            _ => "[Unkown]",
        };
        let message = CStr::from_ptr((*p_callback_data).p_message);

        println!("[Debug]{}{}{:?}", severity, types, message,);

        vk::FALSE
    }

    pub fn check_validation_layer_support(entry: &Entry) -> Result<bool, String> {
        let layer_properties = entry.enumerate_instance_layer_properties();

        if layer_properties.is_err() {
            return Err(String::from(
                "failed to enumerate Instance Layer Properties!",
            ));
        }

        let layer_properties = layer_properties.unwrap();

        if layer_properties.len() <= 0 {
            eprintln!("No available layers.");

            return Ok(false);
        } else {
            println!("Instance Available Layers:");

            for layer in layer_properties.iter() {
                let layer_name = vk_tools::vk_to_string(&layer.layer_name);

                if layer_name.is_err() {
                    let err_string = layer_name.err().unwrap();

                    return Err(err_string);
                }

                println!("\t{}", layer_name.unwrap());
            }
        }

        for required_layer_name in VK_VALIDATION_LAYER_NAMES.required_validation_layers.iter() {
            let mut is_layer_found = false;

            for layer_property in layer_properties.iter() {
                let test_layer_name = vk_tools::vk_to_string(&layer_property.layer_name);

                if test_layer_name.is_err() {
                    return Err(test_layer_name.err().unwrap());
                }

                if (*required_layer_name) == test_layer_name.unwrap() {
                    is_layer_found = true;

                    break;
                }
            }

            if !is_layer_found {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn setup_debug_callback(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> Result<(DebugUtils, vk::DebugUtilsMessengerEXT), String> {
        let debug_utils_loader = DebugUtils::new(&entry, &instance);

        if VK_VALIDATION_LAYER_NAMES.is_enable {
            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));
            let debug_callback =
                unsafe { debug_utils_loader.create_debug_utils_messenger(&debug_info, None) };

            if debug_callback.is_err() {
                return Err(String::from("failed to set up debug messenger!"));
            }

            Ok((debug_utils_loader, debug_callback.unwrap()))
        } else {
            Ok((debug_utils_loader, vk::DebugUtilsMessengerEXT::null()))
        }
    }
}

pub use _debug::{check_validation_layer_support, setup_debug_callback, vulkan_debug_callback};
