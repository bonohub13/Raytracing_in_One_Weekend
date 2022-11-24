pub struct Instance {
    pub instance: ash::Instance,
    validation_layers: Vec<*const std::os::raw::c_char>,
    extensions: Vec<ash::vk::ExtensionProperties>,
    layers: Vec<ash::vk::LayerProperties>,
    physical_devices: Vec<ash::vk::PhysicalDevice>,
}

impl Instance {
    pub fn new(
        appbase: &crate::AppBase,
        window: &crate::window::Window,
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

        Self::check_vulkan_minimum_version(&appbase.entry, vulkan_version)?;

        let mut extensions: Vec<*const i8> =
            ash_window::enumerate_required_extensions(window.raw_display_handle())
                .map_err(|_| String::from("failed to enumerate required extensions"))?
                .to_vec();

        Self::check_vulkan_validation_layer_support(&appbase.entry, validation_layers)?;

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
                appbase
                    .entry
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
        let layers = Self::get_vulkan_layers(&appbase.entry)?;
        let extensions = Self::get_vulkan_extensions(&appbase.entry)?;

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

    pub fn supported_physical_device(
        &self,
        enabled_physical_device_features: Option<&PhysicalDeviceRequiredFeatures>,
    ) -> Result<ash::vk::PhysicalDevice, String> {
        use crate::utils::vk_to_string;
        use ash::{extensions::khr::RayTracingPipeline, vk};

        let physical_devices = self.physical_devices();

        let result = physical_devices.iter().find(|&physical_device| {
            log::info!("checking geometry shader support for device");

            match enabled_physical_device_features {
                Some(features) => {
                    let geometry_shader_supported = if features.geometry_shader_support {
                        let device_features = self.get_physical_device_features(*physical_device);
                        if device_features.geometry_shader > 0 {
                            true
                        } else {
                            log::warn!("geometry shader not supported");

                            false
                        }
                    } else {
                        true
                    };

                    let ray_tracing_supported = if features.ray_tracing_support {
                        let extensions = self
                            .enumerate_device_extension_properties(*physical_device)
                            .unwrap_or(vec![]);
                        match extensions.iter().find(|&extension| {
                            extension.extension_name.as_ptr() == RayTracingPipeline::name().as_ptr()
                        }) {
                            Some(_) => true,
                            None => {
                                log::warn!("ray tracing not supported");

                                false
                            }
                        }
                    } else {
                        true
                    };

                    let graphics_queue_supported = if features.graphics_queue_support {
                        let queue_families =
                            self.get_physical_device_queue_family_properties(*physical_device);
                        match queue_families.iter().find(|&queue_family| {
                            queue_family.queue_count > 0
                                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        }) {
                            Some(_) => true,
                            None => {
                                log::warn!("graphics queue not supported");

                                false
                            }
                        }
                    } else {
                        true
                    };

                    geometry_shader_supported && ray_tracing_supported && graphics_queue_supported
                }
                None => {
                    let device_features = self.get_physical_device_features(*physical_device);
                    let geometry_shader_supported = if device_features.geometry_shader > 0 {
                        true
                    } else {
                        log::warn!("geometry shader not supported");

                        false
                    };

                    log::info!("checking ray tracing support for device");

                    let extensions = self
                        .enumerate_device_extension_properties(*physical_device)
                        .unwrap_or(vec![]);
                    let ray_tracing_supported = match extensions.iter().find(|&extension| {
                        extension.extension_name.as_ptr() == RayTracingPipeline::name().as_ptr()
                    }) {
                        Some(_) => true,
                        None => {
                            log::warn!("ray tracing not supported");

                            false
                        }
                    };

                    log::info!("checking for graphics queue in device");

                    let queue_families =
                        self.get_physical_device_queue_family_properties(*physical_device);
                    let graphics_queue_supported =
                        match queue_families.iter().find(|&queue_family| {
                            queue_family.queue_count > 0
                                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        }) {
                            Some(_) => true,
                            None => {
                                log::warn!("graphics queue not supported");

                                false
                            }
                        };

                    geometry_shader_supported && ray_tracing_supported && graphics_queue_supported
                }
            }
        });

        match result {
            Some(physical_device) => {
                let mut device_properties = vk::PhysicalDeviceProperties2::default();
                unsafe {
                    self.instance
                        .get_physical_device_properties2(*physical_device, &mut device_properties);
                }

                let device_name = vk_to_string(&device_properties.properties.device_name)?;
                log::info!("Setting device [{}]", device_name);

                Ok(*physical_device)
            }
            None => Err(String::from("failed to find a suitable device")),
        }
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

    pub fn create_device(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        create_info: &ash::vk::DeviceCreateInfo,
        allocation_callback: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::Device, String> {
        unsafe {
            self.instance
                .create_device(physical_device, create_info, allocation_callback)
                .map_err(|_| String::from("failed to create device"))
        }
    }

    pub fn get_physical_device_features(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> ash::vk::PhysicalDeviceFeatures {
        unsafe { self.instance.get_physical_device_features(physical_device) }
    }

    pub fn get_physical_device_queue_family_properties(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Vec<ash::vk::QueueFamilyProperties> {
        unsafe {
            self.instance
                .get_physical_device_queue_family_properties(physical_device)
        }
    }

    pub fn get_physical_device_memory_properties(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> ash::vk::PhysicalDeviceMemoryProperties {
        unsafe {
            self.instance
                .get_physical_device_memory_properties(physical_device)
        }
    }

    pub fn get_physical_device_format_properties(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        format: ash::vk::Format,
    ) -> ash::vk::FormatProperties {
        unsafe {
            self.instance
                .get_physical_device_format_properties(physical_device, format)
        }
    }

    pub fn enumerate_device_extension_properties(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Vec<ash::vk::ExtensionProperties>, String> {
        unsafe {
            self.instance
                .enumerate_device_extension_properties(physical_device)
                .map_err(|_| String::from("failed to enumerate device extension properties"))
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

    pub fn cleanup(instance: &mut Self) {
        log::info!("performing cleanup for Instance");

        unsafe {
            instance.instance.destroy_instance(None);
        }
    }
}

pub struct PhysicalDeviceRequiredFeatures {
    pub geometry_shader_support: bool,
    pub ray_tracing_support: bool,
    pub graphics_queue_support: bool,
}

impl Default for PhysicalDeviceRequiredFeatures {
    fn default() -> Self {
        Self {
            geometry_shader_support: true,
            ray_tracing_support: true,
            graphics_queue_support: true,
        }
    }
}
