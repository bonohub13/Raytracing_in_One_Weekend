use crate::core::{
    self,
    error::{RtError, RtResult},
};
use ash::vk;

pub struct Sync {
    compute_finished_semaphores: [vk::Semaphore; core::MAX_FRAMES_IN_FLIGHT],
    image_available_semaphores: [vk::Semaphore; core::MAX_FRAMES_IN_FLIGHT],
    render_finished_semaphores: [vk::Semaphore; core::MAX_FRAMES_IN_FLIGHT],
    compute_in_flight_fences: [vk::Fence; core::MAX_FRAMES_IN_FLIGHT],
    in_flight_fences: [vk::Fence; core::MAX_FRAMES_IN_FLIGHT],
}

impl Sync {
    pub fn new(device: &ash::Device) -> RtResult<Self> {
        let compute_finished_semaphores = Self::create_semaphores(device)?;
        let image_available_semaphores = Self::create_semaphores(device)?;
        let render_finished_semaphores = Self::create_semaphores(device)?;
        let compute_in_flight_fences = Self::create_fences(device)?;
        let in_flight_fences = Self::create_fences(device)?;

        Ok(Self {
            compute_finished_semaphores,
            image_available_semaphores,
            render_finished_semaphores,
            compute_in_flight_fences,
            in_flight_fences,
        })
    }

    #[inline]
    pub const fn compute_finished_semaphore(&self, current_frame: usize) -> &vk::Semaphore {
        &self.compute_finished_semaphores[current_frame]
    }

    #[inline]
    pub const fn image_available_semaphore(&self, current_frame: usize) -> &vk::Semaphore {
        &self.image_available_semaphores[current_frame]
    }

    #[inline]
    pub const fn render_finished_semaphore(&self, current_frame: usize) -> &vk::Semaphore {
        &self.render_finished_semaphores[current_frame]
    }

    #[inline]
    pub const fn compute_in_flight_fence(&self, current_frame: usize) -> &vk::Fence {
        &self.compute_in_flight_fences[current_frame]
    }

    #[inline]
    pub const fn in_flight_fence(&self, current_frame: usize) -> &vk::Fence {
        &self.in_flight_fences[current_frame]
    }

    #[inline]
    pub fn wait_for_compute_fence(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.wait_for_fences(device, &self.compute_in_flight_fences[current_frame])
    }

    #[inline]
    pub fn reset_compute_fence(&self, device: &ash::Device, current_frame: usize) -> RtResult<()> {
        self.reset_fences(device, &self.compute_in_flight_fences[current_frame])
    }

    #[inline]
    pub fn wait_for_render_fence(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.wait_for_fences(device, &self.in_flight_fences[current_frame])
    }

    #[inline]
    pub fn reset_render_fence(&self, device: &ash::Device, current_frame: usize) -> RtResult<()> {
        self.reset_fences(device, &self.in_flight_fences[current_frame])
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        self.compute_finished_semaphores
            .iter_mut()
            .for_each(|semaphore| unsafe { device.destroy_semaphore(*semaphore, None) });
        self.image_available_semaphores
            .iter_mut()
            .for_each(|semaphore| unsafe { device.destroy_semaphore(*semaphore, None) });
        self.render_finished_semaphores
            .iter_mut()
            .for_each(|semaphore| unsafe { device.destroy_semaphore(*semaphore, None) });
        self.compute_in_flight_fences
            .iter_mut()
            .for_each(|fence| unsafe { device.destroy_fence(*fence, None) });
        self.in_flight_fences
            .iter_mut()
            .for_each(|fence| unsafe { device.destroy_fence(*fence, None) });
    }

    fn wait_for_fences(&self, device: &ash::Device, fence: &vk::Fence) -> RtResult<()> {
        if let Err(err) =
            unsafe { device.wait_for_fences(std::slice::from_ref(fence), true, u64::MAX) }
        {
            Err(RtError::WaitForFences(err.into()))
        } else {
            Ok(())
        }
    }

    fn reset_fences(&self, device: &ash::Device, fence: &vk::Fence) -> RtResult<()> {
        if let Err(err) = unsafe { device.reset_fences(std::slice::from_ref(fence)) } {
            Err(RtError::ResetFences(err.into()))
        } else {
            Ok(())
        }
    }

    fn create_semaphores(
        device: &ash::Device,
    ) -> RtResult<[vk::Semaphore; core::MAX_FRAMES_IN_FLIGHT]> {
        let mut semaphores = [vk::Semaphore::null(); core::MAX_FRAMES_IN_FLIGHT];
        let create_info = vk::SemaphoreCreateInfo::default();

        for semaphore_mut in semaphores.iter_mut() {
            match unsafe { device.create_semaphore(&create_info, None) } {
                Ok(semaphore) => {
                    *semaphore_mut = semaphore;
                }
                Err(err) => return Err(RtError::CreateSemaphore(err.into())),
            }
        }

        Ok(semaphores)
    }

    fn create_fences(device: &ash::Device) -> RtResult<[vk::Fence; core::MAX_FRAMES_IN_FLIGHT]> {
        let mut fences = [vk::Fence::null(); core::MAX_FRAMES_IN_FLIGHT];
        let create_info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);

        for fence_mut in fences.iter_mut() {
            match unsafe { device.create_fence(&create_info, None) } {
                Ok(fence) => *fence_mut = fence,
                Err(err) => return Err(RtError::CreateFence(err.into())),
            }
        }

        Ok(fences)
    }
}
