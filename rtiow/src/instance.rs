// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{RtErr, RtError, util};
#[cfg(debug_assertions)]
use ash::ext::debug_utils;
use ash::vk;
use std::{ffi::CStr, sync::Arc};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

pub struct Instance {
    entry: ash::Entry,
    instance: ash::Instance,
}

#[derive(Debug)]
pub struct InstanceDesc<'desc> {
    pub window: Arc<Window>,
    pub app_name: &'desc CStr,
    pub app_version: u32,
}

impl Instance {
    const ENGINE_NAME: &CStr = c"Rtiow";
    const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

    pub fn new(desc: &InstanceDesc) -> RtErr<Self> {
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(desc, &entry)?;

        Ok(Self { entry, instance })
    }

    #[inline]
    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    #[inline]
    pub fn instance(&self) -> &ash::Instance {
        &self.instance
    }

    fn create_instance(desc: &InstanceDesc, entry: &ash::Entry) -> RtErr<ash::Instance> {
        #[cfg(debug_assertions)]
        {
            eprintln!("available extensions:");
            Self::enumerate_instance_extension_properties(entry)?
                .iter()
                .for_each(|property| {
                    let extension_name =
                        unsafe { CStr::from_ptr(property.extension_name.as_ptr()) };

                    eprintln!("\t{extension_name:?}");
                })
        }

        let app_info = vk::ApplicationInfo::default()
            .application_name(desc.app_name)
            .application_version(desc.app_version)
            .engine_name(Self::ENGINE_NAME)
            .engine_version(util::parse_version_from_str(Self::ENGINE_VERSION)?)
            .api_version(vk::API_VERSION_1_3);
        let extensions: Vec<_> = Self::get_required_extension(desc)?
            .iter()
            .map(|ext| ext.as_ptr())
            .collect();
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions);

        unsafe { entry.create_instance(&create_info, None) }
            .map_err(|err| RtError::CreateInstance(err.into()))
    }

    #[cfg(debug_assertions)]
    fn enumerate_instance_extension_properties(
        entry: &ash::Entry,
    ) -> RtErr<Vec<vk::ExtensionProperties>> {
        unsafe { entry.enumerate_instance_extension_properties(None) }
            .map_err(|err| RtError::EnumerateInstanceExtensionProperties(err.into()))
    }

    fn get_required_extension<'ext>(desc: &InstanceDesc) -> RtErr<Vec<&'ext CStr>> {
        let display_handle = desc
            .window
            .display_handle()
            .map_err(|err| RtError::DisplayHandle(err.into()))?
            .as_raw();
        let extensions = ash_window::enumerate_required_extensions(display_handle)
            .map_err(|err| RtError::EnumerateRequiredExtensions(err.into()))?;
        #[allow(unused_mut)]
        let mut extensions: Vec<_> = extensions
            .iter()
            .copied()
            .map(|ext| unsafe { CStr::from_ptr(ext) })
            .collect();

        #[cfg(debug_assertions)]
        extensions.push(debug_utils::NAME);

        Ok(extensions)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
