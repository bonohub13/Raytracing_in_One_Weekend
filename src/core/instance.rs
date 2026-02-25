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

#[cfg(debug_assertions)]
use crate::core::debug::DebugUtilsMessenger;
use crate::core::{RtError, RtResult};
use ash::{ext::debug_utils, khr::surface, vk};
use std::{ffi::CStr, sync::Arc};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

pub struct InstanceDescriptor<'desc> {
    pub application_name: &'desc CStr,
    pub application_version: u32,
    pub window: Arc<Window>,
}

pub struct Instance {
    entry: ash::Entry,
    raw: ash::Instance,
    debug_loader: Option<debug_utils::Instance>,
    surface_loader: surface::Instance,
}

impl Instance {
    #[allow(dead_code)]
    const VALIDATION_LAYERS: [&CStr; 1] = [c"VK_LAYER_KHRONOS_validation"];

    pub fn new(desc: &InstanceDescriptor) -> RtResult<Self> {
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(&entry, desc)?;
        let surface_loader = surface::Instance::new(&entry, &instance);
        #[cfg(debug_assertions)]
        let debug_loader = Some(debug_utils::Instance::new(&entry, &instance));
        #[cfg(not(debug_assertions))]
        let debug_loader = None;

        Ok(Self {
            entry,
            raw: instance,
            surface_loader,
            debug_loader,
        })
    }

    #[inline]
    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    #[inline]
    pub fn raw(&self) -> &ash::Instance {
        &self.raw
    }

    #[inline]
    pub fn debug_loader(&self) -> Option<debug_utils::Instance> {
        self.debug_loader.clone()
    }

    #[inline]
    pub fn surface_loader(&self) -> &surface::Instance {
        &self.surface_loader
    }

    pub fn enumerate_physical_devices(&self) -> RtResult<Vec<vk::PhysicalDevice>> {
        match unsafe { self.raw.enumerate_physical_devices() } {
            Ok(devices) => Ok(devices),
            Err(err) => Err(RtError::EnumeratePhyicalDevices(err.into())),
        }
    }

    fn create_instance(entry: &ash::Entry, desc: &InstanceDescriptor) -> RtResult<ash::Instance> {
        #[cfg(debug_assertions)]
        if !Self::validation_layer_supported(entry)? {
            return Err(RtError::RequiredValidationLayersNotSupported);
        }

        #[cfg(debug_assertions)]
        eprintln!("available extensions:");
        #[cfg(debug_assertions)]
        Self::enumerate_instance_extension_properties(entry)?
            .iter()
            .for_each(|extension| {
                let extension_name = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) };

                eprintln!("\t{:?}", extension_name);
            });

        let app_info = vk::ApplicationInfo::default()
            .application_name(desc.application_name)
            .application_version(desc.application_version)
            .api_version(vk::API_VERSION_1_3);
        let extensions: Vec<*const i8> = Self::get_required_extensions(desc.window.clone())?
            .iter()
            .map(|extension| extension.as_ptr())
            .collect();
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions);
        #[cfg(debug_assertions)]
        let validation_layers: Vec<*const i8> = Self::VALIDATION_LAYERS
            .iter()
            .map(|layer| layer.as_ptr())
            .collect();
        #[cfg(debug_assertions)]
        let mut debug_create_info = DebugUtilsMessenger::create_info();
        #[cfg(debug_assertions)]
        let create_info = create_info
            .enabled_layer_names(&validation_layers)
            .push_next(&mut debug_create_info);

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => Ok(instance),
            Err(err) => Err(RtError::CreateInstance(err.into())),
        }
    }

    #[cfg(debug_assertions)]
    fn enumerate_instance_extension_properties(
        entry: &ash::Entry,
    ) -> RtResult<Vec<vk::ExtensionProperties>> {
        match unsafe { entry.enumerate_instance_extension_properties(None) } {
            Ok(properties) => Ok(properties),
            Err(err) => Err(RtError::EnumerateInstanceExtensionProperties(err.into())),
        }
    }

    fn get_required_extensions<'ext>(window: Arc<Window>) -> RtResult<Vec<&'ext CStr>> {
        let display_handle = match window.display_handle() {
            Ok(display_handle) => Ok(display_handle),
            Err(err) => Err(RtError::DisplayHandle(err.into())),
        }?
        .as_raw();

        match ash_window::enumerate_required_extensions(display_handle) {
            Ok(extensions) => {
                #[allow(unused_mut)]
                let mut extensions: Vec<&'ext CStr> = extensions
                    .iter()
                    .map(|extension| unsafe { CStr::from_ptr(*extension) })
                    .collect();

                #[cfg(debug_assertions)]
                extensions.push(debug_utils::NAME);

                Ok(extensions)
            }
            Err(err) => Err(RtError::EnumerateRequiredExtensions(err.into())),
        }
    }

    #[cfg(debug_assertions)]
    fn validation_layer_supported(entry: &ash::Entry) -> RtResult<bool> {
        let available_layers = match unsafe { entry.enumerate_instance_layer_properties() } {
            Ok(properties) => {
                let layer_names: Vec<&CStr> = properties
                    .iter()
                    .map(|layer| unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) })
                    .collect();

                Ok(layer_names)
            }
            Err(err) => Err(RtError::EnumerateInstanceLayerProperties(err.into())),
        }?;
        let validation_layers_supported = !Self::VALIDATION_LAYERS
            .iter()
            .any(|layer| !available_layers.contains(layer));

        Ok(validation_layers_supported)
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.raw.destroy_instance(None) };
    }
}
