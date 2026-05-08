// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Device, RtErr, RtError, VkState};
use ash::vk;
use std::sync::Arc;

pub struct Descriptor {
    sets: Vec<vk::DescriptorSet>,
    pool: vk::DescriptorPool,
    set_layout: vk::DescriptorSetLayout,
    device: Arc<Device>,
}

impl Descriptor {
    pub fn new(
        state: &VkState,
        bindings: &[vk::DescriptorSetLayoutBinding],
        frames_in_flight: u32,
    ) -> RtErr<Self> {
        let set_layout = Self::create_descriptor_set_layout(state, bindings)?;
        let pool = Self::create_descriptor_pool(state, frames_in_flight)?;
        let sets = Self::allocate_descriptor_sets(state, frames_in_flight, set_layout, pool)?;

        Ok(Self {
            set_layout,
            pool,
            sets,
            device: state.device.clone(),
        })
    }

    #[inline]
    pub fn set_layout(&self) -> vk::DescriptorSetLayout {
        self.set_layout
    }

    #[inline]
    pub fn sets(&self) -> &[vk::DescriptorSet] {
        &self.sets
    }

    fn create_descriptor_set_layout(
        state: &VkState,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> RtErr<vk::DescriptorSetLayout> {
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(bindings);

        unsafe {
            state
                .device
                .device()
                .create_descriptor_set_layout(&create_info, None)
        }
        .map_err(|err| RtError::CreateDescriptorSetLayout(err.into()))
    }

    fn create_descriptor_pool(state: &VkState, frames_in_flight: u32) -> RtErr<vk::DescriptorPool> {
        let pool_size = vk::DescriptorPoolSize::default().descriptor_count(frames_in_flight);
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(std::slice::from_ref(&pool_size))
            .max_sets(frames_in_flight);

        unsafe {
            state
                .device
                .device()
                .create_descriptor_pool(&create_info, None)
        }
        .map_err(|err| RtError::CreateDescriptorPool(err.into()))
    }

    fn allocate_descriptor_sets(
        state: &VkState,
        frames_in_flight: u32,
        set_layout: vk::DescriptorSetLayout,
        pool: vk::DescriptorPool,
    ) -> RtErr<Vec<vk::DescriptorSet>> {
        let layouts = vec![set_layout; frames_in_flight as usize];
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&layouts);

        unsafe { state.device.device().allocate_descriptor_sets(&alloc_info) }
            .map_err(|err| RtError::AllocateDescriptorSets(err.into()))
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        let device = self.device.device();

        unsafe {
            device.destroy_descriptor_set_layout(self.set_layout, None);
            device.destroy_descriptor_pool(self.pool, None);
        }
    }
}
