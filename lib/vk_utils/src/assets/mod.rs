#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    pub model_view: cgmath::Matrix4<f32>,
    pub projection: cgmath::Matrix4<f32>,
    pub model_view_inverse: cgmath::Matrix4<f32>,
    pub projection_inverse: cgmath::Matrix4<f32>,

    pub aperture: f32,
    pub focus_distance: f32,
    pub heatmap_scale: f32,

    pub total_number_of_samples: u32,
    pub number_of_samples: u32,
    pub number_of_bounces: u32,
    pub random_seed: u32,
    pub has_sky: bool,
    pub show_heat_map: bool,
}

impl Default for UniformBufferObject {
    fn default() -> Self {
        use cgmath::Matrix4;

        let zeroed = Matrix4::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        );

        Self {
            model_view: zeroed,
            projection: zeroed,
            model_view_inverse: zeroed,
            projection_inverse: zeroed,

            aperture: 0.0,
            focus_distance: 0.0,
            heatmap_scale: 0.0,

            total_number_of_samples: 0,
            number_of_bounces: 0,
            number_of_samples: 0,
            random_seed: 0,
            has_sky: false,
            show_heat_map: false,
        }
    }
}

pub struct UniformBuffer {
    pub buffer: crate::Buffer,
    memory: crate::DeviceMemory,
}

impl UniformBuffer {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Result<Self, String> {
        use ash::vk;
        use std::mem::size_of_val;

        log::info!("creating UniformBuffer");

        let buffer_size = size_of_val(&UniformBufferObject::default());

        let buffer = crate::Buffer::new(
            device,
            buffer_size as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
        )?;
        let memory = buffer.allocate_memory(
            instance,
            device,
            None,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        log::info!("created UniformBuffer");

        Ok(Self { memory, buffer })
    }

    pub fn set_value(
        &self,
        device: &crate::Device,
        ubo: &UniformBufferObject,
    ) -> Result<(), String> {
        use std::{mem::size_of_val, ptr::copy_nonoverlapping};

        let data = self.memory.map(device, 0, size_of_val(ubo) as u64)?;

        unsafe {
            copy_nonoverlapping(ubo, data as *mut UniformBufferObject, size_of_val(ubo));
        }

        self.memory.unmap(device);

        Ok(())
    }

    pub fn cleanup(device: &crate::Device, ub: &mut Self) {
        log::info!("performing cleanup for UniformBuffer");

        crate::DeviceMemory::cleanup(device, &mut ub.memory);
        crate::Buffer::cleanup(device, &mut ub.buffer);
    }
}
