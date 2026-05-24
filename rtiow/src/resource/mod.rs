// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod aabb;
mod acceleration;
mod blas;
mod buffer;
mod descriptor;
mod image;
mod mesh;
mod vertex;

pub use aabb::*;
pub use acceleration::*;
pub use blas::*;
pub use buffer::*;
pub use descriptor::*;
pub use image::*;
pub use mesh::*;
pub use vertex::*;

use crate::{RtErr, RtError, VkState};
use gpu_allocator::vulkan::{self as vk_alloc, Allocation};

pub struct Allocator {
    pub(crate) allocator: vk_alloc::Allocator,
}

impl Allocator {
    pub fn new(state: &VkState) -> RtErr<Self> {
        let allocator = vk_alloc::Allocator::new(&vk_alloc::AllocatorCreateDesc {
            instance: state.instance.instance().clone(),
            device: state.device.device().clone(),
            physical_device: state.device.physical_device(),
            buffer_device_address: true,
            debug_settings: Default::default(),
            allocation_sizes: Default::default(),
        })
        .map_err(|err| RtError::InitAllocator(err.into()))?;

        Ok(Self { allocator })
    }

    pub(crate) fn free(&mut self, allocation: Allocation) -> RtErr<()> {
        self.allocator
            .free(allocation)
            .map_err(|err| RtError::FreeAllocation(err.into()))
    }
}
