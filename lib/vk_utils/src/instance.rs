pub struct Instance {
    pub instance: ash::Instance,
    validation_layers: Vec<*const std::os::raw::c_char>,
    extensions: Vec<ash::vk::ExtensionProperties>,
    layers: Vec<ash::vk::LayerProperties>,
    physical_devices: Vec<ash::vk::PhysicalDevice>,
}

impl Instance {
    pub fn new<'a>(
        entry: &ash::Entry,
        window: &'a crate::window::Window,
        validation_layers: &Vec<*const std::os::raw::c_char>,
        vulkan_version: u32,
    ) -> Result<Self, String> {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        use ash::vk::{
            KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn, KhrPortabilitySubsetFn,
        };
        use ash::{extensions::ext::DebugUtils, vk};
        use scopeguard::{guard, ScopeGuard};
        use std::ffi::CStr;

        log::info!("creating Instance");

        Self::check_vulkan_minimum_version(&entry, vulkan_version)?;

        let mut extensions: Vec<*const i8> =
            ash_window::enumerate_required_extensions(window.raw_display_handle())
                .map_err(|_| String::from("failed to enumerate required extensions"))?
                .to_vec();

        Self::check_vulkan_validation_layer_support(&entry, validation_layers)?;

        if !validation_layers.is_empty() {
            extensions.push(DebugUtils::name().as_ptr());
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                extensions.push(KhrPortabilityEnumerationFn::name().as_ptr());
                extensions.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
            }
        }

        let app_name = unsafe {
            CStr::from_bytes_with_nul_unchecked(crate::constants::APPLICATION_NAME.as_bytes())
        };
        let engine_name = unsafe {
            CStr::from_bytes_with_nul_unchecked(crate::constants::ENGINE_NAME.as_bytes())
        };

        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(crate::constants::APPLICATION_VERSION)
            .engine_name(engine_name)
            .engine_version(crate::constants::ENGINE_VERSION)
            .api_version(vulkan_version)
            .build();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&validation_layers);

        log::info!("creating instance");

        let instance_sg = {
            let instance = unsafe {
                entry
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

        let physical_devices = Self::get_vulkan_physical_devices(&instance_sg)?;
        let layers = Self::get_vulkan_layers(entry)?;
        let extensions = Self::get_vulkan_extensions(entry)?;

        Ok(Self {
            instance: ScopeGuard::into_inner(instance_sg),
            physical_devices,
            layers,
            extensions,
            validation_layers: validation_layers.clone(),
        })
    }

    pub fn extensions(&self) -> Vec<ash::vk::ExtensionProperties> {
        self.extensions.clone()
    }

    pub fn layers(&self) -> Vec<ash::vk::LayerProperties> {
        self.layers.clone()
    }

    pub fn physical_devices(&self) -> Vec<ash::vk::PhysicalDevice> {
        self.physical_devices.clone()
    }

    pub fn validation_layers(&self) -> Vec<*const std::os::raw::c_char> {
        self.validation_layers.clone()
    }

    fn get_vulkan_physical_devices(
        instance: &ash::Instance,
    ) -> Result<Vec<ash::vk::PhysicalDevice>, String> {
        log::info!("enumerating physical devices");

        match unsafe { instance.enumerate_physical_devices() } {
            Ok(physical_devices) => {
                if physical_devices.is_empty() {
                    Err(String::from("found no Vulkan physical devices"))
                } else {
                    log::info!("found physical devices");

                    Ok(physical_devices)
                }
            }
            Err(_) => Err(String::from("failed to enumerate Vulkan physical devices")),
        }
    }

    fn get_vulkan_layers(entry: &ash::Entry) -> Result<Vec<ash::vk::LayerProperties>, String> {
        log::info!("enumerating instance layer properties");

        match entry.enumerate_instance_layer_properties() {
            Ok(vulkan_layers) => {
                log::info!("found layer properties");

                Ok(vulkan_layers)
            }
            Err(_) => Err(String::from("failed to enumerate instance layers")),
        }
    }

    fn get_vulkan_extensions(
        entry: &ash::Entry,
    ) -> Result<Vec<ash::vk::ExtensionProperties>, String> {
        log::info!("enumerating instance extension properties");

        match entry.enumerate_instance_extension_properties(None) {
            Ok(extensions) => {
                log::info!("found extensions");

                Ok(extensions)
            }
            Err(_) => Err(String::from("failed to enumerate extensions")),
        }
    }

    fn check_vulkan_minimum_version(entry: &ash::Entry, min_version: u32) -> Result<(), String> {
        log::info!("checking minimum Vulkan version");

        let version = entry
            .try_enumerate_instance_version()
            .map_err(|_| String::from("failed to enumerate instance version"))?;

        match version {
            Some(version) => {
                if min_version > version {
                    Err(format!(
                        "minimum required version not found (required {}, found {})",
                        min_version, version
                    ))
                } else {
                    log::info!("found required minimum version of Vulkan");

                    Ok(())
                }
            }
            None => Err(String::from("failed to enumerate instance version")),
        }
    }

    fn check_vulkan_validation_layer_support(
        entry: &ash::Entry,
        validation_layers: &Vec<*const std::os::raw::c_char>,
    ) -> Result<(), String> {
        use std::ffi::CStr;

        log::info!("checking for available vulkan validation layers");

        let available_layers = entry
            .enumerate_instance_layer_properties()
            .map_err(|_| String::from("failed to enumerate available validation layers"))?;

        if available_layers.is_empty() {
            return Err(String::from("no available layers"));
        } else {
            for &layer in validation_layers.iter() {
                let layer_name = unsafe { CStr::from_ptr(layer).to_str().unwrap().to_string() };

                log::info!("checking existance for validation layer: {}", layer_name);

                match available_layers.iter().find(|&available_layer| {
                    let available_layer_name =
                        crate::utils::vk_to_string(&available_layer.layer_name).unwrap();

                    layer_name == available_layer_name
                }) {
                    Some(_) => {
                        log::info!("found validation layer: {}", layer_name)
                    }
                    None => {
                        return Err(format!(
                            "could not find the requested validation layer: '{}'",
                            layer_name
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
