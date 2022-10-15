mod _common {
    use raw_window_handle::HasRawDisplayHandle;

    use ash::vk;

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    use ash::vk::{
        KhrGetPhysicalDeviceProperties2Fn, KhrPortabilityEnumerationFn, KhrPortabilitySubsetFn,
    };

    pub fn create_instance(
        entry: &ash::Entry,
        window: &winit::window::Window,
    ) -> Result<ash::Instance, String> {
        use crate::{
            constants::{
                APPLICATION_NAME, APPLICATION_VERSION, ENGINE_NAME, ENGINE_VERSION,
                VK_VALIDATION_LAYER_NAMES,
            },
            debug as vk_debug,
        };
        use ash::extensions::ext::DebugUtils;
        use std::ffi::{CStr, CString};
        use std::os::raw::c_char;

        let validation_layer_supported = vk_debug::check_validation_layer_support(entry);

        if validation_layer_supported.is_err() {
            return Err(validation_layer_supported.err().unwrap());
        }

        if VK_VALIDATION_LAYER_NAMES.is_enable && !validation_layer_supported.unwrap() {
            return Err(String::from(
                "Validation layers requested, but not available!",
            ));
        }

        let app_name = unsafe { CStr::from_bytes_with_nul_unchecked(APPLICATION_NAME.as_bytes()) };
        let engine_name = unsafe { CStr::from_bytes_with_nul_unchecked(ENGINE_NAME.as_bytes()) };

        let extension_names =
            ash_window::enumerate_required_extensions(window.raw_display_handle());

        if extension_names.is_err() {
            return Err(String::from(extension_names.err().unwrap().to_string()));
        }

        let mut debug_utils_create_info = vk_debug::populate_debug_messenger_create_info();

        let mut extension_names = extension_names.unwrap().to_vec();

        extension_names.push(DebugUtils::name().as_ptr());
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            extension_names.push(KhrPortabilityEnumerationFn::name().as_ptr());
            extension_names.push(KhrGetPhysicalDeviceProperties2Fn::name().as_ptr());
        }

        let required_validation_layer_names: Vec<CString> = VK_VALIDATION_LAYER_NAMES
            .required_validation_layers
            .iter()
            .map(|&layer_name| CString::new(layer_name).unwrap())
            .collect();
        let raw_layer_names: Vec<*const c_char> = required_validation_layer_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(APPLICATION_VERSION)
            .engine_name(engine_name)
            .engine_version(ENGINE_VERSION)
            .api_version(ash::vk::API_VERSION_1_3);

        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::default()
        };

        let create_info = if VK_VALIDATION_LAYER_NAMES.is_enable {
            vk::InstanceCreateInfo::builder()
                .push_next(&mut debug_utils_create_info)
                .application_info(&app_info)
                .enabled_layer_names(&raw_layer_names)
                .enabled_extension_names(&extension_names)
                .flags(create_flags)
        } else {
            vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(&extension_names)
                .flags(create_flags)
        };

        let instance = unsafe { entry.create_instance(&create_info, None) };

        match instance {
            Ok(instance) => Ok(instance),
            Err(err) => Err(err.result().err().unwrap().to_string()),
        }
    }
}

pub struct Descriptor;

impl Descriptor {
    #[inline]
    pub fn set_layout(device: &ash::Device) -> Result<ash::vk::DescriptorSetLayout, String> {
        use ash::vk;

        let ubo_layout_binding = [
            vk::DescriptorSetLayoutBinding::builder()
                .binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .build(),
            vk::DescriptorSetLayoutBinding::builder()
                .binding(1)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                .build(),
        ];

        let layout_info =
            vk::DescriptorSetLayoutCreateInfo::builder().bindings(&ubo_layout_binding);

        let result = unsafe { device.create_descriptor_set_layout(&layout_info, None) };

        match result {
            Ok(descriptor_set_layout) => Ok(descriptor_set_layout),
            Err(_) => Err(String::from("failed to create descriptor set layout")),
        }
    }
}

pub use _common::create_instance;
