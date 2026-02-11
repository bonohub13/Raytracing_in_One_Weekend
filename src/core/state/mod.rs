mod utils;

use crate::core::{
    self, RtError, RtResult,
    debug::DebugUtils,
    objects::Camera,
    renderer::{self, PathTracer},
    shader::{ComputeShaderDescriptor, RenderShaderDescriptor},
    surface::Surface,
};
use ash::{Device, Instance, khr, vk};
use std::{collections::HashSet, ffi::CStr, sync::Arc};
use winit::window::Window;

pub struct StateDescriptor<'name> {
    pub app_name: &'name str,
    pub window: Arc<Window>,
    pub render_shader_desc: RenderShaderDescriptor<'name>,
    pub compute_shader_desc: ComputeShaderDescriptor<'name>,
}

pub struct State {
    instance: Instance,
    debug_utils: Option<DebugUtils>,
    surface: Surface,
    physical_device: vk::PhysicalDevice,
    device: Device,
    compute_queue: vk::Queue,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    path_tracer: PathTracer,
    current_frame: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct QueueFamilyIndices {
    pub compute_family: Option<u32>,
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl State {
    #[cfg(debug_assertions)]
    const VALIDATION_LAYERS: [&'static CStr; 1] = [c"VK_LAYER_KHRONOS_validation"];
    #[cfg(not(debug_assertions))]
    const VALIDATION_LAYERS: [&'static CStr; 0] = [];
    const DEVICE_EXTENSIONS: [&'static CStr; 2] =
        [khr::swapchain::NAME, khr::shader_draw_parameters::NAME];

    pub fn new(desc: &StateDescriptor) -> RtResult<Self> {
        let entry = ash::Entry::linked();
        let instance = Self::create_instance(desc, &entry)?;
        #[cfg(debug_assertions)]
        let debug_utils = Some(DebugUtils::new(&entry, &instance)?);
        #[cfg(not(debug_assertions))]
        let debug_utils = None;
        let surface = Surface::new(desc.window.clone(), &entry, &instance)?;
        let (physical_device, queue_family) = Self::get_valid_physical_device(&instance, &surface)?;
        let (device, compute_queue, graphics_queue, present_queue) =
            Self::create_device(&instance, &physical_device, &queue_family)?;
        let path_tracer = PathTracer::new(&renderer::PathTracerDescriptor {
            window: desc.window.clone(),
            instance: &instance,
            surface: &surface,
            physical_device: &physical_device,
            device: &device,
            render_shader_desc: &desc.render_shader_desc,
            compute_shader_desc: &desc.compute_shader_desc,
            camera: &Camera::new(desc.window.clone()),
        })?;

        Ok(Self {
            instance,
            debug_utils,
            surface,
            physical_device,
            device,
            compute_queue,
            graphics_queue,
            present_queue,
            path_tracer,
            current_frame: 0,
        })
    }

    pub fn draw_frame(&mut self, window: Arc<Window>) -> RtResult<()> {
        self.path_tracer
            .compute_frame(&self.device, self.current_frame)?;
        self.compute_queue_submit()?;

        let image_index = self.path_tracer.draw_frame(
            window,
            &self.instance,
            &self.surface,
            &self.physical_device,
            &self.device,
            self.current_frame,
        )?;

        self.graphics_queue_submit()?;
        self.path_tracer.swapchain().queue_present(
            &self.present_queue,
            self.path_tracer.sync_object(),
            image_index as u32,
            self.current_frame,
        )?;
        self.current_frame = (self.current_frame + 1) % core::MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }

    pub fn resize(&mut self, window: Arc<Window>) -> RtResult<()> {
        self.path_tracer.resize(
            window,
            &self.instance,
            &self.surface,
            &self.physical_device,
            &self.device,
        )
    }

    pub fn device_wait_idle(&self) -> RtResult<()> {
        if let Err(err) = unsafe { self.device.device_wait_idle() } {
            Err(RtError::DeviceWaitIdle(err.into()))
        } else {
            Ok(())
        }
    }

    fn compute_queue_submit(&self) -> RtResult<()> {
        let sync_object = self.path_tracer.sync_object();
        let submit_info = vk::SubmitInfo::default()
            .command_buffers(std::slice::from_ref(
                self.path_tracer
                    .command()
                    .compute_buffer(self.current_frame),
            ))
            .signal_semaphores(std::slice::from_ref(
                sync_object.compute_finished_semaphore(self.current_frame),
            ));

        if let Err(err) = unsafe {
            self.device.queue_submit(
                self.compute_queue,
                std::slice::from_ref(&submit_info),
                *sync_object.compute_in_flight_fence(self.current_frame),
            )
        } {
            Err(RtError::QueueSubmit(err.into()))
        } else {
            Ok(())
        }
    }

    fn graphics_queue_submit(&self) -> RtResult<()> {
        const WAIT_STAGES: [vk::PipelineStageFlags; 2] = [
            vk::PipelineStageFlags::VERTEX_INPUT,
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        ];

        let sync_object = self.path_tracer.sync_object();
        let wait_semaphores = [
            *sync_object.compute_finished_semaphore(self.current_frame),
            *sync_object.image_available_semaphore(self.current_frame),
        ];
        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&WAIT_STAGES)
            .command_buffers(std::slice::from_ref(
                self.path_tracer.command().buffer(self.current_frame),
            ))
            .signal_semaphores(std::slice::from_ref(
                sync_object.render_finished_semaphore(self.current_frame),
            ));

        if let Err(err) = unsafe {
            self.device.queue_submit(
                self.graphics_queue,
                std::slice::from_ref(&submit_info),
                *sync_object.in_flight_fence(self.current_frame),
            )
        } {
            Err(RtError::QueueSubmit(err.into()))
        } else {
            Ok(())
        }
    }

    fn create_instance(desc: &StateDescriptor, entry: &ash::Entry) -> RtResult<Instance> {
        #[cfg(debug_assertions)]
        const ENABLED_FEATURES: [vk::ValidationFeatureEnableEXT; 4] = [
            vk::ValidationFeatureEnableEXT::SYNCHRONIZATION_VALIDATION,
            vk::ValidationFeatureEnableEXT::BEST_PRACTICES,
            vk::ValidationFeatureEnableEXT::GPU_ASSISTED,
            vk::ValidationFeatureEnableEXT::GPU_ASSISTED_RESERVE_BINDING_SLOT,
        ];
        Self::check_validation_support(entry)?;

        let validation_layers = Self::check_validation_support(entry)?;
        let extensions = Self::required_extensions(desc.window.clone(), entry)?;
        let validation_layers = validation_layers
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<_>>();
        let extensions = extensions
            .iter()
            .map(|extension| extension.as_ptr())
            .collect::<Vec<_>>();
        let app_info = vk::ApplicationInfo::default()
            .api_version(vk::API_VERSION_1_3)
            .application_name(unsafe { CStr::from_ptr(desc.app_name.as_ptr() as *const i8) });
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .flags(
                vk::InstanceCreateFlags::empty()
                    | vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR,
            )
            .enabled_layer_names(&validation_layers)
            .enabled_extension_names(&extensions);

        #[cfg(debug_assertions)]
        let mut messenger_create_info = DebugUtils::messenger_create_info();
        #[cfg(debug_assertions)]
        let mut validation_features =
            vk::ValidationFeaturesEXT::default().enabled_validation_features(&ENABLED_FEATURES);
        #[cfg(debug_assertions)]
        let create_info = create_info
            .push_next(&mut messenger_create_info)
            .push_next(&mut validation_features);

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => Ok(instance),
            Err(e) => Err(RtError::CreateInstance(e.into())),
        }
    }

    fn get_valid_physical_device(
        instance: &Instance,
        surface: &Surface,
    ) -> RtResult<(vk::PhysicalDevice, QueueFamilyIndices)> {
        let mut devices = match unsafe { instance.enumerate_physical_devices() } {
            Ok(devices) => Ok(devices),
            Err(e) => Err(RtError::EnumeratePhysicalDevices(e.into())),
        }?;

        devices.sort_by_key(|device: &vk::PhysicalDevice| {
            u32::MAX - Self::sort_by_device_suitability(instance, device)
        });

        if let Some((device, queue_family)) = devices.iter().find_map(|device| {
            Self::suitable_device(instance, surface, device)
                .map(|queue_family| (device, queue_family))
        }) {
            Ok((*device, queue_family))
        } else {
            Err(RtError::NoSuitablePhysicalDevice)
        }
    }

    fn create_device(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
        queue_family: &QueueFamilyIndices,
    ) -> RtResult<(ash::Device, vk::Queue, vk::Queue, vk::Queue)> {
        const QUEUE_PRIORITIES: [f32; 1] = [1f32];
        let unique_queue_indices = queue_family.unique_queue_indices()?;
        let queue_indices = queue_family.queue_indices()?;
        let queue_create_infos: Vec<_> = unique_queue_indices
            .iter()
            .map(|queue_family| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(*queue_family)
                    .queue_priorities(&QUEUE_PRIORITIES)
            })
            .collect();
        let features = unsafe { instance.get_physical_device_features(*physical_device) };
        let extension_names: Vec<_> = Self::DEVICE_EXTENSIONS
            .iter()
            .map(|extension| extension.as_ptr())
            .collect();
        let create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&features)
            .enabled_extension_names(&extension_names);
        let device = match unsafe { instance.create_device(*physical_device, &create_info, None) } {
            Ok(device) => Ok(device),
            Err(e) => Err(RtError::CreateDevice(e.into())),
        }?;
        let queues: Vec<_> = unique_queue_indices
            .iter()
            .map(|queue_family| unsafe { device.get_device_queue(*queue_family, 0) })
            .collect();
        let (compute_queue, graphics_queue, present_queue) = if queues.len() == 1 {
            (queues[0], queues[0], queues[0])
        } else if queues.len() == 3 {
            (queues[0], queues[1], queues[2])
        } else if queue_indices[0] == queue_indices[1] {
            (queues[0], queues[0], queues[1])
        } else if queue_indices[0] == queue_indices[2] {
            (queues[0], queues[1], queues[0])
        } else {
            (queues[0], queues[1], queues[1])
        };

        Ok((device, compute_queue, graphics_queue, present_queue))
    }
}

impl Drop for State {
    fn drop(&mut self) {
        if let Err(err) = self.device_wait_idle() {
            eprintln!("{err}");
        }
        if let Err(err) = unsafe { self.path_tracer.destroy(&self.device) } {
            eprintln!("{err}");
        }
        unsafe {
            self.device.destroy_device(None);
            self.surface.destroy()
        };
        if let Some(debug_utils) = self.debug_utils.as_mut() {
            unsafe { debug_utils.destroy() };
        }
        unsafe { self.instance.destroy_instance(None) };
    }
}

impl QueueFamilyIndices {
    pub(crate) fn find_queue_families(
        instance: &Instance,
        surface: &Surface,
        physical_device: &vk::PhysicalDevice,
    ) -> QueueFamilyIndices {
        let mut queue_family = QueueFamilyIndices::default();

        for (i, property) in
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) }
                .iter()
                .enumerate()
        {
            if property.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                queue_family.compute_family = Some(i as u32);
            }

