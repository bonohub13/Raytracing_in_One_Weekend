// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{
    RtError, RtResult,
    device::Device,
    params,
    renderer::{
        buffer::{DescriptorSet, GraphicsBuffer},
        path_tracer,
        swapchain::Swapchain,
    },
};
use ash::vk;
use std::sync::Arc;

pub struct CommandDescriptor {
    pub device: Arc<Device>,
}

pub struct CommandRecordDescriptor<'desc> {
    pub swapchain: &'desc Swapchain,
    pub descriptor_set: &'desc DescriptorSet,
    pub graphics_buffer: &'desc GraphicsBuffer,

    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: path_tracer::Pipeline,
    pub image_index: usize,
    pub current_frame: usize,
}

pub struct Command {
    device: Arc<Device>,
    pool: vk::CommandPool,
    buffers: Vec<vk::CommandBuffer>,
}

impl Command {
    const SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    };

    pub fn new(desc: &CommandDescriptor) -> RtResult<Self> {
        let pool = Self::create_command_pool(desc)?;
        let buffers: Vec<vk::CommandBuffer> = (0..params::MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_command_buffer(pool, desc))
            .collect::<RtResult<_>>()?;

        Ok(Self {
            device: desc.device.clone(),
            pool,
            buffers,
        })
    }

    #[inline]
    pub const fn buffers(&self) -> &[vk::CommandBuffer] {
        self.buffers.as_slice()
    }

    pub fn reset_command_buffer(&self, current_frame: usize) -> RtResult<()> {
        if let Err(err) = unsafe {
            self.device.raw().reset_command_buffer(
                self.buffers[current_frame],
                vk::CommandBufferResetFlags::empty(),
            )
        } {
            Err(RtError::ResetCommandBuffer(err.into()))
        } else {
            Ok(())
        }
    }

    pub fn begin_command_buffer(&self, current_frame: usize) -> RtResult<()> {
        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            self.device
                .raw()
                .begin_command_buffer(self.buffers[current_frame], &begin_info)
        }
        .map_err(|err| RtError::BeginCommandBuffer(err.into()))
    }

    pub fn end_command_buffer(&self, current_frame: usize) -> RtResult<()> {
        unsafe {
            self.device
                .raw()
                .end_command_buffer(self.buffers[current_frame])
        }
        .map_err(|err| RtError::EndCommandBuffer(err.into()))
    }

    pub fn bind_pipeline(&self, pipeline: path_tracer::Pipeline, current_frame: usize) {
        let (pipeline_bind_point, pipeline) = match pipeline {
            path_tracer::Pipeline::Graphics(pipeline) => {
                (vk::PipelineBindPoint::GRAPHICS, pipeline)
            }
        };

        unsafe {
            self.device.raw().cmd_bind_pipeline(
                self.buffers[current_frame],
                pipeline_bind_point,
                pipeline,
            )
        }
    }

    fn create_command_pool(desc: &CommandDescriptor) -> RtResult<vk::CommandPool> {
        let queue_family_indices = desc.device.queue_families();
        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_indices.graphics_family);

        unsafe { desc.device.raw().create_command_pool(&create_info, None) }
            .map_err(|err| RtError::CreateCommandPool(err.into()))
    }

    fn create_command_buffer(
        command_pool: vk::CommandPool,
        desc: &CommandDescriptor,
    ) -> RtResult<vk::CommandBuffer> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        match unsafe { desc.device.raw().allocate_command_buffers(&alloc_info) } {
            Ok(command_buffers) => {
                if let Some(command_buffer) = command_buffers.first() {
                    Ok(*command_buffer)
                } else {
                    Err(RtError::AllocateCommandBuffers(None))
                }
            }
            Err(err) => Err(RtError::AllocateCommandBuffers(Some(err.into()))),
        }
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        unsafe {
            self.device.raw().destroy_command_pool(self.pool, None);
        }
    }
}
