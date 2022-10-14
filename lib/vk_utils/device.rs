mod _physical_device {
    use crate::{queue::QueueFamilyIndices, surface::VkSurfaceInfo, swapchain::VkSwapchainDetail};
    use ash::vk;

    pub fn pick_physical_device(
        instance: &ash::Instance,
        surface_info: &VkSurfaceInfo,
    ) -> Result<vk::PhysicalDevice, String> {
        let physical_devices = unsafe { instance.enumerate_physical_devices() };

        if physical_devices.is_err() {
            return Err(String::from("failed to find GPU(s) with Vulkan support!"));
        }

        let mut result = None;

        for &physical_device in physical_devices.unwrap().iter() {
            let is_device_suitable = check_device_suitable(instance, physical_device, surface_info);

            if is_device_suitable.is_err() {
                return Err(is_device_suitable.err().unwrap());
            }

            if is_device_suitable.unwrap() && result.is_none() {
                result = Some(physical_device);
                break;
            }
        }

        match result {
            None => Err(String::from("failed to find a suitable GPU!")),
            Some(physical_device) => Ok(physical_device),
        }
    }

    pub fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_info: &VkSurfaceInfo,
    ) -> Result<QueueFamilyIndices, String> {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut queue_family_indices = QueueFamilyIndices::new(None, None);
        let mut index: u32 = 0;

        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_indices.graphics_family = Some(index);
            }

            let is_present_support = unsafe {
                surface_info
                    .surface_loader
                    .get_physical_device_surface_support(
                        physical_device,
                        index,
                        surface_info.surface,
                    )
            };

            if is_present_support.is_err() {
                return Err(is_present_support.err().unwrap().to_string());
            }

            if queue_family.queue_count > 0 && is_present_support.unwrap() {
                queue_family_indices.present_family = Some(index);
            }

            if queue_family_indices.is_complete() {
                break;
            }

            index += 1;
        }

        Ok(queue_family_indices)
    }

    pub fn get_max_usable_sample_count(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::SampleCountFlags {
        let physical_device_properties =
            unsafe { instance.get_physical_device_properties(physical_device) };
        let counts = std::cmp::min(
            physical_device_properties
                .limits
                .framebuffer_color_sample_counts,
            physical_device_properties
                .limits
                .framebuffer_depth_sample_counts,
        );

        if counts.contains(vk::SampleCountFlags::TYPE_64) {
            vk::SampleCountFlags::TYPE_64
        } else if counts.contains(vk::SampleCountFlags::TYPE_32) {
            vk::SampleCountFlags::TYPE_32
        } else if counts.contains(vk::SampleCountFlags::TYPE_16) {
            vk::SampleCountFlags::TYPE_16
        } else if counts.contains(vk::SampleCountFlags::TYPE_8) {
            vk::SampleCountFlags::TYPE_8
        } else if counts.contains(vk::SampleCountFlags::TYPE_4) {
            vk::SampleCountFlags::TYPE_4
        } else if counts.contains(vk::SampleCountFlags::TYPE_2) {
            vk::SampleCountFlags::TYPE_2
        } else {
            vk::SampleCountFlags::TYPE_1
        }
    }

    pub fn get_memory_property(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        unsafe { instance.get_physical_device_memory_properties(physical_device) }
    }

    pub fn get_property(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        unsafe { instance.get_physical_device_properties(physical_device) }
    }

    fn check_device_suitable(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_info: &VkSurfaceInfo,
    ) -> Result<bool, String> {
        let indices = find_queue_family(instance, physical_device, surface_info);

        if indices.is_err() {
            return Err(indices.err().unwrap());
        }

        let extensions_supported = check_device_extension_support(instance, physical_device);

        if extensions_supported.is_err() {
            return Err(extensions_supported.err().unwrap());
        }

        let swapchain_support =
            VkSwapchainDetail::query_swapchain_support(physical_device, surface_info);

        if swapchain_support.is_err() {
            return Err(swapchain_support.err().unwrap());
        }

        let extensions_supported = extensions_supported.unwrap();

        let swapchain_adequate = if extensions_supported {
            let swapchain_support = swapchain_support.unwrap();
            !(swapchain_support.formats.is_empty() || swapchain_support.present_modes.is_empty())
        } else {
            false
        };

        let supported_features = unsafe { instance.get_physical_device_features(physical_device) };

        Ok(indices.unwrap().is_complete()
            && extensions_supported
            && swapchain_adequate
            && supported_features.sampler_anisotropy == 1)
    }

    fn check_device_extension_support(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> Result<bool, String> {
        use crate::{constants::VK_DEVICE_EXTENSIONS, tools as vk_tools};
        use std::collections::HashSet;

        let available_extensions =
            unsafe { instance.enumerate_device_extension_properties(physical_device) };

        if available_extensions.is_err() {
            return Err(String::from("failed to get device extension properties."));
        }

        let mut available_extension_names: Vec<String> = Vec::new();

        for extension in available_extensions.unwrap().iter() {
            let extension_name = vk_tools::vk_to_string(&extension.extension_name);

            if extension_name.is_err() {
                return Err(extension_name.err().unwrap());
            }

            available_extension_names.push(extension_name.unwrap());
        }

        let mut required_extensions: HashSet<String> = VK_DEVICE_EXTENSIONS
            .names
            .iter()
            .map(|extension| extension.to_string())
            .collect();

        for extension_name in available_extension_names.iter() {
            required_extensions.remove(extension_name);
        }

        Ok(required_extensions.is_empty())
    }
}

mod _device {
    use super::_physical_device::find_queue_family;
    use crate::{queue::QueueFamilyIndices, surface::VkSurfaceInfo};
    use ash::vk;

    pub fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_info: &VkSurfaceInfo,
    ) -> Result<(ash::Device, QueueFamilyIndices), String> {
        use crate::constants::VK_VALIDATION_LAYER_NAMES;
        use ash::extensions::khr::Swapchain;
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        use ash::vk::KhrPortabilitySubsetFn;
        use std::{collections::HashSet, ffi::CString, os::raw::c_char};

        let indices = find_queue_family(instance, physical_device, surface_info);

        if indices.is_err() {
            return Err(indices.err().unwrap());
        }

        let indices = indices.unwrap();
        let mut unique_queue_families = HashSet::new();

        unique_queue_families.insert(indices.graphics_family.unwrap());
        unique_queue_families.insert(indices.present_family.unwrap());

        let queue_priorities = [1.0_f32];
        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = unique_queue_families
            .iter()
            .map(|&queue_family| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(queue_family)
                    .queue_priorities(&queue_priorities)
                    .build()
            })
            .collect();

        let device_features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true)
            .sample_rate_shading(true)
            .shader_clip_distance(true)
            .build();
        let mut required_validation_layers_raw = Vec::<CString>::new();
        for &layer_name in VK_VALIDATION_LAYER_NAMES.required_validation_layers.iter() {
            let layer_name_raw = CString::new(layer_name);

            if layer_name_raw.is_err() {
                return Err(String::from(
                    "failed to convert Vulkan Validation Layer name into raw cstring.",
                ));
            }

            required_validation_layers_raw.push(layer_name_raw.unwrap());
        }

        let enabled_layer_names: Vec<*const c_char> = required_validation_layers_raw
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let device_extensions = [
            Swapchain::name().as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            KhrPortabilitySubsetFn::name().as_ptr(),
        ];
        let create_info = if VK_VALIDATION_LAYER_NAMES.is_enable {
            vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_create_infos)
                .enabled_features(&device_features)
                .enabled_layer_names(&enabled_layer_names)
                .enabled_extension_names(&device_extensions)
        } else {
            vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_create_infos)
                .enabled_features(&device_features)
                .enabled_extension_names(&device_extensions)
        };

        let device = unsafe { instance.create_device(physical_device, &create_info, None) };

        match device {
            Ok(device) => Ok((device, indices)),
            Err(_) => Err(String::from("failed to create logical device!")),
        }
    }
}

pub use _physical_device::{
    get_max_usable_sample_count, get_memory_property, get_property, pick_physical_device,
};

pub use _device::create_logical_device;
