struct PhysicalDeviceTier {
    physical_device: ash::vk::PhysicalDevice,
    device_type: u8,
}

pub fn create_instance(
    app_base: &crate::AppBase,
    window: &crate::window::Window,
    validation_layers: &Vec<*const i8>,
    required_extensions: &[*const i8],
) -> Result<ash::Instance, String> {
    use crate::{constants, vk_init};
    use ash::vk;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    use ash::vk::{
        KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn, KhrPortabilitySubsetFn,
    };
    use scopeguard::{guard, ScopeGuard};
    use std::ffi::CStr;

    let vk_version = match app_base
        .entry()
        .try_enumerate_instance_version()
        .map_err(|_| String::from("failed to enumerate instance version"))?
    {
        Some(version) => {
            log::info!(
                "Vulkan API supported verison {}.{}.{}",
                vk::api_version_major(version),
                vk::api_version_minor(version),
                vk::api_version_patch(version)
            );

            version
        }
        None => {
            log::warn!("failed to find supported Vulkan API version, defaulting to 1.2.0");

            vk::API_VERSION_1_2
        }
    };

    let mut extensions = ash_window::enumerate_required_extensions(window.raw_display_handle())
        .map_err(|_| String::from("failed to enumerate required extensions"))?
        .to_vec();

    let validated_validation_layers =
        check_vulkan_validation_layer_support(app_base.entry(), validation_layers)?;

    if !validated_validation_layers.is_empty() {
        for &extension in required_extensions.into_iter() {
            if !extensions.contains(&extension) {
                extensions.push(extension);
            }
        }
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            extensions.push(KhrPortabilityEnumerationFn::name().as_ptr());
            extensions.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        }
    }

    let application_name =
        unsafe { CStr::from_bytes_with_nul_unchecked(constants::APPLICATION_NAME.as_bytes()) };
    let engine_name =
        unsafe { CStr::from_bytes_with_nul_unchecked(constants::ENGINE_NAME.as_bytes()) };

    let app_info = vk::ApplicationInfo::builder()
        .application_name(application_name)
        .application_version(constants::APPLICATION_VERSION)
        .engine_name(engine_name)
        .engine_version(constants::ENGINE_VERSION)
        .api_version(vk_version)
        .build();

    let create_info = vk::InstanceCreateInfo::builder()
        .push_next(&mut vk_init::DebugUtils::debug_create_info())
        .application_info(&app_info)
        .enabled_extension_names(&extensions)
        .enabled_layer_names(&validated_validation_layers)
        .build();

    let instance_sg = {
        let instance = unsafe {
            app_base
                .entry()
                .create_instance(&create_info, None)
                .map_err(|_| String::from("failed to create instance"))?
        };

        guard(instance, |instance| {
            log::warn!("instance scopeguard");

            unsafe {
                instance.destroy_instance(None);
            }
        })
    };

    log::info!("created instance");

    Ok(ScopeGuard::into_inner(instance_sg))
}

pub fn pick_physical_device(instance: &ash::Instance) -> Result<ash::vk::PhysicalDevice, String> {
    use ash::vk;

    log::info!("finding suitable physical device");

    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .map_err(|_| String::from("failed to enumerate physical devices"))?
    };

    if physical_devices.is_empty() {
        Err(String::from("failed to find GPU(s) with Vulkan support"))
    } else {
        let suitable_device = match physical_devices.iter().find(|&device| {
            let physical_device_properties =
                unsafe { instance.get_physical_device_properties(*device) };

            physical_device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        }) {
            Some(physical_device) => *physical_device,
            None => return Err(String::from("failed to find suitable physical device")),
        };

        Ok(suitable_device)
    }
}

pub fn find_queue_families(
    instance: &ash::Instance,
    surface: &crate::VkSurface,
    physical_device: ash::vk::PhysicalDevice,
) -> Result<crate::QueueFamilyIndices, String> {
    use ash::vk;

    log::info!("finding queue families");

    let queue_families =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let mut compute_family_index: Option<u32> = None;
    let mut graphics_family_index: Option<u32> = None;
    let mut present_family_index: Option<u32> = None;

    for i in 0..queue_families.len() {
        let supports_compute = queue_families[i]
            .queue_flags
            .contains(vk::QueueFlags::COMPUTE);
        let supports_graphics = queue_families[i]
            .queue_flags
            .contains(vk::QueueFlags::GRAPHICS);
        let supports_presenting =
            surface.get_physical_device_surface_support(physical_device, i as u32)?;

        if supports_graphics && graphics_family_index.is_none() {
            graphics_family_index = Some(i as u32);
        }
        if supports_presenting && present_family_index.is_none() {
            present_family_index = Some(i as u32);
        }
        if supports_compute && compute_family_index.is_none() {
            compute_family_index = Some(i as u32);
        }

        if compute_family_index.is_some()
            && graphics_family_index.is_some()
            && present_family_index.is_some()
        {
            break;
        }
    }

    match (
        compute_family_index,
        graphics_family_index,
        present_family_index,
    ) {
        (Some(compute), Some(graphics), Some(present)) => {
            log::info!("found all required queue family index:");
            log::info!("\tcompute queue family index: {}", compute);
            log::info!("\tgraphics queue family index: {}", graphics);
            log::info!("\tpresent queue family index: {}", present);

            Ok(crate::QueueFamilyIndices {
                compute_family_index: compute,
                graphics_family_index: graphics,
                present_family_index: present,
            })
        }
        _ => Err(format!(
            "failed to find suitable queue family index (compute: {:?}, graphics: {:?}, present: {:?})",
            compute_family_index,
            graphics_family_index,
            present_family_index,
        )),
    }
}

fn check_vulkan_validation_layer_support(
    entry: &ash::Entry,
    validation_layers: &Vec<*const std::os::raw::c_char>,
) -> Result<Vec<*const i8>, String> {
    use std::ffi::CStr;

    log::info!("checking for available Vulkan validation layers");

    let available_layers = entry
        .enumerate_instance_layer_properties()
        .map_err(|_| String::from("failed to enumerate available validation layers"))?;

    if available_layers.is_empty() {
        return Err(String::from("no availble validation layers"));
    } else {
        use crate::utils;

        let mut validated_layers = Vec::new();

        for &layer in validation_layers.iter() {
            let layer_name = unsafe {
                CStr::from_ptr(layer)
                    .to_str()
                    .map_err(|err| format!("{}", err))?
                    .to_string()
            };

            log::info!("checking support for validation layer: {}", layer_name);

            match available_layers.iter().find(|&available_layer| {
                let available_layer_name =
                    utils::vk_to_string(&available_layer.layer_name).unwrap_or(String::from(""));

                layer_name == available_layer_name
            }) {
                Some(_) => {
                    log::info!("found validation layer: {}", layer_name);

                    validated_layers.push(layer);
                }
                None => {
                    log::warn!("validation layer not supported: {}", layer_name);
                }
            }
        }

        if validated_layers.is_empty() {
            Err(String::from("no validation layer were supported"))
        } else {
            Ok(validated_layers)
        }
    }
}
