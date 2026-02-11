use crate::core::{
    self,
    error::{RtError, RtResult},
    surface,
};
use ash::vk;

pub struct Command {
    pool: vk::CommandPool,
    compute_buffers: Vec<vk::CommandBuffer>,
    buffers: Vec<vk::CommandBuffer>,
}

impl Command {
    pub fn new(
        instance: &ash::Instance,
        surface: &surface::Surface,
        physical_device: &vk::PhysicalDevice,
        device: &ash::Device,
    ) -> RtResult<Self> {
        let pool = Self::create_command_pool(instance, surface, physical_device, device)?;
        let compute_buffers = Self::create_command_buffers(device, &pool)?;
        let buffers = Self::create_command_buffers(device, &pool)?;

        Ok(Self {
            pool,
            compute_buffers,
            buffers,
        })
    }

    #[inline]
    pub const fn pool(&self) -> &vk::CommandPool {
        &self.pool
    }

    #[inline]
    pub fn compute_buffer(&self, current_frame: usize) -> &vk::CommandBuffer {
        &self.compute_buffers[current_frame]
    }

    #[inline]
    pub fn buffer(&self, current_frame: usize) -> &vk::CommandBuffer {
        &self.buffers[current_frame]
    }

    #[inline]
    pub fn reset_compute_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.reset_command_buffer(device, self.compute_buffers[current_frame])
    }

    #[inline]
    pub fn begin_compute_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.begin_command_buffer(device, self.compute_buffers[current_frame])
    }

    #[inline]
    pub fn end_compute_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.end_command_buffer(device, self.compute_buffers[current_frame])
    }

    #[inline]
    pub fn reset_render_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.reset_command_buffer(device, self.buffers[current_frame])
    }

    #[inline]
    pub fn begin_render_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.begin_command_buffer(device, self.buffers[current_frame])
    }

    #[inline]
    pub fn end_render_command_buffer(
        &self,
        device: &ash::Device,
        current_frame: usize,
    ) -> RtResult<()> {
        self.end_command_buffer(device, self.buffers[current_frame])
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        unsafe {
            device.destroy_command_pool(self.pool, None);
        }
    }

    fn reset_command_buffer(
        &self,
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
    ) -> RtResult<()> {
        if let Err(err) = unsafe {
            device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
        } {
            Err(RtError::ResetCommandBuffer(err.into()))
        } else {
            Ok(())
        }
    }

    fn begin_command_buffer(
        &self,
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
    ) -> RtResult<()> {
        let begin_info = vk::CommandBufferBeginInfo::default();

        match unsafe { device.begin_command_buffer(command_buffer, &begin_info) } {
            Ok(_) => Ok(()),
            Err(err) => Err(RtError::BeginCommandBuffer(err.into())),
        }
    }

    fn end_command_buffer(
        &self,
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
    ) -> RtResult<()> {
        match unsafe { device.end_command_buffer(command_buffer) } {
            Ok(_) => Ok(()),
            Err(err) => Err(RtError::EndCommandBuffer(err.into())),
        }
    }

    fn create_command_pool(
        instance: &ash::Instance,
        surface: &surface::Surface,
        physical_device: &vk::PhysicalDevice,
        device: &ash::Device,
    ) -> RtResult<vk::CommandPool> {
        let queue_family_indices =
            core::QueueFamilyIndices::find_queue_families(instance, surface, physical_device);

        // After physical device is created, queue family index checks are unnecessary
        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_indices.graphics_family.unwrap());

        match unsafe { device.create_command_pool(&create_info, None) } {
            Ok(command_pool) => Ok(command_pool),
            Err(err) => Err(RtError::CreateCommandPool(err.into())),
        }
    }

    fn create_command_buffers(
        device: &ash::Device,
        command_pool: &vk::CommandPool,
    ) -> RtResult<Vec<vk::CommandBuffer>> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(*command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(core::MAX_FRAMES_IN_FLIGHT as u32);

        match unsafe { device.allocate_command_buffers(&alloc_info) } {
            Ok(command_buffers) => {
                if command_buffers.len() == core::MAX_FRAMES_IN_FLIGHT {
                    return Ok(command_buffers);
                }
            }
            Err(err) => {
                return Err(RtError::AllocateCommandBuffer(Some(err.into())));
            }
        }

        Err(RtError::AllocateCommandBuffer(None))
    }
}
