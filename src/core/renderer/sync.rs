// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::core::{
    RtError, RtResult,
    device::Device,
    params::MAX_FRAMES_IN_FLIGHT,
    renderer::{command::Command, swapchain::Swapchain},
};
use ash::vk;
use std::sync::Arc;

pub struct SyncObjectDescriptor {
    pub device: Arc<Device>,
}

#[derive(Clone)]
pub struct SyncObject {
    device: Arc<Device>,
    image_available_semaphores: Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
}

impl SyncObject {
    pub fn new(desc: &SyncObjectDescriptor) -> RtResult<Self> {
        let image_available_semaphores: Vec<vk::Semaphore> = (0..MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_semaphore(desc))
            .collect::<RtResult<_>>()?;
        let render_finished_semaphores: Vec<vk::Semaphore> = (0..MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_semaphore(desc))
            .collect::<RtResult<_>>()?;
        let in_flight_fences: Vec<vk::Fence> = (0..2)
            .map(|_| Self::create_fence(desc))
            .collect::<RtResult<_>>()?;

        Ok(Self {
            device: desc.device.clone(),
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
        })
    }

    #[inline]
    pub const fn image_available_semaphores(&self) -> &[vk::Semaphore] {
        self.image_available_semaphores.as_slice()
    }

    #[inline]
    pub const fn render_finished_semaphores(&self) -> &[vk::Semaphore] {
        self.render_finished_semaphores.as_slice()
    }

    #[inline]
    pub const fn in_flight_fences(&self) -> &[vk::Fence] {
        self.in_flight_fences.as_slice()
    }

    pub fn wait_for_fences(&self, current_frame: usize) -> RtResult<()> {
        unsafe {
            self.device.raw().wait_for_fences(
                std::slice::from_ref(&self.in_flight_fences[current_frame]),
                true,
                u64::MAX,
            )
        }
        .map_err(|err| RtError::WaitForFences(err.into()))
    }

    pub fn reset_fences(&self, current_frame: usize) -> RtResult<()> {
        unsafe {
            self.device
                .raw()
                .reset_fences(std::slice::from_ref(&self.in_flight_fences[current_frame]))
        }
        .map_err(|err| RtError::ResetFences(err.into()))
    }

    pub fn graphics_queue_submit(&self, command: &Command, current_frame: usize) -> RtResult<()> {
        let wait_info = vk::SemaphoreSubmitInfo::default()
            .semaphore(self.image_available_semaphores[current_frame])
            .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .device_index(0)
            .value(0);
        let cmd_info = vk::CommandBufferSubmitInfo::default()
            .command_buffer(command.buffers()[current_frame])
            .device_mask(0);
        let signal_info = vk::SemaphoreSubmitInfo::default()
            .semaphore(self.render_finished_semaphores[current_frame])
            .stage_mask(vk::PipelineStageFlags2::ALL_GRAPHICS)
            .device_index(0)
            .value(0);
        let submit_info = vk::SubmitInfo2::default()
            .wait_semaphore_infos(std::slice::from_ref(&wait_info))
            .command_buffer_infos(std::slice::from_ref(&cmd_info))
            .signal_semaphore_infos(std::slice::from_ref(&signal_info));

        unsafe {
            self.device.raw().queue_submit2(
                self.device.graphics_queue(),
                std::slice::from_ref(&submit_info),
                self.in_flight_fences[current_frame],
            )
        }
        .map_err(|err| RtError::SubmitQueue(err.into()))
    }

    pub fn present_queue(
        &self,
        swapchain: &Swapchain,
        image_index: u32,
        current_frame: usize,
    ) -> RtResult<bool> {
        let swapchains = [swapchain.raw()];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(
                &self.render_finished_semaphores[current_frame],
            ))
            .swapchains(&swapchains)
            .image_indices(std::slice::from_ref(&image_index));

        unsafe {
            swapchain
                .loader()
                .queue_present(self.device.present_queue(), &present_info)
        }
        .map_err(|err| RtError::QueuePresent(err.into()))
    }

    fn create_semaphore(desc: &SyncObjectDescriptor) -> RtResult<vk::Semaphore> {
        let create_info = vk::SemaphoreCreateInfo::default();

        unsafe { desc.device.raw().create_semaphore(&create_info, None) }
            .map_err(|err| RtError::CreateSemaphore(err.into()))
    }

    fn create_fence(desc: &SyncObjectDescriptor) -> RtResult<vk::Fence> {
        let create_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        unsafe { desc.device.raw().create_fence(&create_info, None) }
            .map_err(|err| RtError::CreateFence(err.into()))
    }
}

impl Drop for SyncObject {
    fn drop(&mut self) {
        let device = self.device.raw();

        self.image_available_semaphores
            .iter()
            .for_each(|semaphore| unsafe { device.destroy_semaphore(*semaphore, None) });
        self.render_finished_semaphores
            .iter()
            .for_each(|semaphore| unsafe { device.destroy_semaphore(*semaphore, None) });
        self.in_flight_fences
            .iter()
            .for_each(|fence| unsafe { device.destroy_fence(*fence, None) });
    }
}
