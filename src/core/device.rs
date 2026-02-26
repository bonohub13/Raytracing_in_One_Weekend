// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{RtError, RtResult, instance::Instance, surface::Surface};
use ash::{khr::swapchain, vk};
use gpu_allocator::vulkan::{self, Allocator};
use std::{
    ffi::CStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Copy, Default)]
struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct QueueFamilies {
    pub graphics_family: u32,
    pub present_family: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SwapchainSupportDetail {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

pub struct DeviceDescriptor {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface>,
}

pub struct Device {
    instance: Arc<Instance>,
    raw: ash::Device,
    physical_device: vk::PhysicalDevice,
    allocator: Option<Mutex<Allocator>>,
    queue_families: QueueFamilies,
    swapchain_support: SwapchainSupportDetail,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
}

impl QueueFamilyIndices {
    fn find_queue_families(
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<Self> {
        let mut indices = Self::default();
        let queue_family_properties: Vec<vk::QueueFamilyProperties> = {
            let count = unsafe {
                instance
                    .raw()
                    .get_physical_device_queue_family_properties2_len(physical_device)
            };
            let mut properties = vec![vk::QueueFamilyProperties2::default(); count];

            unsafe {
                instance
                    .raw()
                    .get_physical_device_queue_family_properties2(physical_device, &mut properties)
            };

            properties
                .iter()
                .map(|property| property.queue_family_properties)
                .collect()
        };

        for (i, queue_family) in queue_family_properties.iter().enumerate() {
            let current_index = i as u32;
            let present_support = unsafe {
                instance
                    .surface_loader()
                    .get_physical_device_surface_support(
                        physical_device,
                        current_index,
                        surface.raw(),
                    )
            }
            .map_err(|err| RtError::GetPhysicalDeviceSurfaceSupport(err.into()))?;

            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indices.graphics_family = Some(current_index);
            }

            if present_support {
                indices.present_family = Some(current_index);
            }

            if indices.is_complete() {
                break;
            }
        }

        Ok(indices)
    }

    #[inline]
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

impl SwapchainSupportDetail {
    fn query_swapchain_support(
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<Self> {
        let capabilities = Self::get_physical_device_surface_capabilities(
            instance.clone(),
            surface.clone(),
            physical_device,
        )?;
        let formats = Self::get_physical_device_surface_formats(
            instance.clone(),
            surface.clone(),
            physical_device,
        )?;
        let present_modes =
            Self::get_physical_device_surface_present_modes(instance, surface, physical_device)?;

        Ok(Self {
            capabilities,
            formats,
            present_modes,
        })
    }

    pub fn choose_swap_surface_format(&self) -> vk::SurfaceFormatKHR {
        self.formats
            .iter()
            .copied()
            .find(|format| {
                (format.format == vk::Format::B8G8R8_SRGB)
                    && (format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
            })
            .unwrap_or(self.formats[0])
    }

    pub fn choose_swap_present_mode(&self) -> vk::PresentModeKHR {
        self.present_modes
            .iter()
            .copied()
            .find(|present_mode| *present_mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn choose_swap_extent(&self, size: winit::dpi::PhysicalSize<u32>) -> vk::Extent2D {
        if self.capabilities.current_extent.width != u32::MAX {
            self.capabilities.current_extent
        } else {
            vk::Extent2D {
                width: size.width.clamp(
                    self.capabilities.min_image_extent.width,
                    self.capabilities.max_image_extent.width,
                ),
                height: size.height.clamp(
                    self.capabilities.min_image_extent.height,
                    self.capabilities.max_image_extent.height,
                ),
            }
        }
    }

    fn get_physical_device_surface_capabilities(
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<vk::SurfaceCapabilitiesKHR> {
        unsafe {
            instance
                .surface_loader()
                .get_physical_device_surface_capabilities(physical_device, surface.raw())
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfaceCapabilities(err.into()))
    }

    fn get_physical_device_surface_formats(
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<Vec<vk::SurfaceFormatKHR>> {
        unsafe {
            instance
                .surface_loader()
                .get_physical_device_surface_formats(physical_device, surface.raw())
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfaceFormats(err.into()))
    }

    fn get_physical_device_surface_present_modes(
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<Vec<vk::PresentModeKHR>> {
        unsafe {
            instance
                .surface_loader()
                .get_physical_device_surface_present_modes(physical_device, surface.raw())
        }
        .map_err(|err| RtError::GetPhysicalDeviceSurfacePresentModes(err.into()))
    }
}

impl Device {
    const DEVICE_EXTENSIONS: [&CStr; 1] = [swapchain::NAME];

    pub fn new(desc: &DeviceDescriptor) -> RtResult<Self> {
        let physical_device = Self::choose_physical_device(desc)?;
        let queue_families = {
            let queue_families = QueueFamilyIndices::find_queue_families(
                desc.instance.clone(),
                desc.surface.clone(),
                physical_device,
            )?;

            QueueFamilies {
                graphics_family: queue_families.graphics_family.unwrap(),
                present_family: queue_families.present_family.unwrap(),
            }
        };
        let swapchain_support = SwapchainSupportDetail::query_swapchain_support(
            desc.instance.clone(),
            desc.surface.clone(),
            physical_device,
        )?;
        let (device, graphics_queue, present_queue) = Self::create_device(desc, physical_device)?;
        let allocator = Some(Mutex::new(Self::create_allocator(
            desc.instance.clone(),
            device.clone(),
            physical_device,
        )?));

        Ok(Self {
            instance: desc.instance.clone(),
            physical_device,
            raw: device,
            allocator,
            queue_families,
            swapchain_support,
            graphics_queue,
            present_queue,
        })
    }

    #[inline]
    pub const fn raw(&self) -> &ash::Device {
        &self.raw
    }

    /// If this is called AFTER Device has been dropped, it will panic since
    /// it is set to NONE.
    /// However, if Device has been dropped, other resources are also no longer
    /// available, making this a non-issue
    #[inline]
    pub const fn allocator(&self) -> &Mutex<Allocator> {
        self.allocator.as_ref().expect("Allocator doesn't exist")
    }

    #[inline]
    pub const fn queue_families(&self) -> QueueFamilies {
        self.queue_families
    }

    #[inline]
    pub const fn swapchain_support(&self) -> &SwapchainSupportDetail {
        &self.swapchain_support
    }

    #[inline]
    pub const fn graphics_queue(&self) -> vk::Queue {
        self.graphics_queue
    }

    #[inline]
    pub const fn present_queue(&self) -> vk::Queue {
        self.present_queue
    }

    pub fn device_wait_idle(&self) -> RtResult<()> {
        if let Err(err) = unsafe { self.raw.device_wait_idle() } {
            Err(RtError::DeviceWaitIdle(err.into()))
        } else {
            Ok(())
        }
    }

    fn choose_physical_device(desc: &DeviceDescriptor) -> RtResult<vk::PhysicalDevice> {
        desc.instance
            .enumerate_physical_devices()
            .map(|mut devices| {
                devices.sort_by_key(|device| {
                    Self::rate_device_suitability(desc.instance.raw(), device)
                });
                if let Some(device) = devices
                    .iter()
                    .find(|device| Self::is_suitable_device(desc, device).unwrap_or(false))
                {
                    Ok(*device)
                } else {
                    Err(RtError::NoSuitableDevice)
                }
            })?
    }

    fn create_device(
        desc: &DeviceDescriptor,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<(ash::Device, vk::Queue, vk::Queue)> {
        const QUEUE_PRIORITY: [f32; 1] = [1f32];

        let indices = QueueFamilyIndices::find_queue_families(
            desc.instance.clone(),
            desc.surface.clone(),
            physical_device,
        )?;
        let graphics_queue_family_index = indices.graphics_family.unwrap();
        let present_queue_family_index = indices.present_family.unwrap();
        let queue_indices = if graphics_queue_family_index == present_queue_family_index {
            vec![graphics_queue_family_index]
        } else {
            vec![graphics_queue_family_index, present_queue_family_index]
        };
        let queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = queue_indices
            .iter()
            .map(|queue_index| {
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(*queue_index)
                    .queue_priorities(&QUEUE_PRIORITY)
            })
            .collect();
        let mut shader_draw_parameters =
            vk::PhysicalDeviceShaderDrawParameterFeatures::default().shader_draw_parameters(true);
        let mut dynamic_rendering =
            vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);
        let mut syncrhonization2 =
            vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true);
        let mut buffer_device_address =
            vk::PhysicalDeviceBufferDeviceAddressFeatures::default().buffer_device_address(true);
        let extension_names: Vec<*const i8> = Self::DEVICE_EXTENSIONS
            .iter()
            .map(|extension| extension.as_ptr())
            .collect();
        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&extension_names)
            .push_next(&mut shader_draw_parameters)
            .push_next(&mut dynamic_rendering)
            .push_next(&mut syncrhonization2)
            .push_next(&mut buffer_device_address);
        let device = unsafe {
            desc.instance
                .raw()
                .create_device(physical_device, &device_create_info, None)
        }
        .map_err(|err| RtError::CreateDevice(err.into()))?;
        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family_index, 0) };
        let present_queue = if present_queue_family_index == graphics_queue_family_index {
            graphics_queue
        } else {
            unsafe { device.get_device_queue(present_queue_family_index, 0) }
        };

        Ok((device, graphics_queue, present_queue))
    }

    fn create_allocator(
        instance: Arc<Instance>,
        device: ash::Device,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<Allocator> {
        Allocator::new(&vulkan::AllocatorCreateDesc {
            instance: instance.raw().clone(),
            device,
            physical_device,
            debug_settings: Default::default(),
            buffer_device_address: true,
            allocation_sizes: Default::default(),
        })
        .map_err(|err| RtError::CreateAllocator(err.into()))
    }

    fn is_suitable_device(
        desc: &DeviceDescriptor,
        physical_device: &&vk::PhysicalDevice,
    ) -> RtResult<bool> {
        let indices = QueueFamilyIndices::find_queue_families(
            desc.instance.clone(),
            desc.surface.clone(),
            **physical_device,
        )?;
        let extensions_supported = Self::check_device_extension_support(desc, **physical_device)?;
        let features_supported = Self::check_device_feature_support(desc, **physical_device);
        let mut swapchain_adequate = false;

        if extensions_supported && features_supported {
            let swapchain_support = SwapchainSupportDetail::query_swapchain_support(
                desc.instance.clone(),
                desc.surface.clone(),
                **physical_device,
            )?;

            swapchain_adequate = (!swapchain_support.formats.is_empty())
                && (!swapchain_support.present_modes.is_empty());
        }

        Ok(indices.is_complete() && extensions_supported && swapchain_adequate)
    }

    fn check_device_extension_support(
        desc: &DeviceDescriptor,
        physical_device: vk::PhysicalDevice,
    ) -> RtResult<bool> {
        let properties = unsafe {
            desc.instance
                .raw()
                .enumerate_device_extension_properties(physical_device)
        }
        .map_err(|err| RtError::EnumerateDeviceExtensionProperties(err.into()))?;
        let available_extensions: Vec<&CStr> = properties
            .iter()
            .map(|extension| unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) })
            .collect();

        Ok(!Self::DEVICE_EXTENSIONS
            .iter()
            .any(|extension| !available_extensions.contains(extension)))
    }

    fn check_device_feature_support(
        desc: &DeviceDescriptor,
        physical_device: vk::PhysicalDevice,
    ) -> bool {
        let mut shader_draw_parameters = vk::PhysicalDeviceShaderDrawParametersFeatures::default();
        let mut dynamic_rendering = vk::PhysicalDeviceDynamicRenderingFeatures::default();
        let mut syncrhonization2 = vk::PhysicalDeviceSynchronization2Features::default();
        let mut buffer_device_address = vk::PhysicalDeviceBufferDeviceAddressFeatures::default();
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut shader_draw_parameters)
            .push_next(&mut dynamic_rendering)
            .push_next(&mut syncrhonization2)
            .push_next(&mut buffer_device_address);

        unsafe {
            desc.instance
                .raw()
                .get_physical_device_features2(physical_device, &mut features)
        };

        (shader_draw_parameters.shader_draw_parameters == vk::TRUE)
            && (dynamic_rendering.dynamic_rendering == vk::TRUE)
            && (syncrhonization2.synchronization2 == vk::TRUE)
            && (buffer_device_address.buffer_device_address == vk::TRUE)
    }

    fn rate_device_suitability(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> u32 {
        let properties = {
            let mut properties = vk::PhysicalDeviceProperties2::default();

            unsafe { instance.get_physical_device_properties2(*physical_device, &mut properties) };

            properties.properties
        };
        let features = {
            let mut features = vk::PhysicalDeviceFeatures2::default();

            unsafe { instance.get_physical_device_features2(*physical_device, &mut features) };

            features.features
        };
        let mut score = properties.limits.max_image_dimension2_d;

        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        if features.geometry_shader == 0 {
            0
        } else {
            score
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if let Some(allocator) = self.allocator.take() {
            drop(allocator);
        }
        unsafe { self.raw.destroy_device(None) };
    }
}
