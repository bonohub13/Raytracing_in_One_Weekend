// Copyright 2026 Kensuke
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::core::{RtError, RtResult, instance::Instance};
use ash::vk::{self, DebugUtilsMessengerEXT};
use std::{
    ffi::{CStr, c_void},
    sync::Arc,
};

pub struct DebugUtilsMessenger {
    instance: Arc<Instance>,
    messenger: DebugUtilsMessengerEXT,
}

impl DebugUtilsMessenger {
    #[allow(dead_code)]
    pub fn new(instance: Arc<Instance>) -> RtResult<Self> {
        let messenger = Self::create_debug_utils_messenger(instance.clone())?;

        Ok(Self {
            instance,
            messenger,
        })
    }

    #[allow(dead_code)]
    pub fn create_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
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
            .pfn_user_callback(Some(Self::debug_callback))
    }

    #[allow(dead_code)]
    fn create_debug_utils_messenger(
        instance: Arc<Instance>,
    ) -> RtResult<vk::DebugUtilsMessengerEXT> {
        if let Some(debug_loader) = instance.debug_loader().as_ref() {
            let create_info = Self::create_info();

            match unsafe { debug_loader.create_debug_utils_messenger(&create_info, None) } {
                Ok(messenger) => Ok(messenger),
                Err(err) => Err(RtError::CreateDebugUtilsmessenger(err.into())),
            }
        } else {
            Err(RtError::DebugLoaderUninitialized)
        }
    }

    #[allow(dead_code)]
    unsafe extern "system" fn debug_callback(
        msg_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
        p_cb_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut c_void,
    ) -> vk::Bool32 {
        const RED: &str = "\x1b[0;31m";
        const GREEN: &str = "\x1b[0;32m";
        const YELLOW: &str = "\x1b[0;33m";
        const WHITE: &str = "\x1b[0;37m";

        let message = unsafe { CStr::from_ptr((*p_cb_data).p_message) };
        let msg_severity = match msg_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "Info".to_string(),
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => [GREEN, "Verbose", WHITE].concat(),
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => [YELLOW, "Warning", WHITE].concat(),
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => [RED, "Error", WHITE].concat(),
            _ => "Unknown".to_string(),
        };
        let msg_type = match msg_type {
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
            vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING => "DEVICE_ADDRESS_BINDING",
            _ => "UNKNOWN",
        };

        eprintln!("[{} | {}] {:?}", msg_type, msg_severity, message);

        vk::FALSE
    }
}

impl Drop for DebugUtilsMessenger {
    fn drop(&mut self) {
        if let Some(debug_loader) = self.instance.debug_loader().as_ref() {
            unsafe { debug_loader.destroy_debug_utils_messenger(self.messenger, None) };
        }
    }
}
