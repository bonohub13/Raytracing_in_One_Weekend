use crate::core::{
    buffers,
    error::{RtError, RtResult},
};
use ash::vk;
use glam::{Vec2, Vec3A};
use gpu_allocator::vulkan as vk_alloc;
use std::sync::Arc;
use winit::window::Window;

#[repr(C, align(256))]
#[derive(Debug, Clone, Copy, Default)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    _pad: Vec2,
    center: Vec3A,
    pixel_delta_u: Vec3A,
    pixel_delta_v: Vec3A,
    pixel00_loc: Vec3A,
}

impl Camera {
    pub fn new(window: Arc<Window>) -> Self {
        const FOCAL_LENGTH: f32 = 1f32;

        let inner_size = window.inner_size();
        let (width, height) = (inner_size.width as f32, inner_size.height as f32);
        let viewport_height = 2f32;
        let viewport_width = viewport_height * (width / height);
        let center = Vec3A::ZERO;
        let viewport_u = glam::vec3a(viewport_width, 0f32, 0f32);
        let viewport_v = glam::vec3a(0f32, -viewport_height, 0f32);
        let pixel_delta_u = viewport_u / width;
        let pixel_delta_v = viewport_v / height;
        let viewport_upper_left =
            center - glam::vec3a(0f32, 0f32, FOCAL_LENGTH) - viewport_u / 2f32 - viewport_v / 2f32;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            width: inner_size.width,
            height: inner_size.height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            _pad: Vec2::default(),
        }
    }

    pub fn create_uniform_buffer(
        &self,
        allocator: &mut vk_alloc::Allocator,
        device: &ash::Device,
    ) -> RtResult<buffers::Buffer> {
        const BUFFER_NAME: &str = "Camera Uniform Buffer";

        let buffer_size = size_of::<Self>() as u64;
        let create_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let uniform_buffer = match unsafe { device.create_buffer(&create_info, None) } {
            Ok(buffer) => Ok(buffer),
            Err(err) => Err(RtError::CreateBuffer(err.into())),
        }?;
        let requirements = unsafe { device.get_buffer_memory_requirements(uniform_buffer) };
        let allocation = match allocator.allocate(&vk_alloc::AllocationCreateDesc {
            name: BUFFER_NAME,
            linear: true,
            requirements,
            location: gpu_allocator::MemoryLocation::CpuToGpu,
            allocation_scheme: vk_alloc::AllocationScheme::GpuAllocatorManaged,
        }) {
            Ok(allocation) => Ok(allocation),
            Err(err) => Err(RtError::GpuAllocator(err.into())),
        }?;
        let mut buffer = buffers::Buffer::new(uniform_buffer, buffer_size, allocation);

        self.update_buffer(&mut buffer);
        buffer.bind(device)?;

        Ok(buffer)
    }

    #[inline]
    pub fn update_buffer(&self, buffer: &mut buffers::Buffer) {
        buffer.write(*self);
    }
}
