mod _physical_device {
    use crate::{queue::QueueFamilyIndices, surface::VkSurfaceInfo};
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
            crate::swapchain::query_swapchain_support(physical_device, surface_info);

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

    fn find_queue_family(
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
}

pub use _physical_device::pick_physical_device;
