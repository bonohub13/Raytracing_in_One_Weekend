use crate::core::error::{RtError, RtResult};
use ash::vk;
use gpu_allocator::vulkan as vk_alloc;

pub struct Buffer {
    buffer: vk::Buffer,
    size: u64,
    allocation: Option<vk_alloc::Allocation>,
}

impl Buffer {
    pub fn new(buffer: vk::Buffer, size: u64, allocation: vk_alloc::Allocation) -> Self {
        Self {
            buffer,
            size,
            allocation: Some(allocation),
        }
    }

    pub fn write<T>(&mut self, data: T)
    where
        T: Clone + Copy + Sized,
    {
        if let Some(allocation) = self.allocation.as_mut() {
            unsafe {
                let mapped_ptr = allocation
                    .mapped_ptr()
                    .expect("Memory is not visible from host")
                    .as_ptr() as *mut T;

                mapped_ptr.write(data)
            }
        }
    }

    pub unsafe fn destroy(
        &mut self,
        allocator: &mut Option<vk_alloc::Allocator>,
        device: &ash::Device,
    ) -> RtResult<()> {
        if let Some(allocator) = allocator.as_mut() {
            if let Some(allocation) = self.allocation.take()
                && let Err(err) = allocator.free(allocation)
            {
                Err(RtError::GpuAllocator(err.into()))
            } else {
                Ok(())
            }?;
            unsafe {
                device.destroy_buffer(self.buffer, None);
            }
        }

        Ok(())
    }

    pub(crate) fn bind(&self, device: &ash::Device) -> RtResult<()> {
        if let Some(allocation) = self.allocation.as_ref()
            && let Err(err) = unsafe {
                device.bind_buffer_memory(self.buffer, allocation.memory(), allocation.offset())
            }
        {
            Err(RtError::BindBufferMemory(err.into()))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn buffer_info(&self) -> vk::DescriptorBufferInfo {
        vk::DescriptorBufferInfo::default()
            .buffer(self.buffer)
            .range(self.size)
    }
}
