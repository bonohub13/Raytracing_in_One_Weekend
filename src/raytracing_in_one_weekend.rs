mod appbase {
    use winit::{event_loop::EventLoop, window::Window};

    pub struct AppBase;

    impl AppBase {
        pub fn init_window(event_loop: &EventLoop<()>) -> Result<Window, String> {
            use vk_utils::constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
            use winit::{dpi::LogicalSize, window::WindowBuilder};

            let window = WindowBuilder::new()
                .with_title(WINDOW_TITLE)
                .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
                .build(event_loop);

            match window {
                Ok(window) => Ok(window),
                Err(err) => Err(err.to_string()),
            }
        }

        pub fn main_loop(event_loop: EventLoop<()>) {
            use winit::{
                event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
                event_loop::ControlFlow,
            };

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                virtual_keycode,
                                state,
                                ..
                            } => match (virtual_keycode, state) {
                                (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                    *control_flow = ControlFlow::Exit;
                                }
                                _ => {}
                            },
                        },
                        _ => {}
                    },
                    _ => (),
                }
            })
        }
    }
}

mod rt_in_one_weekend {
    use vk_utils::queue::QueueFamilyIndices;

    use ash::{
        extensions::{ext::DebugUtils, khr::Surface},
        vk,
    };

    use winit::window::Window;

    pub struct RayTracingInOneWeekend {
        _entry: ash::Entry,
        instance: ash::Instance,

        debug_utils_loader: DebugUtils,
        debug_callback: vk::DebugUtilsMessengerEXT,

        surface_loader: Surface,
        surface: vk::SurfaceKHR,

        physical_device: vk::PhysicalDevice,
        msaa_samples: vk::SampleCountFlags,
        physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,

        device: ash::Device,

        queue_family: QueueFamilyIndices,
    }

    impl RayTracingInOneWeekend {
        pub fn new(window: &Window) -> Self {
            use vk_utils::debug as vk_debug;

            let entry = ash::Entry::linked();
            let instance = vk_utils::create_instance(&entry, window).unwrap();

            let (debug_utils_loader, debug_callback) =
                vk_debug::setup_debug_callback(&entry, &instance).unwrap();

            let surface_info =
                vk_utils::surface::create_surface(&entry, &instance, window).unwrap();

            let physical_device =
                vk_utils::device::pick_physical_device(&instance, &surface_info).unwrap();

            let msaa_samples =
                vk_utils::device::get_max_usable_sample_count(&instance, physical_device);
            let physical_device_memory_properties =
                vk_utils::device::get_memory_property(&instance, physical_device);
            let physical_device_properties =
                vk_utils::device::get_property(&instance, physical_device);

            let (device, family_indices) =
                vk_utils::device::create_logical_device(&instance, physical_device, &surface_info)
                    .unwrap();

            Self {
                _entry: entry,
                instance,

                debug_utils_loader,
                debug_callback,

                surface_loader: surface_info.surface_loader,
                surface: surface_info.surface,

                physical_device,
                msaa_samples,
                physical_device_memory_properties,

                device,

                queue_family: family_indices,
            }
        }
    }

    impl Drop for RayTracingInOneWeekend {
        fn drop(&mut self) {
            use vk_utils::constants::VK_VALIDATION_LAYER_NAMES;

            unsafe {
                self.device.destroy_device(None);

                self.surface_loader.destroy_surface(self.surface, None);

                if VK_VALIDATION_LAYER_NAMES.is_enable {
                    self.debug_utils_loader
                        .destroy_debug_utils_messenger(self.debug_callback, None);
                }

                self.instance.destroy_instance(None);
            }
        }
    }
}

pub use appbase::AppBase;
pub use rt_in_one_weekend::RayTracingInOneWeekend;
