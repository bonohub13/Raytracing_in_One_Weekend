pub struct Engine {
    instance: ash::Instance,
    surface: crate::VkSurface,
    physical_device: ash::vk::PhysicalDevice,
    queue_family_indices: crate::QueueFamilyIndices,
    device: ash::Device,
    compute_queue: ash::vk::Queue,
    graphics_queue: ash::vk::Queue,
    present_queue: ash::vk::Queue,
    swapchain: crate::VkSwapchain,
}

impl Engine {
    // constants
    const SWAP_CHAIN_IMAGE_FORMAT: ash::vk::Format = ash::vk::Format::R8G8B8A8_UNORM;
    const SUMMED_PIXEL_COLOR_IMAGE_FORMAT: ash::vk::Format = ash::vk::Format::R16G16B16A16_UNORM;
    const COLOR_SPACE: ash::vk::ColorSpaceKHR = ash::vk::ColorSpaceKHR::SRGB_NONLINEAR;
    const REQUIRED_INSTANCE_EXTENSIONS: [*const i8; 2] = [
        ash::extensions::ext::DebugUtils::name().as_ptr(),
        ash::extensions::khr::Surface::name().as_ptr(),
    ];
    const REQUIRED_DEVICE_EXTENSIONS: [*const i8; 2] = [
        ash::extensions::khr::Swapchain::name().as_ptr(),
        ash::extensions::khr::Synchronization2::name().as_ptr(),
    ];

    pub fn new(
        settings: &crate::VkSettings,
        app_base: &crate::AppBase,
        window: &crate::window::Window,
    ) -> Result<Engine, String> {
        use crate::vk_init;
        use std::ffi::CStr;

        let validation_layers: Vec<*const i8> =
            vec!["VK_LAYER_KHRONOS_validation\0", "VK_LAYER_LUNARG_monitor\0"]
                .iter()
                .map(|layer| unsafe {
                    CStr::from_bytes_with_nul_unchecked(layer.as_bytes()).as_ptr()
                })
                .collect();

        let instance = vk_init::create_instance(
            app_base,
            window,
            &validation_layers,
            &Self::REQUIRED_INSTANCE_EXTENSIONS,
        )?;

        let surface = vk_init::create_surface(app_base, window, &instance)?;

        let physical_device = vk_init::pick_physical_device(&instance)?;

        let queue_family_indices =
            vk_init::find_queue_families(&instance, &surface, physical_device)?;

        let (device, queues) = vk_init::create_logical_device(
            &instance,
            physical_device,
            &queue_family_indices,
            &Self::REQUIRED_DEVICE_EXTENSIONS,
        )?;

        let present_modes = surface.get_physical_device_surface_present_modes(physical_device)?;
        let present_mode = vk_init::choose_swapchain_present_mode(&present_modes);
        let surface_formats = surface.get_physical_device_surface_formats(physical_device)?;
        let surface_format = vk_init::choose_swapchain_format(&surface_formats)?;
        let surface_capabilities =
            surface.get_physical_device_surface_capabilities(physical_device)?;
        let swapchain = vk_init::create_swapchain(
            &instance,
            physical_device,
            &surface,
            &device,
            crate::constants::WIDTH,
            crate::constants::HEIGHT,
            &surface_capabilities,
            present_mode,
            &surface_format,
        )?;

        Ok(Self {
            instance,
            surface,
            physical_device,
            queue_family_indices,
            device,
            compute_queue: queues.compute,
            graphics_queue: queues.graphics,
            present_queue: queues.present,
            swapchain,
        })
    }

    fn create_summed_pixel_color_image(
        settings: &crate::VkSettings,
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        device: &ash::Device,
    ) -> Result<crate::VkImage, String> {
        use crate::vk_init;

        vk_init::create_summed_pixel_color_image(
            settings,
            instance,
            physical_device,
            device,
            Self::SUMMED_PIXEL_COLOR_IMAGE_FORMAT,
        )
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        log::info!("performing cleanup for Engine");

        crate::VkSwapchain::cleanup(&self.device, &mut self.swapchain);
        unsafe {
            self.device.destroy_device(None);
        }
        crate::VkSurface::cleanup(&mut self.surface);
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
