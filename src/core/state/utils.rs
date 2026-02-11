use crate::core::{
    error::{RtError, RtResult},
    state::QueueFamilyIndices,
    surface::{self, Surface},
};
use ash::{Instance, vk};
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

impl super::State {
    pub(crate) fn check_validation_support(entry: &ash::Entry) -> RtResult<Vec<CString>> {
        let available_layers = match unsafe { entry.enumerate_instance_layer_properties() } {
            Ok(properties) => Ok(properties),
            Err(e) => Err(RtError::EnumerateInstanceLayer(e.into())),
        }?;
        let supported_layers: Vec<_> = available_layers
            .iter()
            .map(|layer| unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) })
            .collect();
        let unsupported_layers: Vec<_> = Self::VALIDATION_LAYERS
            .iter()
            .filter_map(|layer_name| {
                if supported_layers.contains(layer_name) {
                    None
                } else {
                    Some(unsafe {
                        CString::from_vec_with_nul_unchecked(
                            layer_name.to_bytes_with_nul().to_vec(),
                        )
                    })
                }
            })
            .collect();

        if unsupported_layers.is_empty() {
            Ok(Self::VALIDATION_LAYERS
                .iter()
                .map(|layer| unsafe {
                    CString::from_vec_with_nul_unchecked(layer.to_bytes_with_nul().to_vec())
                })
                .collect())
        } else {
            Err(RtError::ValidationLayerSupport(unsupported_layers))
        }
    }

    pub(crate) fn supported_extensions(entry: &ash::Entry) -> RtResult<Vec<&'static CStr>> {
        match unsafe { entry.enumerate_instance_extension_properties(None) } {
            Ok(extensions) => Ok(extensions
                .iter()
                .map(|ext| unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) })
                .collect()),
            Err(e) => Err(RtError::EnumerateRequiredExtensions(Some(e.into()))),
        }
    }

    pub(crate) fn required_extensions(
        window: Arc<Window>,
        entry: &ash::Entry,
    ) -> RtResult<Vec<CString>> {
        let display = match window.display_handle() {
            Ok(display) => Ok(display.as_raw()),
            Err(e) => Err(RtError::DisplayHandle(e)),
        }?;
        let supported_extensions = Self::supported_extensions(entry)?;

        match ash_window::enumerate_required_extensions(display) {
            Ok(extensions) => {
                let required_extensions = {
                    let mut extensions: Vec<_> = extensions
                        .iter()
                        .map(|ext| unsafe { CStr::from_ptr(*ext) })
                        .collect();

                    extensions.push(ash::khr::portability_enumeration::NAME);
                    #[cfg(debug_assertions)]
                    extensions.push(ash::ext::debug_utils::NAME);

                    extensions
                };
                let all_supported = required_extensions
                    .iter()
                    .filter(|ext| supported_extensions.contains(*ext))
                    .count()
                    == required_extensions.len();

                if all_supported {
                    Ok(required_extensions
                        .iter()
                        .map(|extension| CString::from(*extension))
                        .collect())
                } else {
                    Err(RtError::EnumerateRequiredExtensions(None))
                }
            }
            Err(e) => Err(RtError::EnumerateRequiredExtensions(Some(e.into()))),
        }
    }

    pub(crate) fn sort_by_device_suitability(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> u32 {
        let properties = unsafe { instance.get_physical_device_properties(*physical_device) };
        let mut rate = properties.limits.max_image_dimension2_d;

        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            rate += 0x1000;
        }

        rate
    }

    pub(crate) fn suitable_device(
        instance: &Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
    ) -> Option<QueueFamilyIndices> {
        let features = unsafe { instance.get_physical_device_features(*physical_device) };
        let queue_family =
            QueueFamilyIndices::find_queue_families(instance, surface, physical_device);

        if Self::check_device_extension_support(instance, physical_device).unwrap_or(false) {
            let swapchain_supported = if let Ok(swapchain_support) =
                surface::SwapchainSupportDetails::query_swapchain_support(surface, physical_device)
            {
                swapchain_support.is_adequate()
            } else {
                false
            };

            if swapchain_supported && (features.geometry_shader != 0) && queue_family.is_complete()
            {
                return Some(queue_family);
            }
        }

        None
    }

    pub(crate) fn check_device_extension_support(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> RtResult<bool> {
        let supported_extensions: Vec<_> =
            match unsafe { instance.enumerate_device_extension_properties(*physical_device) } {
                Ok(extensions) => Ok(extensions
                    .iter()
                    .map(|property| unsafe { CStr::from_ptr(property.extension_name.as_ptr()) })
                    .collect()),
                Err(err) => Err(RtError::EnumerateDeviceExtensions(err.into())),
            }?;
        Ok(Self::DEVICE_EXTENSIONS
            .iter()
            .filter(|extension| !supported_extensions.contains(extension))
            .count()
            == 0)
    }
}
