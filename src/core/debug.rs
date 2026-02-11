#[cfg(debug_assertions)]
use crate::core::error::{RtError, RtResult};
#[cfg(debug_assertions)]
use ash::{Entry, Instance};
use ash::{ext::debug_utils, vk};
#[cfg(debug_assertions)]
use std::ffi::{CStr, c_void};

pub struct DebugUtils {
    instance: debug_utils::Instance,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugUtils {
    #[cfg(debug_assertions)]
    pub fn new(entry: &Entry, instance: &Instance) -> RtResult<Self> {
        let instance = debug_utils::Instance::new(entry, instance);
        let messenger = Self::create_messenger(&instance)?;

        Ok(Self {
            instance,
            messenger,
        })
    }

    #[cfg(debug_assertions)]
    pub fn messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
        vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(debug_callback))
    }

    pub unsafe fn destroy(&mut self) {
        unsafe {
            self.instance
                .destroy_debug_utils_messenger(self.messenger, None)
        };
    }

    #[cfg(debug_assertions)]
    fn create_messenger(instance: &debug_utils::Instance) -> RtResult<vk::DebugUtilsMessengerEXT> {
        let create_info = Self::messenger_create_info();

        match unsafe { instance.create_debug_utils_messenger(&create_info, None) } {
            Ok(messenger) => Ok(messenger),
            Err(e) => Err(RtError::CreateDebugUtilsMessenger(e.into())),
        }
    }
}

#[cfg(debug_assertions)]
unsafe extern "system" fn debug_callback(
    serverity: vk::DebugUtilsMessageSeverityFlagsEXT,
    types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    let message = unsafe { CStr::from_ptr((*p_callback_data).p_message) };

    eprintln!(
        "[VULKAN | {:?}] {:?}: {}",
        serverity,
        types,
        message.to_string_lossy()
    );

    vk::FALSE
}
