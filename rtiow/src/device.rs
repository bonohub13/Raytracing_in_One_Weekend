// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Instance, RtErr, RtError, Surface};
use ash::{ext::vertex_input_dynamic_state, khr::swapchain, vk};
use std::{collections::HashSet, ffi::CStr};

pub struct Device {
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    device: ash::Device,
    physical_device: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct QueueFamilyIndices {
    pub(crate) graphics_family: Option<u32>,
    pub(crate) present_family: Option<u32>,
}

impl Device {
    const DEVICE_EXTENSIONS: [&CStr; 2] = [swapchain::NAME, vertex_input_dynamic_state::NAME];

    pub(crate) fn new(instance: &Instance, surface: &Surface) -> RtErr<Self> {
        let physical_device = Self::query_physical_device(instance, surface)?;
        let (device, graphics_queue, present_queue) =
            Self::create_device(instance, surface, physical_device)?;
        let properties = Self::query_device_properties(instance, physical_device);
        let memory_properties = Self::query_memory_properties(instance, physical_device);

        Ok(Self {
            physical_device,
            device,
            graphics_queue,
            present_queue,
            properties,
            memory_properties,
        })
    }

    #[inline]
    pub(crate) fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    #[inline]
    pub(crate) fn device(&self) -> &ash::Device {
        &self.device
    }

    #[inline]
    pub(crate) fn graphics_queue(&self) -> vk::Queue {
        self.graphics_queue
    }

    #[inline]
    pub(crate) fn present_queue(&self) -> vk::Queue {
        self.present_queue
    }

    #[inline]
    pub(crate) fn properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.properties
    }

    #[inline]
    pub(crate) fn memory_properties(&self) -> &vk::PhysicalDeviceMemoryProperties {
        &self.memory_properties
    }

    #[inline]
    pub(crate) fn device_wait_idle(&self) -> RtErr<()> {
        unsafe { self.device.device_wait_idle() }.map_err(|err| RtError::DeviceWaitIdle(err.into()))
    }

    fn query_physical_device(instance: &Instance, surface: &Surface) -> RtErr<vk::PhysicalDevice> {
        let rate_device_suitability = |device| Self::rate_device_suitability(instance, device);
        let is_suitable_device = |device| Self::is_suitable_device(instance, surface, device);
        let mut devices = unsafe { instance.instance().enumerate_physical_devices() }
            .map_err(|err| RtError::EnumeratePhysicalDevices(err.into()))?;

        devices.sort_by(|device, next_device| {
            rate_device_suitability(*device).cmp(&rate_device_suitability(*next_device))
        });
        if let Some(device) = devices
            .iter()
            .find(|device| is_suitable_device(**device).unwrap_or_default())
        {
            Ok(*device)
        } else {
            Err(RtError::FindSuitableDevice)
        }
    }

    fn create_device(
        instance: &Instance,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> RtErr<(ash::Device, vk::Queue, vk::Queue)> {
        const QUEUE_PRIORITIES: [f32; 1] = [1f32];

        let indices = surface.find_queue_families(instance, device)?;

        if let (Some(graphics_family), Some(present_family)) =
            (indices.graphics_family, indices.present_family)
        {
            let instance = instance.instance();
            let is_unique_queue_families = graphics_family == present_family;
            let queue_create_infos = if is_unique_queue_families {
                vec![vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(graphics_family)
                    .queue_priorities(&QUEUE_PRIORITIES)]
            } else {
                vec![
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(graphics_family)
                        .queue_priorities(&QUEUE_PRIORITIES),
                    vk::DeviceQueueCreateInfo::default()
                        .queue_family_index(present_family)
                        .queue_priorities(&QUEUE_PRIORITIES),
                ]
            };

            // Enable specific device features
            let core_features = vk::PhysicalDeviceFeatures::default().sample_rate_shading(true);
            let mut shader_draw_parameters =
                vk::PhysicalDeviceShaderDrawParametersFeatures::default()
                    .shader_draw_parameters(true);
            let mut dynamic_state = vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT::default()
                .vertex_input_dynamic_state(true);
            let mut dynamic_rendering =
                vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);
            let mut synchronization2 =
                vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true);
            let mut device_features = vk::PhysicalDeviceFeatures2::default()
                .features(core_features)
                .push_next(&mut shader_draw_parameters)
                .push_next(&mut dynamic_state)
                .push_next(&mut dynamic_rendering)
                .push_next(&mut synchronization2);

            unsafe { instance.get_physical_device_features2(device, &mut device_features) };

            let extension_names: Vec<_> = Self::DEVICE_EXTENSIONS
                .iter()
                .map(|extension| extension.as_ptr())
                .collect();
            let create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&extension_names)
                .push_next(&mut device_features);
            let device = unsafe { instance.create_device(device, &create_info, None) }
                .map_err(|err| RtError::CreateDevice(err.into()))?;
            let (graphics_queue, present_queue) = if is_unique_queue_families {
                let queue_info = vk::DeviceQueueInfo2::default()
                    .queue_family_index(graphics_family)
                    .queue_index(0);
                let queue = unsafe { device.get_device_queue2(&queue_info) };

                (queue, queue)
            } else {
                let graphics_queue_info = vk::DeviceQueueInfo2::default()
                    .queue_family_index(graphics_family)
                    .queue_index(0);
                let present_queue_info = vk::DeviceQueueInfo2::default()
                    .queue_family_index(present_family)
                    .queue_index(0);
                let graphics_queue = unsafe { device.get_device_queue2(&graphics_queue_info) };
                let present_queue = unsafe { device.get_device_queue2(&present_queue_info) };

                (graphics_queue, present_queue)
            };

