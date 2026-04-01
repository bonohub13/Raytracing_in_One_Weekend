// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Instance, RtErr, RtError, Surface};
use ash::vk;
use std::sync::Arc;

pub struct Device {
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl Device {
    pub(crate) fn new(instance: Arc<Instance>, surface: &Surface) -> RtErr<Self> {
        let physical_device = Self::query_physical_device(instance.clone(), surface)?;
        let (device, graphics_queue, present_queue) =
            Self::create_device(instance, surface, physical_device)?;

        Ok(Self {
            physical_device,
            device,
            graphics_queue,
            present_queue,
        })
    }

    #[inline]
    pub(crate) fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    #[inline]
    pub(crate) fn device_wait_idle(&self) -> RtErr<()> {
        unsafe { self.device.device_wait_idle() }.map_err(|err| RtError::DeviceWaitIdle(err.into()))
    }

    fn query_physical_device(
        instance: Arc<Instance>,
        surface: &Surface,
    ) -> RtErr<vk::PhysicalDevice> {
        let rate_device_suitability =
            |device| Self::rate_device_suitability(instance.clone(), device);
        let is_suitable_device =
            |device| Self::is_suitable_device(instance.clone(), surface, device);
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
        instance: Arc<Instance>,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> RtErr<(ash::Device, vk::Queue, vk::Queue)> {
        const QUEUE_PRIORITIES: [f32; 1] = [1f32];

        let indices = QueueFamilyIndices::find_queue_families(instance.clone(), surface, device)?;

        if let (Some(graphics_family), Some(present_family)) =
            (indices.graphics_family, indices.present_family)
        {
            let instance = instance.instance();
            let queue_create_infos = if graphics_family == present_family {
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
            let device_features = {
                let mut features = vk::PhysicalDeviceFeatures2::default();

                unsafe { instance.get_physical_device_features2(device, &mut features) };

                features.features
            };
            let create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_features(&device_features);
            let device = unsafe { instance.create_device(device, &create_info, None) }
                .map_err(|err| RtError::CreateDevice(err.into()))?;
            let (graphics_queue, present_queue) = if graphics_family == present_family {
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

    fn is_suitable_device(
        instance: Arc<Instance>,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> RtErr<bool> {
        let indices = QueueFamilyIndices::find_queue_families(instance, surface, device)?;

        Ok(indices.is_complete())
    }

    fn rate_device_suitability(instance: Arc<Instance>, device: vk::PhysicalDevice) -> u32 {
        let instance = instance.instance();
        let device_properties = {
            let mut properties = vk::PhysicalDeviceProperties2::default();

            unsafe {
                instance.get_physical_device_properties2(device, &mut properties);

                properties.properties
            }
        };
        let device_features = {
            let mut features = vk::PhysicalDeviceFeatures2::default();

            unsafe {
                instance.get_physical_device_features2(device, &mut features);
            }

            features.features
        };

        if device_features.geometry_shader == 0 {
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

impl QueueFamilyIndices {
    fn find_queue_families(
        instance: Arc<Instance>,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> RtErr<Self> {
        let instance = instance.instance();
        let mut indices = Self::default();
        let queue_families = {
            let queue_family_len =
                unsafe { instance.get_physical_device_queue_family_properties2_len(device) };
            let mut queue_families = vec![vk::QueueFamilyProperties2::default(); queue_family_len];

            unsafe {
                instance.get_physical_device_queue_family_properties2(device, &mut queue_families)
            };

            queue_families
        };

        for (i, queue_family) in queue_families.iter().enumerate() {
            let current_family_index = i as u32;
            if queue_family
                .queue_family_properties
                .queue_flags
                .contains(vk::QueueFlags::GRAPHICS)
            {
                indices.graphics_family = Some(current_family_index)
            }

            if surface.get_physical_device_surface_support(device, current_family_index)? {
                indices.present_family = Some(current_family_index)
            }

            if indices.is_complete() {
                break;
            }
        }

        Ok(indices)
    }

    #[inline]
    pub(crate) fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        while let Err(err) = self.device_wait_idle() {
            eprintln!("{err}");
        }

        unsafe {
            self.device.destroy_device(None);
        }
    }
}
