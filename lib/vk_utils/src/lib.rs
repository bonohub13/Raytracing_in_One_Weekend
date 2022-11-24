pub mod constants {
    // Application stuff
    pub const APPLICATION_NAME: &str = "Ray Tracing in One Weekend";
    pub const APPLICATION_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);

    // Engine stuff
    pub const ENGINE_NAME: &str = "No Engine";
    pub const ENGINE_VERSION: u32 = ash::vk::make_api_version(0, 1, 0, 0);

    pub const VULKAN_VERSION: u32 = ash::vk::API_VERSION_1_2;
}

pub mod application;
pub mod assets;
pub mod utils;
pub mod window;

mod buffer;
mod command_buffer;
mod command_pool;
mod debug;
mod depth_buffer;
mod descriptor_binding;
mod descriptor_pool;
mod descriptor_set_layout;
mod descriptor_set_manager;
mod descriptor_sets;
mod device;
mod device_memory;
mod fence;
mod graphics_pipeline;
mod image;
mod image_view;
mod instance;
mod semaphore;
mod surface;
mod swapchain;

pub use buffer::Buffer;
pub use command_buffer::CommandBuffers;
pub use command_pool::CommandPool;
pub use debug::{vulkan_debug_callback, DebugUtilsMessenger};
pub use depth_buffer::DepthBuffer;
pub use descriptor_binding::DescriptorBinding;
pub use descriptor_pool::DescriptorPool;
pub use descriptor_set_layout::DescriptorSetLayout;
pub use device::{Device, QueueFamilyIndices};
pub use device_memory::DeviceMemory;
pub use fence::Fence;
pub use image::Image;
pub use image_view::ImageView;
pub use instance::{Instance, PhysicalDeviceRequiredFeatures};
pub use semaphore::Semaphore;
pub use surface::Surface;
pub use swapchain::{SupportDetails, SwapChain};

pub struct AppBase {
    pub entry: ash::Entry,
    pub event_loop: winit::event_loop::EventLoop<()>,
}

impl AppBase {
    pub fn new() -> Self {
        use winit::event_loop::EventLoop;

        Self {
            entry: ash::Entry::linked(),
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(&mut self, window: &window::Window) {
        use winit::platform::run_return::EventLoopExtRunReturn;

        self.event_loop.run_return(|event, _, control_flow| {
            use winit::event::{Event, WindowEvent};

            control_flow.set_poll();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => {}
                },
                Event::MainEventsCleared => {
                    // Do not do anything if window is minimized
                    // Could not use self.is_minimized() due to event_loop
                    // being borrowed once as mutable
                    if window.is_minimized() {
                        return;
                    }
                    // Draw here
                }
                _ => {}
            }
        });
    }
}
