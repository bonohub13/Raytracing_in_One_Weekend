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

    present_complete: ash::vk::Semaphore,
    render_complete: ash::vk::Semaphore,

    command_pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,

    compute_command_pool: ash::vk::CommandPool,
    compute_command_buffer: ash::vk::CommandBuffer,
    compute_fence: ash::vk::Fence,
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
        use ash::vk;
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

        let present_complete = vk_init::create_semaphore(&device, "present complete")?;
        let render_complete = vk_init::create_semaphore(&device, "render complete")?;

        let command_pool = vk_init::create_command_pool(
            &device,
            queue_family_indices.graphics_family_index,
            "graphics family",
        )?;
        let command_buffer = vk_init::create_command_buffer(&device, &command_pool)?;

        let compute_command_pool = vk_init::create_command_pool(
            &device,
            queue_family_indices.compute_family_index,
            "compute family",
        )?;
        let compute_command_buffer =
            vk_init::create_command_buffer(&device, &compute_command_pool)?;
        let compute_fence = vk_init::create_fence(&device, Some(vk::FenceCreateFlags::SIGNALED))?;

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
            present_complete,
            render_complete,
            command_pool,
            command_buffer,
            compute_command_pool,
            compute_command_buffer,
            compute_fence,
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

        unsafe {
            self.device.destroy_fence(self.compute_fence, None);
            self.device
                .free_command_buffers(self.compute_command_pool, &[self.compute_command_buffer]);
            self.device
                .destroy_command_pool(self.compute_command_pool, None);
            self.device
                .free_command_buffers(self.command_pool, &[self.command_buffer]);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_semaphore(self.render_complete, None);
            self.device.destroy_semaphore(self.present_complete, None);
        }
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