            Ok((device, graphics_queue, present_queue))
        } else {
            Err(RtError::FindSuitableDevice)
        }
    }

    fn query_device_properties(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        let mut properties = vk::PhysicalDeviceProperties2::default();

        unsafe {
            instance
                .instance()
                .get_physical_device_properties2(physical_device, &mut properties)
        };

        properties.properties
    }

    fn query_memory_properties(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties2::default();

        unsafe {
            instance
                .instance()
                .get_physical_device_memory_properties2(physical_device, &mut properties)
        };

        properties.memory_properties
    }

    fn is_suitable_device(
        instance: &Instance,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> RtErr<bool> {
        let indices = surface.find_queue_families(instance, device)?;
        let extensions_supported = Self::check_device_extension_support(instance, device)?;
        let swapchain_support = if extensions_supported {
            let swapchain_support = surface.query_swapchain_support(device)?;

            !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty()
        } else {
            false
        };

        Ok(indices.is_complete() && swapchain_support)
    }

    fn check_device_extension_support(
        instance: &Instance,
        device: vk::PhysicalDevice,
    ) -> RtErr<bool> {
        let instance = instance.instance();
        let available_extensions =
            unsafe { instance.enumerate_device_extension_properties(device) }
                .map_err(|err| RtError::EnumerateDeviceExtensionProperties(err.into()))?;
        let available_extensions: Vec<_> = available_extensions
            .iter()
            .map(|extension| unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) })
            .collect();

        Ok(!Self::DEVICE_EXTENSIONS
            .iter()
            .any(|extension| !available_extensions.contains(extension)))
    }

    fn rate_device_suitability(instance: &Instance, device: vk::PhysicalDevice) -> u32 {
        let instance = instance.instance();
        let device_properties = {
            let mut properties = vk::PhysicalDeviceProperties2::default();

            unsafe {
                instance.get_physical_device_properties2(device, &mut properties);

                properties.properties
            }
        };
        let mut shader_draw_parameters = vk::PhysicalDeviceShaderDrawParametersFeatures::default();
        let mut dynamic_state = vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT::default();
        let mut dynamic_rendering = vk::PhysicalDeviceDynamicRenderingFeatures::default();
        let mut synchronization2 = vk::PhysicalDeviceSynchronization2Features::default();
        let mut device_features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut shader_draw_parameters)
            .push_next(&mut dynamic_state)
            .push_next(&mut dynamic_rendering)
            .push_next(&mut synchronization2);

        unsafe {
            instance.get_physical_device_features2(device, &mut device_features);
        }

        if device_features.features.geometry_shader == 0
            || device_features.features.sample_rate_shading == 0
            || shader_draw_parameters.shader_draw_parameters == 0
            || dynamic_state.vertex_input_dynamic_state == 0
            || dynamic_rendering.dynamic_rendering == 0
            || synchronization2.synchronization2 == 0
        {
            0
        } else {
            device_properties.limits.max_image_dimension2_d
                + if device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                    1000u32
                } else {
                    0u32
                }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}

impl QueueFamilyIndices {
    pub(crate) fn unique_queue_families(&self) -> RtErr<Vec<u32>> {
        if !self.is_complete() {
            return Err(RtError::FindSuitableDevice);
        }

        let queue_families: HashSet<u32> = [self.graphics_family, self.present_family]
            .into_iter()
            .flatten()
            .collect();

        Ok(queue_families.into_iter().collect())
    }

    #[inline]
    pub(crate) fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}
