// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

#[cfg(debug_assertions)]
use crate::Instance;
#[cfg(debug_assertions)]
use crate::{RtErr, RtError};
#[cfg(debug_assertions)]
use ash::vk;
use ash::{ext::debug_utils, vk::DebugUtilsMessengerEXT};
#[cfg(debug_assertions)]
use std::ffi::{c_void, CStr};

pub struct DebugUtilsMessenger {
    handle: DebugUtilsMessengerEXT,
    loader: debug_utils::Instance,
}

#[cfg(debug_assertions)]
impl DebugUtilsMessenger {
    pub(crate) fn new(entry: &ash::Entry, instance: &Instance) -> RtErr<Self> {
        let loader = debug_utils::Instance::new(entry, instance.instance());
        let handle = Self::create_debug_utils_messenger(&loader)?;

        Ok(Self { loader, handle })
    }

    #[inline]
    pub(crate) fn loader(&self) -> &debug_utils::Instance {
        &self.loader
    }

    #[inline]
    pub(crate) fn messenger(&self) -> &vk::DebugUtilsMessengerEXT {
        &self.handle
    }

    pub(crate) fn create_info<'info>() -> vk::DebugUtilsMessengerCreateInfoEXT<'info> {
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

    fn create_debug_utils_messenger(
        loader: &debug_utils::Instance,
    ) -> RtErr<vk::DebugUtilsMessengerEXT> {
        let create_info = Self::create_info();

        unsafe { loader.create_debug_utils_messenger(&create_info, None) }
            .map_err(|err| RtError::CreateDebugUtilsMessenger(err.into()))
    }

    unsafe extern "system" fn debug_callback(
        msg_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        msg_type: vk::DebugUtilsMessageTypeFlagsEXT,
        p_cb_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut c_void,
    ) -> vk::Bool32 {
        const TEXT_COLOR: [&str; 4] = [
            "\x1b[0;31m", // red
            "\x1b[0;32m", // green
            "\x1b[0;33m", // yellow
            "\x1b[0;37m", // white
        ];
        const SEVERITY: [&str; 5] = ["Info", "Verbose", "Warning", "Error", "Unknown"];
        const TYPE: [&str; 5] = [
            "GENERAL",
            "VALIDATION",
            "PERFORMANCE",
            "DEVICE_ADDRESS_BINDING",
            "UNKNOWN",
        ];

        let message = unsafe { CStr::from_ptr((*p_cb_data).p_message) };
        let msg_severity = match msg_severity {
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO => SEVERITY[0].to_string(),
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
                [TEXT_COLOR[1], SEVERITY[1], TEXT_COLOR[3]].concat()
            }
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
                [TEXT_COLOR[2], SEVERITY[2], TEXT_COLOR[3]].concat()
            }
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
                [TEXT_COLOR[0], SEVERITY[3], TEXT_COLOR[3]].concat()
            }
            _ => SEVERITY[4].to_string(),
        };
        let msg_type = match msg_type {
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => TYPE[0],
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => TYPE[1],
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => TYPE[2],
            vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING => TYPE[3],
            _ => TYPE[4],
        };

        eprintln!("[{msg_type} | {msg_severity}] {message:?}");

        vk::FALSE
    }
}

impl Drop for DebugUtilsMessenger {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_debug_utils_messenger(self.handle, None);
        }
    }
}
