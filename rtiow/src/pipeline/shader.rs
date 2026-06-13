// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Allocator, Buffer, BufferType, Device, RtErr, RtError, VkState, util::align_up};
use ash::vk;
use std::{
    fs,
    io::Read,
    path::Path,
    sync::{Arc, Mutex},
};

pub struct ShaderModule {
    shader: vk::ShaderModule,
    device: Arc<Device>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SbtLayout {
    pub(crate) handle_size: u32,
    pub(crate) rgen_offset: vk::DeviceSize,
    pub(crate) miss_offset: vk::DeviceSize,
    pub(crate) hit_offset: vk::DeviceSize,
    pub(crate) hit_stride: vk::DeviceSize,
    pub(crate) total_size: vk::DeviceSize,
}

pub(crate) struct ShaderBindingTable {
    _buffer: Buffer<u8>,
    raygen: vk::StridedDeviceAddressRegionKHR,
    miss: vk::StridedDeviceAddressRegionKHR,
    hit: vk::StridedDeviceAddressRegionKHR,
    callable: vk::StridedDeviceAddressRegionKHR,
}

impl ShaderModule {
    pub fn new(device: Arc<Device>, path: &Path) -> RtErr<Self> {
        let code = read_shader(path)?;
        let shader = Self::create_shader_module(device.clone(), &code)?;

        Ok(Self { device, shader })
    }

    #[inline]
    pub(crate) fn shader(&self) -> vk::ShaderModule {
        self.shader
    }

    fn create_shader_module(device: Arc<Device>, code: &[u32]) -> RtErr<vk::ShaderModule> {
        let create_info = vk::ShaderModuleCreateInfo::default().code(code);

        unsafe { device.device().create_shader_module(&create_info, None) }
            .map_err(|err| RtError::CreateShaderModule(err.into()))
    }
}

impl ShaderBindingTable {
    pub(crate) const GROUP_COUNT: usize = 4;

    pub(crate) fn new(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        layout: &SbtLayout,
        handle_data: &[u8],
    ) -> RtErr<Self> {
        let buffer = Buffer::<u8>::new(state, allocator, BufferType::Sbt(layout.total_size))?;
        let handle_size = layout.handle_size as vk::DeviceSize;
        let hit_stride = layout.hit_stride as vk::DeviceSize;
        let raygen = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(buffer.gpu_address()[0] + layout.rgen_offset)
            .stride(hit_stride)
            .size(handle_size);
        let miss = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(buffer.gpu_address()[0] + layout.miss_offset)
            .stride(hit_stride)
            .size(handle_size);
        let hit = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(buffer.gpu_address()[0] + layout.hit_offset)
            .stride(hit_stride)
            .size(handle_size * 2);
        let callable = vk::StridedDeviceAddressRegionKHR::default();

        if let Some(alloc) = buffer.allocation()
            && let Some(alloc_ptr) = alloc.mapped_ptr()
        {
            let handle_size = layout.handle_size as usize;
            let raygen_offset = layout.rgen_offset as usize;
            let miss_offset = layout.miss_offset as usize;
            let hit_offset = layout.hit_offset as usize;
            let hit_stride = layout.hit_stride as usize;
            let dst_ptr = alloc_ptr.as_ptr() as *mut u8;
            let src_ptr = handle_data.as_ptr();

            unsafe {
                std::ptr::copy_nonoverlapping(src_ptr, dst_ptr.add(raygen_offset), handle_size);
                std::ptr::copy_nonoverlapping(
                    src_ptr.add(handle_size),
                    dst_ptr.add(miss_offset),
                    handle_size,
                );
                std::ptr::copy_nonoverlapping(
                    src_ptr.add(2 * handle_size),
                    dst_ptr.add(hit_offset),
                    handle_size,
                );
                std::ptr::copy_nonoverlapping(
                    src_ptr.add(3 * handle_size),
                    dst_ptr.add(hit_offset + hit_stride),
                    handle_size,
                );
            }
        }

        Ok(Self {
            _buffer: buffer,
            raygen,
            miss,
            hit,
            callable,
        })
    }

    #[inline]
    pub(crate) fn raygen_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.raygen
    }

    #[inline]
    pub(crate) fn miss_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.miss
    }

    #[inline]
    pub(crate) fn hit_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.hit
    }

    #[inline]
    pub(crate) fn callable_region(&self) -> &vk::StridedDeviceAddressRegionKHR {
        &self.callable
    }

    pub(crate) fn calculate_sbt_layout(state: &VkState) -> SbtLayout {
        let properties = {
            let mut rt_properties = vk::PhysicalDeviceRayTracingPipelinePropertiesKHR::default();
            let mut properties =
                vk::PhysicalDeviceProperties2::default().push_next(&mut rt_properties);

            unsafe {
                state.instance.instance().get_physical_device_properties2(
                    state.device.physical_device(),
                    &mut properties,
                )
            };

            rt_properties
        };
        let handle_size = properties.shader_group_handle_size;
        let base_alignment = properties.shader_group_base_alignment;
        let handle_alignment = properties.shader_group_handle_alignment;
        let hit_stride = align_up!(handle_size as u64, handle_alignment as u64);
        let raygen_section_size = hit_stride;
        let miss_section_size = hit_stride;
        let hit_section_size = hit_stride * 2;
        let rgen_offset = 0;
        let miss_offset = align_up!(rgen_offset + raygen_section_size, base_alignment as u64);
        let hit_offset = align_up!(miss_offset + miss_section_size, base_alignment as u64);
        let total_size = align_up!(hit_offset + hit_section_size, base_alignment as u64);

        SbtLayout {
            handle_size,
            rgen_offset,
            miss_offset,
            hit_offset,
            hit_stride,
            total_size,
        }
    }
}

fn read_shader(path: &Path) -> RtErr<Box<[u32]>> {
    const BYTE_ALIGNMENT: usize = 4;
    const BYTE_ALIGNMENT_MASK: usize = 0b11;

    let mut reader = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|err| RtError::ReadFile(err.into()))?;
    let file_size = reader
        .metadata()
        .map_err(|err| RtError::ReadFile(err.into()))?
        .len() as usize;
    let padding_size = BYTE_ALIGNMENT - (file_size & BYTE_ALIGNMENT_MASK);
    let mut buffer: Vec<u8> = Vec::with_capacity(file_size + padding_size);

    reader
        .read_to_end(&mut buffer)
        .map_err(|err| RtError::ReadFile(err.into()))?;
    if !buffer.len().is_multiple_of(BYTE_ALIGNMENT) {
        buffer.extend(std::iter::repeat_n(0, padding_size))
    }

    Ok(Box::from(bytemuck::cast_slice(&buffer)))
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.device
                .device()
                .destroy_shader_module(self.shader, None)
        }
    }
}
