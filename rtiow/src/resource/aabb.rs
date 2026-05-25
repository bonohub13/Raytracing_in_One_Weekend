// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{Buffer, StagingData};
use ash::vk;
use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Aabb(pub(crate) vk::AabbPositionsKHR);

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    position: Vec3,
    radius: f32,
}

impl Aabb {
    pub fn bytes(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[
            self.0.min_x,
            self.0.min_y,
            self.0.min_z,
            self.0.max_x,
            self.0.max_y,
            self.0.max_z,
        ])
        .to_vec()
    }

    pub fn geometry_data<T>(
        &self,
        buffer: &Buffer<T>,
    ) -> vk::AccelerationStructureGeometryAabbsDataKHR<'_>
    where
        T: Clone + Sized,
    {
        vk::AccelerationStructureGeometryAabbsDataKHR::default()
            .data(vk::DeviceOrHostAddressConstKHR {
                device_address: buffer.gpu_address()[0],
            })
            .stride(self.size())
    }
}

impl Sphere {
    pub fn new(position: &[f32; 3], radius: f32) -> Self {
        Self {
            position: glam::vec3(position[0], position[1], position[2]),
            radius,
        }
    }

    pub const fn aabb_data(&self) -> Aabb {
        Aabb(vk::AabbPositionsKHR {
            min_x: self.position.x - self.radius,
            max_x: self.position.x + self.radius,
            min_y: self.position.y - self.radius,
            max_y: self.position.y + self.radius,
            min_z: self.position.z - self.radius,
            max_z: self.position.z + self.radius,
        })
    }
}

impl StagingData for Aabb {
    fn size(&self) -> vk::DeviceSize {
        size_of_val(&self.0) as vk::DeviceSize
    }

    fn copy_regions<'a>(&'a self) -> Vec<vk::BufferCopy2<'a>> {
        let size = self.size();

        vec![vk::BufferCopy2::default()
            .src_offset(0)
            .dst_offset(0)
            .size(size)]
    }

    fn copy_barrier<'a>(&self) -> vk::MemoryBarrier2<'a> {
        vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR)
            .dst_access_mask(vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR)
    }

    fn write(&self, dst_slice: &mut [u8]) {
        let size = self.size() as usize;
        let bytes = self.bytes();

        dst_slice[0..size].copy_from_slice(&bytes);
    }
}