            if property.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                queue_family.graphics_family = Some(i as u32);
            }

            if let Ok(supported) = surface.get_surface_support(physical_device, i as u32)
                && supported
            {
                queue_family.present_family = Some(i as u32);
            }

            if queue_family.is_complete() {
                break;
            }
        }

        queue_family
    }

    #[inline]
    fn is_complete(&self) -> bool {
        self.compute_family.is_some()
            && self.graphics_family.is_some()
            && self.present_family.is_some()
    }

    pub(crate) fn unique_queue_indices(&self) -> RtResult<Vec<u32>> {
        if let (Some(compute_family), Some(graphics_family), Some(present_family)) = (
            self.compute_family,
            self.graphics_family,
            self.present_family,
        ) {
            let mut unique_queue_indices: HashSet<u32> = HashSet::new();
            let queue_families: Vec<_> = [compute_family, graphics_family, present_family]
                .iter()
                .filter_map(|queue_family| {
                    if unique_queue_indices.insert(*queue_family) {
                        Some(*queue_family)
                    } else {
                        None
                    }
                })
                .collect();

            Ok(queue_families)
        } else {
            Err(RtError::NoSuitablePhysicalDevice)
        }
    }

    fn queue_indices(&self) -> RtResult<Vec<u32>> {
        if let (Some(compute_family), Some(graphics_family), Some(present_family)) = (
            self.compute_family,
            self.graphics_family,
            self.present_family,
        ) {
            let queue_families: Vec<_> = vec![compute_family, graphics_family, present_family];

            Ok(queue_families)
        } else {
            Err(RtError::NoSuitablePhysicalDevice)
        }
    }
}
