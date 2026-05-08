// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Device, RtErr, RtError, Swapchain, VkState};
use ash::vk;
use std::sync::Arc;

pub struct Encoder {
    command_buffers: Vec<vk::CommandBuffer>,
    pool: vk::CommandPool,
    device: Arc<Device>,
}

impl Encoder {
    pub fn new(state: &VkState, frames_in_flight: u32) -> RtErr<Self> {
        let pool = Self::create_command_pool(state)?;
        let command_buffers =
            Self::allocate_command_buffers(state.device.clone(), pool, frames_in_flight)?;

        Ok(Self {
            pool,
            command_buffers,
            device: state.device.clone(),
        })
    }

    #[inline]
    pub(crate) fn command_buffers(&self) -> &[vk::CommandBuffer] {
        &self.command_buffers
    }

    pub fn record_frame<CmdFn>(&self, current_frame: usize, commands: CmdFn) -> RtErr<()>
    where
        CmdFn: FnOnce(vk::CommandBuffer) -> RtErr<()>,
    {
        let device = self.device.device();
        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            self.device.device().reset_command_buffer(
                self.command_buffers[current_frame],
                vk::CommandBufferResetFlags::empty(),
            )
        }
        .map_err(|err| RtError::ResetCommandBuffer(err.into()))?;
        unsafe { device.begin_command_buffer(self.command_buffers[current_frame], &begin_info) }
            .map_err(|err| RtError::BeginCommandBuffer(err.into()))?;

        commands(self.command_buffers[current_frame])?;

        unsafe { device.end_command_buffer(self.command_buffers[current_frame]) }
            .map_err(|err| RtError::EndCommandBuffer(err.into()))
    }

    pub fn submit_single_command_buffer<CmdFn>(&self, commands: CmdFn) -> RtErr<()>
    where
        CmdFn: FnOnce(vk::CommandBuffer),
    {
        let device = self.device.device();
        let queue = self.device.graphics_queue();
        let command_buffer = Self::allocate_command_buffers(self.device.clone(), self.pool, 1)?[0];
        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        let command_buffer_info =
            vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer);
        let submit_info = vk::SubmitInfo2::default()
            .command_buffer_infos(std::slice::from_ref(&command_buffer_info));

        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }
            .map_err(|err| RtError::BeginCommandBuffer(err.into()))?;

        commands(command_buffer);

        unsafe { device.end_command_buffer(command_buffer) }
            .map_err(|err| RtError::EndCommandBuffer(err.into()))?;
        unsafe {
            device.queue_submit2(queue, std::slice::from_ref(&submit_info), vk::Fence::null())
        }
        .map_err(|err| RtError::QueueSubmit(err.into()))?;
        unsafe { device.queue_wait_idle(queue) }
            .map_err(|err| RtError::QueueWaitIdle(err.into()))?;
        unsafe {
            device.free_command_buffers(self.pool, std::slice::from_ref(&command_buffer));
        }

        Ok(())
    }

    pub fn render<Fn>(
        &self,
        swapchain: &Swapchain,
        image_index: usize,
        current_frame: usize,
        render_frame: Fn,
    ) where
        Fn: FnOnce(vk::CommandBuffer),
    {
        static OFFSET: vk::Offset2D = vk::Offset2D { x: 0, y: 0 };

        let device = self.device.device();
        let command_buffer = self.command_buffers[current_frame];
        let color_attachment = swapchain.color_attachment(image_index);
        let depth_attachment = swapchain.depth_attachment();
        let rendering_info = vk::RenderingInfo::default()
            .render_area(vk::Rect2D {
                offset: OFFSET,
                extent: *swapchain.extent(),
            })
            .layer_count(1)
            .color_attachments(std::slice::from_ref(&color_attachment))
            .depth_attachment(&depth_attachment);

        unsafe {
            device.cmd_begin_rendering(command_buffer, &rendering_info);
        }

        // Draw calls
        render_frame(command_buffer);

        unsafe {
            device.cmd_end_rendering(command_buffer);
        }
    }

    fn create_command_pool(state: &VkState) -> RtErr<vk::CommandPool> {
        let queue_family_indices = state
            .surface
            .find_queue_families(&state.instance, state.device.physical_device())?;
        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(
                queue_family_indices
                    .graphics_family
                    .expect("Queue family index for graphics not found!"),
            );

        unsafe {
            state
                .device
                .device()
                .create_command_pool(&create_info, None)
        }
        .map_err(|err| RtError::CreateCommandPool(err.into()))
    }

    fn allocate_command_buffers(
        device: Arc<Device>,
        pool: vk::CommandPool,
        frames_in_flight: u32,
    ) -> RtErr<Vec<vk::CommandBuffer>> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(frames_in_flight);

        unsafe { device.device().allocate_command_buffers(&alloc_info) }
            .map_err(|err| RtError::AllocateCommandBuffers(err.into()))
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_command_pool(self.pool, None);
        }
    }
}
