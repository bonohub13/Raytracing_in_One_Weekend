struct Swapchain {
    swapchain: crate::SwapChain,
    depth_buffer: crate::DepthBuffer,
    image_available_semaphores: Vec<crate::Semaphore>,
    render_finished_semaphores: Vec<crate::Semaphore>,
    in_flight_fences: Vec<crate::Fence>,
    uniform_buffers: Vec<crate::assets::UniformBuffer>,
}

pub struct App {
    instance: crate::Instance,
    debug_utils_messenger: crate::DebugUtilsMessenger,
    surface: crate::Surface,
    device: crate::Device,
}

impl App {
    pub fn new(
        appbase: &crate::AppBase,
        window: &crate::window::Window,
        present_mode: ash::vk::PresentModeKHR,
        enabled_physical_device_features: Option<&crate::PhysicalDeviceRequiredFeatures>,
        enable_validation_layers: bool,
    ) -> Result<Self, String> {
        use ash::vk;
        use std::ffi::CString;

        let validation_layers: Vec<CString> = if enable_validation_layers {
            vec!["VK_LAYER_KHRONOS_validation"]
                .iter()
                .map(|&layer| CString::new(layer).unwrap())
                .collect()
        } else {
            vec![]
        };
        let raw_validation_layers: Vec<*const i8> = validation_layers
            .iter()
            .map(|layer| layer.as_ptr())
            .collect();

        let instance = crate::Instance::new(
            appbase,
            window,
            &raw_validation_layers,
            crate::constants::VULKAN_VERSION,
        )?;

        let debug_utils_messenger = if enable_validation_layers {
            crate::DebugUtilsMessenger::new(
                &appbase.entry,
                &instance,
                vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )?
        } else {
            crate::DebugUtilsMessenger::null(&appbase.entry, &instance)
        };

        let surface =
            crate::Surface::new(appbase, &instance, window).map_err(|err| format!("{}", err))?;

        let physical_device =
            instance.supported_physical_device(enabled_physical_device_features)?;

        let (device, command_pool) =
            Self::set_physical_device(&instance, physical_device, &surface)?;

        Ok(Self {
            instance,
            surface,
            debug_utils_messenger,
            device,
        })
    }

    pub fn extensions(&self) -> Vec<ash::vk::ExtensionProperties> {
        self.instance.extensions()
    }

    pub fn layers(&self) -> Vec<ash::vk::LayerProperties> {
        self.instance.layers()
    }

    pub fn physical_devices(&self) -> Vec<ash::vk::PhysicalDevice> {
        self.instance.physical_devices()
    }

    pub fn set_physical_device(
        instance: &crate::Instance,
        physical_device: ash::vk::PhysicalDevice,
        surface: &crate::Surface,
    ) -> Result<(crate::Device, crate::CommandPool), String> {
        use ash::{extensions::khr::Swapchain, vk};

        let required_extensions = vec![Swapchain::name().as_ptr()];
        let device_features = vk::PhysicalDeviceFeatures::default();
        let device = crate::Device::new(
            instance,
            physical_device,
            surface,
            &required_extensions,
            device_features,
            None,
        )?;

        let command_pool = crate::CommandPool::new(&device, device.graphics_queue_index(), true)?;

        Ok((device, command_pool))
    }

    fn create_swapchain(
        window: &crate::window::Window,
        instance: &crate::Instance,
        device: &crate::Device,
        surface: &crate::Surface,
        pool: &crate::CommandPool,
        present_mode: ash::vk::PresentModeKHR,
    ) -> Result<Swapchain, String> {
        let swapchain = crate::SwapChain::new(window, instance, device, surface, present_mode)?;
        let depth_buffer = crate::DepthBuffer::new(instance, device, pool, swapchain.extent())?;

        let mut image_available_semaphores: Vec<crate::Semaphore> = Vec::new();
        let mut render_finished_semaphores: Vec<crate::Semaphore> = Vec::new();

        let mut in_flight_fences: Vec<crate::Fence> = Vec::new();

        let mut uniform_buffers: Vec<crate::assets::UniformBuffer> = Vec::new();

        for _ in 0..swapchain.image_views.len() {
            let image_available_semaphore = crate::Semaphore::new(device)?;
            let render_finished_semaphore = crate::Semaphore::new(device)?;
            let in_flight_fence = crate::Fence::new(device, true)?;
            let uniform_buffer = crate::assets::UniformBuffer::new(instance, device)?;

            image_available_semaphores.push(image_available_semaphore);
            render_finished_semaphores.push(render_finished_semaphore);
            in_flight_fences.push(in_flight_fence);
            uniform_buffers.push(uniform_buffer);
        }

        Ok(Swapchain {
            swapchain,
            depth_buffer,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            uniform_buffers,
        })
    }

    fn delete_swapchain(&self) {}
}

impl Drop for App {
    fn drop(&mut self) {
        crate::Device::cleanup(&mut self.device);
        crate::Surface::cleanup(&mut self.surface);
        crate::debug::DebugUtilsMessenger::cleanup(&mut self.debug_utils_messenger);
        crate::Instance::cleanup(&mut self.instance);
    }
}
