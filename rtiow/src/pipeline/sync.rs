// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Device, Encoder, RtErr, RtError, VkState};
use ash::vk;
use std::sync::Arc;

pub struct SyncObject {
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    device: Arc<Device>,
}

impl SyncObject {
    pub fn new(state: &VkState, frames_in_flight: u32) -> RtErr<Self> {
        let image_available_semaphores: Vec<_> = (0..frames_in_flight)
            .map(|_| Self::create_semaphore(state))
            .collect::<RtErr<_>>()?;
        let render_finished_semaphores: Vec<_> = (0..frames_in_flight)
            .map(|_| Self::create_semaphore(state))
            .collect::<RtErr<_>>()?;
        let in_flight_fences: Vec<_> = (0..frames_in_flight)
            .map(|_| Self::create_fence(state))
            .collect::<RtErr<_>>()?;

        Ok(Self {
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            device: state.device.clone(),
        })
    }

    #[inline]
    pub fn image_available_semaphores(&self) -> &[vk::Semaphore] {
        &self.image_available_semaphores
    }

    #[inline]
    pub fn render_finished_semaphores(&self) -> &[vk::Semaphore] {
        &self.render_finished_semaphores
    }

    #[inline]
    pub fn in_flight_fences(&self) -> &[vk::Fence] {
        &self.in_flight_fences
    }

    pub fn wait_for_fences(&self, current_frame: usize) -> RtErr<()> {
        unsafe {
            self.device.device().wait_for_fences(
                std::slice::from_ref(&self.in_flight_fences[current_frame]),
                true,
                u64::MAX,
            )
        }
        .map_err(|err| RtError::WaitForFences(err.into()))
    }

    pub fn reset_fences(&self, current_frame: usize) -> RtErr<()> {
        unsafe {
            self.device
                .device()
                .reset_fences(std::slice::from_ref(&self.in_flight_fences[current_frame]))
        }
        .map_err(|err| RtError::ResetFences(err.into()))
    }

    pub fn submit_graphics_queue(&self, encoder: &Encoder, current_frame: usize) -> RtErr<()> {
        let wait_semaphore_info = vk::SemaphoreSubmitInfo::default()
            .semaphore(self.image_available_semaphores[current_frame])
            .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .device_index(0)
            .value(0);
        let command_buffer_info = vk::CommandBufferSubmitInfo::default()
            .command_buffer(encoder.command_buffers()[current_frame])
            .device_mask(0);
        let signal_semaphore_info = vk::SemaphoreSubmitInfo::default()
            .semaphore(self.render_finished_semaphores[current_frame])
            .stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS)
            .device_index(0)
            .value(0);
        let submit_info = vk::SubmitInfo2::default()
            .wait_semaphore_infos(std::slice::from_ref(&wait_semaphore_info))
            .command_buffer_infos(std::slice::from_ref(&command_buffer_info))
            .signal_semaphore_infos(std::slice::from_ref(&signal_semaphore_info));

        unsafe {
            self.device.device().queue_submit2(
                self.device.graphics_queue(),
                std::slice::from_ref(&submit_info),
                self.in_flight_fences[current_frame],
            )
        }
        .map_err(|err| RtError::QueueSubmit(err.into()))
    }

    fn create_semaphore(state: &VkState) -> RtErr<vk::Semaphore> {
        let create_info = vk::SemaphoreCreateInfo::default();

        unsafe { state.device.device().create_semaphore(&create_info, None) }
            .map_err(|err| RtError::CreateSemaphore(err.into()))
    }

    fn create_fence(state: &VkState) -> RtErr<vk::Fence> {
        let create_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        unsafe { state.device.device().create_fence(&create_info, None) }
            .map_err(|err| RtError::CreateFence(err.into()))
    }
}

impl Drop for SyncObject {
    fn drop(&mut self) {
        let device = self.device.device();

        self.image_available_semaphores
            .iter()
            .copied()
            .for_each(|semaphore| unsafe {
                device.destroy_semaphore(semaphore, None);
            });
        self.render_finished_semaphores
            .iter()
            .copied()
            .for_each(|semaphore| unsafe {
                device.destroy_semaphore(semaphore, None);
            });
        self.in_flight_fences
            .iter()
            .copied()
            .for_each(|fence| unsafe {
                device.destroy_fence(fence, None);
            });
    }
}
