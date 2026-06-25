// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Device, PipelineLayout, RtErr, RtError, VkState};
use ash::vk;
use std::sync::Arc;

// Ignore frames in flight on the app side!
static DESCRIPTOR_POOL_SIZES: [vk::DescriptorPoolSize; 3] = [
    // Ray Tracing pipeline
    vk::DescriptorPoolSize {
        ty: vk::DescriptorType::ACCELERATION_STRUCTURE_KHR,
        descriptor_count: 1,
    },
    vk::DescriptorPoolSize {
        ty: vk::DescriptorType::STORAGE_IMAGE,
        descriptor_count: 1,
    },
    // Graphics pipeline
    vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 1,
    },
];

pub struct Descriptor {
    sets: Vec<vk::DescriptorSet>,
    pool: vk::DescriptorPool,
    set_layouts: Vec<vk::DescriptorSetLayout>,
    frames_in_flight: usize,
    device: Arc<Device>,
}

impl Descriptor {
    pub fn new(state: &VkState, frames_in_flight: u32) -> RtErr<Self> {
        let pool_sizes: Vec<_> = DESCRIPTOR_POOL_SIZES
            .iter()
            .copied()
            .map(|mut pool_size| {
                pool_size.descriptor_count *= frames_in_flight;

                pool_size
            })
            .collect();
        let pool = Self::create_descriptor_pool(state, &pool_sizes)?;

        Ok(Self {
            pool,
            frames_in_flight: frames_in_flight as usize,
            device: state.device.clone(),
            set_layouts: Vec::new(),
            sets: Vec::new(),
        })
    }

    #[inline]
    pub fn set_layouts(&self) -> &[vk::DescriptorSetLayout] {
        &self.set_layouts
    }

    #[inline]
    pub fn sets(&self, frame: usize) -> &[vk::DescriptorSet] {
        &self.sets[frame * self.frames_in_flight..(frame + 1) * self.frames_in_flight]
    }

    pub fn add_descriptor_set_layout(
        &mut self,
        state: &VkState,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> RtErr<()> {
        let set_layout = Self::create_descriptor_set_layout(state, bindings)?;

        for _ in 0..self.frames_in_flight {
            match Self::allocate_descriptor_sets(state, set_layout, self.pool) {
                Ok(mut sets) => {
                    self.set_layouts.push(set_layout);
                    self.sets.append(&mut sets);

                    Ok(())
                }
                Err(err) => {
                    unsafe {
                        self.device
                            .device()
                            .destroy_descriptor_set_layout(set_layout, None);
                    }
                    Err(err)
                }
            }?;
        }

        Ok(())
    }

    pub(crate) fn update_descriptor_sets(
        &self,
        descriptor_writes: &[vk::WriteDescriptorSet],
        descriptor_copies: &[vk::CopyDescriptorSet],
    ) {
        unsafe {
            self.device
                .device()
                .update_descriptor_sets(descriptor_writes, descriptor_copies)
        }
    }

    pub(crate) fn bind_descriptor_sets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: Arc<PipelineLayout>,
        first_set: u32,
        dynamic_offsets: &[u32],
        current_frame: usize,
    ) {
        let range = match pipeline_bind_point {
            vk::PipelineBindPoint::RAY_TRACING_KHR => 0..2,
            vk::PipelineBindPoint::GRAPHICS => 2..3,
            _ => todo!(),
        };
        unsafe {
            self.device.device().cmd_bind_descriptor_sets(
                command_buffer,
                pipeline_bind_point,
                layout.layout,
                first_set,
                &self.sets(current_frame)[range],
                dynamic_offsets,
            )
        }
    }

    fn create_descriptor_pool(
        state: &VkState,
        pool_sizes: &[vk::DescriptorPoolSize],
    ) -> RtErr<vk::DescriptorPool> {
        let max_sets: u32 = pool_sizes
            .iter()
            .map(|pool_size| pool_size.descriptor_count)
            .sum();
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(pool_sizes)
            .max_sets(max_sets);

        unsafe {
            state
                .device
                .device()
                .create_descriptor_pool(&create_info, None)
        }
        .map_err(|err| RtError::CreateDescriptorPool(err.into()))
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

    fn allocate_descriptor_sets(
        state: &VkState,
        set_layout: vk::DescriptorSetLayout,
        pool: vk::DescriptorPool,
    ) -> RtErr<Vec<vk::DescriptorSet>> {
        let alloc_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(std::slice::from_ref(&set_layout));

        unsafe { state.device.device().allocate_descriptor_sets(&alloc_info) }
            .map_err(|err| RtError::AllocateDescriptorSets(err.into()))
    }
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        let device = self.device.device();

        self.set_layouts
            .iter()
            .copied()
            .for_each(|set_layout| unsafe {
                device.destroy_descriptor_set_layout(set_layout, None)
            });
        unsafe {
            device.destroy_descriptor_pool(self.pool, None);
        }
    }
}
