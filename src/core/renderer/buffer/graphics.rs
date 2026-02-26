use crate::core::{
    RtResult,
    device::Device,
    params,
    renderer::buffer::{
        DescriptorSet,
        texture::{self, Texture},
    },
    surface::Surface,
};
use ash::vk;
use std::sync::Arc;

pub struct GraphicsBufferDescriptor<'desc> {
    pub projection_texture_name: &'desc str,
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
}

pub struct GraphicsBuffer {
    device: Arc<Device>,
    texture: Texture,
}

impl GraphicsBuffer {
    const PER_DESCRIPTOR_SET_COUNT: u32 = params::MAX_FRAMES_IN_FLIGHT as u32;

    pub fn new(desc: &GraphicsBufferDescriptor) -> RtResult<Self> {
        let texture = Texture::new(&texture::TextureDescriptor {
            name: desc.projection_texture_name,
            surface: desc.surface.clone(),
            device: desc.device.clone(),
        })?;

        Ok(Self {
            device: desc.device.clone(),
            texture,
        })
    }

    pub fn render_transition_barrier(
        &self,
        current_frame: usize,
    ) -> vk::ImageMemoryBarrier2<'static> {
        const SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::NONE)
            .src_access_mask(vk::AccessFlags2::NONE)
            .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
            .dst_access_mask(vk::AccessFlags2::SHADER_READ)
            .old_layout(vk::ImageLayout::GENERAL)
            .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            // TODO: Remove UNDEFINED after acceleration pipeline is implementated
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::UNDEFINED)
            .image(self.texture.images()[current_frame])
            .subresource_range(SUBRESOURCE_RANGE)
    }

    pub fn write_descriptor_sets(&self, descriptor_set: &DescriptorSet) {
        descriptor_set
            .graphics_sets()
            .iter()
            .enumerate()
            .for_each(|(current_frame, set)| {
                let image_info = self.texture.read_only_image_info(current_frame);
                let write = vk::WriteDescriptorSet::default()
                    .dst_set(*set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .descriptor_count(1)
                    .image_info(std::slice::from_ref(&image_info));

                unsafe {
                    self.device
                        .raw()
                        .update_descriptor_sets(std::slice::from_ref(&write), &[])
                }
            })
    }

    pub fn layout_bindings() -> Vec<vk::DescriptorSetLayoutBinding<'static>> {
        [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)]
        .to_vec()
    }

    pub fn pool_sizes() -> Vec<vk::DescriptorPoolSize> {
        vec![
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(Self::PER_DESCRIPTOR_SET_COUNT),
        ]
    }
}
