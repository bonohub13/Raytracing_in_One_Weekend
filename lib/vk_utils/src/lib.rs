pub mod types;
pub mod utils;
pub mod vk_init;
pub mod window;

pub mod constants {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;
    pub const APPLICATION_NAME: &'static str = "Ray Tracing in Vulkan";
    pub const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 0, 1, 0);
    pub const ENGINE_NAME: &'static str = "Ray Tracer (Vulkan-Compute)";
    pub const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 0, 1, 0);
}

mod engine;

pub use engine::Engine;

pub struct QueueFamilyIndices {
    pub compute_family_index: u32,
    pub graphics_family_index: u32,
    pub present_family_index: u32,
}

pub struct ExpectedQueues {
    pub compute: ash::vk::Queue,
    pub graphics: ash::vk::Queue,
    pub present: ash::vk::Queue,
}

pub struct VkBuffer {
    pub buffer: ash::vk::Buffer,
    pub memory: ash::vk::DeviceMemory,
    pub size: ash::vk::DeviceSize,
}

impl VkBuffer {
    pub fn unmap_buffer(device: &ash::Device, buffer: &mut Self) {
        log::info!("unmmaping memory for buffer");

        unsafe {
            device.unmap_memory(buffer.memory);
        }

        log::info!("unmapped memory for buffer")
    }

    pub fn cleanup(device: &ash::Device, buffer: &mut Self) {
        log::info!("performing cleanup for VkBuffer");

        unsafe {
            device.destroy_buffer(buffer.buffer, None);
            device.free_memory(buffer.memory, None);
        }
    }
}

pub struct VkImage {
    pub image: ash::vk::Image,
    pub memory: ash::vk::DeviceMemory,
    pub sampler: ash::vk::Sampler,
    pub image_view: ash::vk::ImageView,
}

impl VkImage {
    pub fn cleanup(device: &ash::Device, image: &mut Self) {
        log::info!("performing cleanup VkImage");

        unsafe {
            device.destroy_image_view(image.image_view, None);
            device.destroy_sampler(image.sampler, None);
            device.destroy_image(image.image, None);
            device.free_memory(image.memory, None);
        }
    }
}

pub struct VkRenderCallInfo {
    pub number: u32,
    pub total_render_calls: u32,
    pub total_samples: u32,
}

pub struct VkSurface {
    surface_loader: ash::extensions::khr::Surface,
    pub surface: ash::vk::SurfaceKHR,
}

impl VkSurface {
    pub fn cleanup(surface: &mut Self) {
        log::info!("performing cleanup for VkSurface");

        unsafe {
            surface
                .surface_loader
                .destroy_surface(surface.surface, None);
        }
    }

    pub fn get_physical_device_surface_support(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> Result<bool, String> {
        unsafe {
            self.surface_loader
                .get_physical_device_surface_support(
                    physical_device,
                    queue_family_index,
                    self.surface,
                )
                .map_err(|_| String::from("failed to get surface support for physical device"))
        }
    }

    pub fn get_physical_device_surface_capabilities(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<ash::vk::SurfaceCapabilitiesKHR, String> {
        log::info!("getting surface capabilities for physical device");

        unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(physical_device, self.surface)
                .map_err(|_| String::from("failed to get surface capabities for physical device"))
        }
    }

    pub fn get_physical_device_surface_formats(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Vec<ash::vk::SurfaceFormatKHR>, String> {
        log::info!("getting surface formats for physical device");

        unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(physical_device, self.surface)
                .map_err(|_| String::from("failed to get surface formats for physical device"))
        }
    }

    pub fn get_physical_device_surface_present_modes(
        &self,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<Vec<ash::vk::PresentModeKHR>, String> {
        log::info!("getting surface present modes for physical device");

        unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(physical_device, self.surface)
                .map_err(|_| {
                    String::from("failed to get surface present modes for physical device")
                })
        }
    }

    pub fn find_suitable_swap_chain_surface_format(
        &self,
        physical_device: ash::vk::PhysicalDevice,
        swapchain_image_format: ash::vk::Format,
        color_space: ash::vk::ColorSpaceKHR,
    ) -> Result<ash::vk::SurfaceFormatKHR, String> {
        log::info!("finding suitable surface format for swap chain");

        let available_formats = self.get_physical_device_surface_formats(physical_device)?;
        match available_formats.iter().find(|available_format| {
            available_format.format == swapchain_image_format
                && available_format.color_space == color_space
        }) {
            Some(&surface_format) => {
                log::info!("found suitable surface format for swap chain");

                Ok(surface_format)
            }
            None => {
                log::warn!("defaulting to first surface format");

                match available_formats.first() {
                    Some(&surface_first) => Ok(surface_first),
                    None => Err(String::from("failed to find any surface format")),
                }
            }
        }
    }
}

pub struct VkSwapchain {
    loader: ash::extensions::khr::Swapchain,
    swapchain: ash::vk::SwapchainKHR,
    extent: ash::vk::Extent2D,
    images: Vec<ash::vk::Image>,
    image_views: Vec<ash::vk::ImageView>,
}

impl VkSwapchain {
    pub fn cleanup(device: &ash::Device, swapchain: &mut Self) {
        log::info!("performing cleanup for VkSwapchain");

        unsafe {
            for &iv in swapchain.image_views.iter() {
                device.destroy_image_view(iv, None);
            }
            swapchain
                .loader
                .destroy_swapchain(swapchain.swapchain, None);
        }
    }
}

pub struct VkSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub compute_shader_file: &'static str,
    pub compute_shader_group_size_x: u32,
    pub compute_shader_group_size_y: u32,
}

pub struct AppBase {
    entry: ash::Entry,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl AppBase {
    pub fn new() -> Self {
        Self {
            entry: ash::Entry::linked(),
            event_loop: winit::event_loop::EventLoop::new(),
        }
    }

    pub fn entry(&self) -> &ash::Entry {
        &self.entry
    }

    pub fn event_loop(&self) -> &winit::event_loop::EventLoop<()> {
        &self.event_loop
    }

    pub fn run(&mut self) {
        use winit::platform::run_return::EventLoopExtRunReturn;

        self.event_loop.run_return(|event, _, control_flow| {
            use winit::event::{Event, WindowEvent};

            control_flow.set_poll();
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => (),
                },
                Event::MainEventsCleared => (),
                _ => (),
            }
        });
    }
}
