// Copyright 2026 Kensuke
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::core::{RtError, RtResult, device::Device, params, surface::Surface};
use ash::vk;
use gpu_allocator::vulkan::{self, Allocation};
use std::sync::Arc;

pub struct TextureDescriptor {
    pub surface: Arc<Surface>,
    pub device: Arc<Device>,
}

pub struct Texture {
    surface: Arc<Surface>,
    device: Arc<Device>,
    images: Vec<vk::Image>,
    allocation: Allocation,
    image_views: Vec<vk::ImageView>,
    samplers: Vec<vk::Sampler>,
}

impl Texture {
    pub fn new(desc: &TextureDescriptor) -> RtResult<Self> {
        let images: Vec<vk::Image> = (0..params::MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_image())
            .collect::<RtResult<_>>()?;
        let allocation = Self::create_allocation(&images)?;
        let image_views: Vec<vk::ImageView> = (0..params::MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_image_view())
            .collect::<RtResult<_>>()?;
        let samplers: Vec<vk::Sampler> = (0..params::MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_sampler())
            .collect::<RtResult<_>>()?;

        Ok(Self {
            surface: desc.surface.clone(),
            device: desc.device.clone(),
            images,
            allocation,
            image_views,
            samplers,
        })
    }

    #[inline]
    pub const fn images(&self) -> &[vk::Image] {
        self.images.as_slice()
    }

    #[inline]
    pub const fn image_views(&self) -> &[vk::ImageView] {
        self.image_views.as_slice()
    }

    #[inline]
    pub const fn samplers(&self) -> &[vk::Sampler] {
        self.samplers.as_slice()
    }

    fn create_image() -> RtResult<vk::Image> {
        todo!()
    }

    fn create_allocation(images: &[vk::Image]) -> RtResult<Allocation> {
        todo!()
    }

    fn create_image_view() -> RtResult<vk::ImageView> {
        todo!()
    }

    fn create_sampler() -> RtResult<vk::Sampler> {
        todo!()
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        let device = self.device.raw();

        self.samplers
            .iter()
            .for_each(|sampler| unsafe { device.destroy_sampler(*sampler, None) });
        self.image_views
            .iter()
            .for_each(|image_view| unsafe { device.destroy_image_view(*image_view, None) });
        self.images
            .iter()
            .for_each(|image| unsafe { device.destroy_image(*image, None) });
    }
}
