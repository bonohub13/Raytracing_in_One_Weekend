use crate::core::{
    RtError, RtResult,
    device::Device,
    params::MAX_FRAMES_IN_FLIGHT,
    renderer::{command::Command, swapchain::Swapchain},
    util as rt_util,
};
use ash::vk;
use std::sync::{Arc, Mutex};

pub struct SyncObjectDescriptor {
    pub device: Arc<Device>,
    pub swapchain: Arc<Mutex<Swapchain>>,
    pub command: Arc<Command>,
}

#[derive(Clone)]
pub struct SyncObject {
    device: Arc<Device>,
    swapchain: Arc<Mutex<Swapchain>>,
    command: Arc<Command>,
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
            swapchain: desc.swapchain.clone(),
            command: desc.command.clone(),
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
        })
    }

    pub fn wait_for_fences(&self, current_frame: usize) -> RtResult<()> {
        if let Err(err) = unsafe {
            self.device.device().wait_for_fences(
                std::slice::from_ref(&self.in_flight_fences[current_frame]),
                true,
                u64::MAX,
            )
        } {
            Err(RtError::WaitForFences(err.into()))
        } else {
            Ok(())
        }
    }

    pub fn reset_fences(&self, current_frame: usize) -> RtResult<()> {
        if let Err(err) = unsafe {
            self.device
                .device()
                .reset_fences(std::slice::from_ref(&self.in_flight_fences[current_frame]))
        } {
            Err(RtError::ResetFences(err.into()))
        } else {
            Ok(())
        }
    }

    pub fn acquire_next_image(&mut self, current_frame: usize) -> RtResult<Option<(usize, bool)>> {
        let mut guard = rt_util::lock_mutex!(self.swapchain);
        let image_info = vk::AcquireNextImageInfoKHR::default()
            .swapchain(guard.swapchain())
            .timeout(u64::MAX)
            .semaphore(self.image_available_semaphores[current_frame])
            .fence(vk::Fence::null())
            .device_mask(1);
        let result = match unsafe { guard.loader().acquire_next_image2(&image_info) } {
            Ok((image_index, is_suboptimal)) => Ok(Some((image_index as usize, is_suboptimal))),
            Err(err) => match err {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    if let Err(err) = guard.recreate_swapchain() {
                        Err(err)
                    } else {
                        Ok(None)
                    }
                }
                _ => Err(RtError::AcquireNextImage(err.into())),
            },
        };

        drop(guard);

        result
    }

    pub fn graphics_queue_submit(&self, current_frame: usize) -> RtResult<()> {
        let wait_info = vk::SemaphoreSubmitInfo::default()
            .semaphore(self.image_available_semaphores[current_frame])
            .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .device_index(0)
            .value(0);
        let cmd_info = vk::CommandBufferSubmitInfo::default()
            .command_buffer(self.command.buffers()[current_frame])
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

        if let Err(err) = unsafe {
            self.device.device().queue_submit2(
                self.device.graphics_queue(),
                std::slice::from_ref(&submit_info),
                self.in_flight_fences[current_frame],
            )
        } {
            Err(RtError::SubmitQueue(err.into()))
        } else {
            Ok(())
        }
    }

    pub fn present_queue(&self, image_index: u32, current_frame: usize) -> RtResult<bool> {
        let guard = rt_util::lock_mutex!(self.swapchain);
        let swapchains = [guard.swapchain()];
        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(std::slice::from_ref(
                &self.render_finished_semaphores[current_frame],
            ))
            .swapchains(&swapchains)
            .image_indices(std::slice::from_ref(&image_index));
        let result = match unsafe {
            guard
                .loader()
                .queue_present(self.device.present_queue(), &present_info)
        } {
            Ok(queue_presented) => Ok(queue_presented),
            Err(err) => Err(RtError::QueuePresent(err.into())),
        };

        drop(guard);

        result
    }

    fn create_semaphore(desc: &SyncObjectDescriptor) -> RtResult<vk::Semaphore> {
        let create_info = vk::SemaphoreCreateInfo::default();

        match unsafe { desc.device.device().create_semaphore(&create_info, None) } {
            Ok(semaphore) => Ok(semaphore),
            Err(err) => Err(RtError::CreateSemaphore(err.into())),
        }
    }

    fn create_fence(desc: &SyncObjectDescriptor) -> RtResult<vk::Fence> {
        let create_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        match unsafe { desc.device.device().create_fence(&create_info, None) } {
            Ok(fence) => Ok(fence),
            Err(err) => Err(RtError::CreateFence(err.into())),
        }
    }
}

impl Drop for SyncObject {
    fn drop(&mut self) {
        let device = self.device.device();

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
