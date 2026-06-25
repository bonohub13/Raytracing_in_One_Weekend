// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    AllocatedImage, Allocator, Device, Encoder, ImageType, RtErr, RtError, Swapchain, VkState,
};
use ash::vk;
use std::{
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};

pub struct Texture {
    image: ManuallyDrop<AllocatedImage>,
    sampler: vk::Sampler,
    device: Arc<Device>,
}

impl Texture {
    pub fn new(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        swapchain: &Swapchain,
        encoder: &Encoder,
    ) -> RtErr<Self> {
        let image = ManuallyDrop::new(AllocatedImage::new(
            state,
            allocator,
            encoder,
            ImageType::Storage(swapchain.extent()),
        )?);
        let sampler = Self::create_sampler(state)?;

        Ok(Self {
            image,
            sampler,
            device: state.device.clone(),
        })
    }
    pub(crate) fn image_info(&self) -> vk::DescriptorImageInfo {
        vk::DescriptorImageInfo {
            sampler: self.sampler,
            image_view: self.image.image_view(),
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        }
    }

    fn create_sampler(state: &VkState) -> RtErr<vk::Sampler> {
        let create_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .mip_lod_bias(0f32)
            .max_anisotropy(1f32)
            .anisotropy_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .compare_enable(false)
            .min_lod(0f32)
            .max_lod(0f32)
            .border_color(vk::BorderColor::FLOAT_OPAQUE_BLACK)
            .unnormalized_coordinates(false);

        unsafe { state.device.device().create_sampler(&create_info, None) }
            .map_err(|err| RtError::CreateSampler(err.into()))
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.device.device().destroy_sampler(self.sampler, None);
            ManuallyDrop::drop(&mut self.image);
        }
    }
}
