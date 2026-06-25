// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

mod aabb;
mod acceleration;
mod blas;
mod buffer;
mod descriptor;
mod image;
mod mesh;
mod texture;
mod tlas;
mod vertex;

pub use aabb::*;
pub use acceleration::*;
pub use blas::*;
pub use buffer::*;
pub use descriptor::*;
pub use image::*;
pub use mesh::*;
pub use texture::*;
pub use tlas::*;
pub use vertex::*;

use crate::{RtErr, RtError, VkState};
use ash::vk;
use gpu_allocator::vulkan::{self as vk_alloc, Allocation};

pub struct Allocator {
    pub(crate) allocator: vk_alloc::Allocator,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AccelerationStructureBuildSizesInfo {
    pub acceleration_structure_size: u64,
    pub build_scratch_size: u64,
    pub update_scratch_size: u64,
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

impl AccelerationStructureBuildSizesInfo {
    #[inline]
    pub const fn new(size_info: &vk::AccelerationStructureBuildSizesInfoKHR) -> Self {
        Self {
            acceleration_structure_size: size_info.acceleration_structure_size,
            build_scratch_size: size_info.build_scratch_size,
            update_scratch_size: size_info.update_scratch_size,
        }
    }
}
