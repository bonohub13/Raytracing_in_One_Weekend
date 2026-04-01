// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

#[cfg(debug_assertions)]
use crate::DebugUtilsMessenger;
use crate::{RtErr, RtError, util};
use ash::{ext::debug_utils, khr::surface, vk};
use std::{ffi::CStr, sync::Arc};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

pub struct Instance {
    entry: ash::Entry,
    instance: ash::Instance,
    debug_loader: Option<debug_utils::Instance>,
    surface_loader: surface::Instance,
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
    #[cfg(debug_assertions)]
    const VALIDATION_LAYERS: [&CStr; 1] = [c"VK_LAYER_KHRONOS_validation"];

    pub(crate) fn new(desc: &InstanceDesc) -> RtErr<Self> {
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(desc, &entry)?;
        #[cfg(debug_assertions)]
        let debug_loader = Some(debug_utils::Instance::new(&entry, &instance));
        #[cfg(not(debug_assertions))]
        let debug_loader = None;
        let surface_loader = surface::Instance::new(&entry, &instance);

        Ok(Self {
            entry,
            instance,
            debug_loader,
            surface_loader,
        })
    }

    #[inline]
    pub(crate) fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    #[inline]
    pub(crate) fn instance(&self) -> &ash::Instance {
        &self.instance
    }

    #[inline]
    pub fn debug_loader(&self) -> Option<&debug_utils::Instance> {
        self.debug_loader.as_ref()
    }

    #[inline]
    pub fn surface_loader(&self) -> &surface::Instance {
        &self.surface_loader
    }

    fn create_instance(desc: &InstanceDesc, entry: &ash::Entry) -> RtErr<ash::Instance> {
        #[cfg(debug_assertions)]
        if !Self::validation_layer_supported(entry)? {
            return Err(RtError::ValidationLayersNotSupported);
        }

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

        #[cfg(debug_assertions)]
        let vaildation_layers: Vec<_> = Self::VALIDATION_LAYERS
            .iter()
            .map(|layer| layer.as_ptr())
            .collect();
        #[cfg(debug_assertions)]
        let mut debug_binding = DebugUtilsMessenger::create_info();
        #[cfg(debug_assertions)]
        let create_info = create_info
            .enabled_layer_names(&vaildation_layers)
            .push_next(&mut debug_binding);

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

    #[cfg(debug_assertions)]
    fn validation_layer_supported(entry: &ash::Entry) -> RtErr<bool> {
        let layer_properties = unsafe { entry.enumerate_instance_layer_properties() }
            .map_err(|err| RtError::EnumerateInstanceLayerProperties(err.into()))?;
        let available_layers: Vec<_> = layer_properties
            .iter()
            .filter_map(|properties| properties.layer_name_as_c_str().ok())
            .collect();
        let layers_not_found = Self::VALIDATION_LAYERS
            .iter()
            .find(|layer| !available_layers.contains(layer));

        Ok(layers_not_found.is_none())
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
