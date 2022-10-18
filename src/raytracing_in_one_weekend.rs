pub struct AppBase;

impl AppBase {
    pub fn init_window(
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<winit::window::Window, String> {
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

    pub fn main_loop(event_loop: winit::event_loop::EventLoop<()>) {
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

pub struct RayTracingInOneWeekend {
    _entry: ash::Entry,
    instance: ash::Instance,

    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_callback: ash::vk::DebugUtilsMessengerEXT,

    surface_loader: ash::extensions::khr::Surface,
    surface: ash::vk::SurfaceKHR,

    _physical_device: ash::vk::PhysicalDevice,
    _msaa_samples: ash::vk::SampleCountFlags,
    _physical_device_memory_properties: ash::vk::PhysicalDeviceMemoryProperties,

    device: ash::Device,

    _queue_family: vk_utils::queue::QueueFamilyIndices,
    _graphics_queue: ash::vk::Queue,
    _present_queue: ash::vk::Queue,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: ash::vk::SwapchainKHR,
    swapchain_images: Vec<ash::vk::Image>,
    _swapchain_format: ash::vk::Format,
    _swapchain_extent: ash::vk::Extent2D,
    swapchain_image_views: Vec<ash::vk::ImageView>,

    render_pass: ash::vk::RenderPass,

    _descriptor_set_layout: ash::vk::DescriptorSetLayout,

    vertex3d_pipeline_layout: ash::vk::PipelineLayout,
    vertex3d_graphics_pipeline: ash::vk::Pipeline,
}

impl RayTracingInOneWeekend {
    pub fn new(window: &winit::window::Window) -> Self {
        use std::path::Path;
        use vk_utils::{
            attributes::*,
            constants::{FRAG_SHADER_PATH, VERT_SHADER_PATH},
            debug as vk_debug, types as vk_types,
        };

        let entry = ash::Entry::linked();
        let instance = vk_utils::create_instance(&entry, window).unwrap();

        let (debug_utils_loader, debug_callback) =
            vk_debug::setup_debug_callback(&entry, &instance).unwrap();

        let surface_info = vk_utils::surface::create_surface(&entry, &instance, window).unwrap();

        let physical_device =
            vk_utils::device::pick_physical_device(&instance, &surface_info).unwrap();

        let msaa_samples =
            vk_utils::device::get_max_usable_sample_count(&instance, physical_device);
        let physical_device_memory_properties =
            vk_utils::device::get_memory_property(&instance, physical_device);
        let _physical_device_properties =
            vk_utils::device::get_property(&instance, physical_device);

        let (device, family_indices) =
            vk_utils::device::create_logical_device(&instance, physical_device, &surface_info)
                .unwrap();
        let (graphics_queue, present_queue) = family_indices.device_queues(&device, 0, 0).unwrap();

        let swapchain_info = vk_utils::swapchain::create_swapchain(
            &instance,
            &device,
            physical_device,
            &surface_info,
            &family_indices,
        )
        .unwrap();
        let swapchain_image_views = vk_utils::image::create_image_views(
            &device,
            swapchain_info.swapchain_format,
            &swapchain_info.swapchain_images,
        )
        .unwrap();

        let render_pass = vk_utils::render_pass::create_render_pass(
            &instance,
            physical_device,
            msaa_samples,
            &device,
            swapchain_info.swapchain_format,
        )
        .unwrap();

        let descriptor_set_layout = vk_utils::Descriptor::set_layout(&device).unwrap();

        let vert_shader_path = Path::new(VERT_SHADER_PATH);
        let frag_shader_path = Path::new(FRAG_SHADER_PATH);

        let (graphics_pipeline, pipeline_layout) = vk_types::Vertex3D::create_graphics_pipeline(
            &device,
            msaa_samples,
            swapchain_info.swapchain_extent,
            render_pass,
            descriptor_set_layout,
            &vert_shader_path,
            &frag_shader_path,
        )
        .unwrap();

        Self {
            _entry: entry,
            instance,

            debug_utils_loader,
            debug_callback,

            surface_loader: surface_info.surface_loader,
            surface: surface_info.surface,

            _physical_device: physical_device,
            _msaa_samples: msaa_samples,
            _physical_device_memory_properties: physical_device_memory_properties,

            device,

            _queue_family: family_indices,
            _graphics_queue: graphics_queue,
            _present_queue: present_queue,

            swapchain_loader: swapchain_info.swapchain_loader,
            swapchain: swapchain_info.swapchain,
            swapchain_images: swapchain_info.swapchain_images,
            _swapchain_format: swapchain_info.swapchain_format,
            _swapchain_extent: swapchain_info.swapchain_extent,
            swapchain_image_views,

            render_pass,
            _descriptor_set_layout: descriptor_set_layout,

            vertex3d_pipeline_layout: pipeline_layout,
            vertex3d_graphics_pipeline: graphics_pipeline,
        }
    }

    fn cleanup_swapchain(&self) {
        unsafe {
            self.device
                .destroy_pipeline(self.vertex3d_graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.vertex3d_pipeline_layout, None);

            self.device.destroy_render_pass(self.render_pass, None);

            for &swapchain_image_view in self.swapchain_image_views.iter() {
                self.device.destroy_image_view(swapchain_image_view, None);
            }
            for &swapchain_image in self.swapchain_images.iter() {
                self.device.destroy_image(swapchain_image, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }

    #[allow(dead_code)]
    fn recreate_swapchain(&mut self) -> Result<(), String> {
        let result = unsafe { self.device.device_wait_idle() };

        if result.is_err() {
            return Err(String::from("failed to wait device idle!"));
        }

        self.cleanup_swapchain();

        Ok(())
    }
}

impl Drop for RayTracingInOneWeekend {
    fn drop(&mut self) {
        use vk_utils::constants::VK_VALIDATION_LAYER_NAMES;

        unsafe {
            self.cleanup_swapchain();

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
