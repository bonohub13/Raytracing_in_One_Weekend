// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    Aabb, Allocator, AsLoader, Buffer, BufferData, BufferType, Encoder, Mesh, RtErr, RtError,
    VkState,
};
use ash::vk;
use std::{
    marker::PhantomData,
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};

pub struct Blas<T>
where
    T: Clone + Sized,
{
    handle: vk::AccelerationStructureKHR,
    buffer: ManuallyDrop<Buffer<T>>,
    as_loader: Arc<AsLoader>,
    _ty: PhantomData<T>,
}

impl<T> Blas<T>
where
    T: Clone + Sized,
{
    pub fn new(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        loader: Arc<AsLoader>,
        ty: BufferType,
    ) -> RtErr<Self> {
        match ty {
            BufferType::Vertex(mesh) => {
                Self::create_mesh_blas(state, allocator, encoder, loader, mesh)
            }
            BufferType::Aabb(aabb) => {
                Self::create_aabb_blas(state, allocator, encoder, loader, aabb)
            }
            _ => Err(RtError::InvalidBufferType), // Treat as error
        }
    }

    pub fn device_address(&self) -> vk::DeviceAddress {
        self.buffer.gpu_address()[0]
    }

    fn create_mesh_blas(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        loader: Arc<AsLoader>,
        mesh: &Mesh,
    ) -> RtErr<Self> {
        let buffer = {
            let buffer = Buffer::<Mesh>::new(state, allocator.clone(), BufferType::Vertex(mesh))?;
            let mut staging_buffer =
                Buffer::<Mesh>::new(state, allocator.clone(), BufferType::Staging(mesh.size()))?;

            staging_buffer.write(encoder, &buffer, mesh)?;

            buffer
        };
        let mesh_data = mesh.geometry_data(&buffer);
        let blas_mesh_geometry = vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::TRIANGLES)
            .geometry(vk::AccelerationStructureGeometryDataKHR {
                triangles: mesh_data,
            })
            .flags(vk::GeometryFlagsKHR::OPAQUE);
        let (handle, blas_buffer) = Self::create_blas_buffer(
            state,
            allocator,
            encoder,
            loader.clone(),
            std::slice::from_ref(&blas_mesh_geometry),
        )?;

        Ok(Self {
            as_loader: loader,
            handle,
            buffer: ManuallyDrop::new(blas_buffer),
            _ty: PhantomData,
        })
    }

    fn create_aabb_blas(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        loader: Arc<AsLoader>,
        aabb: &Aabb,
    ) -> RtErr<Self> {
        let buffer = Buffer::<Aabb>::new(state, allocator.clone(), BufferType::Aabb(aabb))?;
        let aabb_data = aabb.geometry_data(&buffer);
        let blas_aabb_geometry = vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::AABBS)
            .geometry(vk::AccelerationStructureGeometryDataKHR { aabbs: aabb_data })
            .flags(vk::GeometryFlagsKHR::OPAQUE);
        let (handle, blas_buffer) = Self::create_blas_buffer(
            state,
            allocator,
            encoder,
            loader.clone(),
            std::slice::from_ref(&blas_aabb_geometry),
        )?;

        Ok(Self {
            as_loader: loader,
            handle,
            buffer: ManuallyDrop::new(blas_buffer),
            _ty: PhantomData,
        })
    }

    fn create_blas_buffer(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        encoder: &Encoder,
        as_loader: Arc<AsLoader>,
        geometries: &[vk::AccelerationStructureGeometryKHR],
    ) -> RtErr<(vk::AccelerationStructureKHR, Buffer<T>)> {
        static MAX_PRIMITIVE_COUNTS: [u32; 1] = [1];
        static BUILD_RANGE_INFOS: [vk::AccelerationStructureBuildRangeInfoKHR; 1] =
            [vk::AccelerationStructureBuildRangeInfoKHR {
                primitive_count: MAX_PRIMITIVE_COUNTS[0],
                primitive_offset: 0,
                first_vertex: 0,
                transform_offset: 0,
            }];
        static STRUCTURAL_BARRIER: vk::MemoryBarrier2 = vk::MemoryBarrier2 {
            s_type: vk::StructureType::MEMORY_BARRIER_2,
            p_next: std::ptr::null(),
            src_stage_mask: vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR,
            src_access_mask: vk::AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR,
            dst_stage_mask: vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR,
            dst_access_mask: vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR,
            _marker: PhantomData,
        };

        let build_geometry_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(geometries)
            .scratch_data(vk::DeviceOrHostAddressKHR { device_address: 0 });
        let size_info = {
            let mut size_info = vk::AccelerationStructureBuildSizesInfoKHR::default();

            unsafe {
                as_loader.raw().get_acceleration_structure_build_sizes(
                    vk::AccelerationStructureBuildTypeKHR::DEVICE,
                    &build_geometry_info,
                    &MAX_PRIMITIVE_COUNTS,
                    &mut size_info,
                )
            };

            size_info
        };
        let blas_buffer = Buffer::<T>::new(
            state,
            allocator.clone(),
            BufferType::Blas(size_info.acceleration_structure_size),
        )?;
        let handle = Self::create_blas(as_loader.clone(), &blas_buffer, &size_info)?;
        let scratch_buffer = Buffer::<()>::new(
            state,
            allocator,
            BufferType::Scratch(size_info.build_scratch_size),
        )?;
        let build_geometry_info = build_geometry_info
            .dst_acceleration_structure(handle)
            .scratch_data(vk::DeviceOrHostAddressKHR {
                device_address: scratch_buffer.gpu_address()[0],
            });
        let dependency_info = vk::DependencyInfo::default()
            .memory_barriers(std::slice::from_ref(&STRUCTURAL_BARRIER));

        encoder.submit_single_command_buffer(|command_buffer| unsafe {
            as_loader.raw().cmd_build_acceleration_structures(
                command_buffer,
                std::slice::from_ref(&build_geometry_info),
                std::slice::from_ref(&BUILD_RANGE_INFOS.as_ref()),
            );
            state
                .device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
        })?;

        Ok((handle, blas_buffer))
    }

    fn create_blas(
        as_loader: Arc<AsLoader>,
        buffer: &Buffer<T>,
        size_info: &vk::AccelerationStructureBuildSizesInfoKHR,
    ) -> RtErr<vk::AccelerationStructureKHR> {
        let create_info = vk::AccelerationStructureCreateInfoKHR::default()
            .buffer(buffer.buffer())
            .offset(0)
            .size(size_info.acceleration_structure_size)
            .ty(vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL);

        unsafe {
            as_loader
                .raw()
                .create_acceleration_structure(&create_info, None)
        }
        .map_err(|err| RtError::CreateAccelerationStructure(err.into()))
    }
}

impl<T> Drop for Blas<T>
where
    T: Clone + Sized,
{
    fn drop(&mut self) {
        unsafe {
            self.as_loader
                .raw()
                .destroy_acceleration_structure(self.handle, None);
            ManuallyDrop::drop(&mut self.buffer);
        }
    }
}
