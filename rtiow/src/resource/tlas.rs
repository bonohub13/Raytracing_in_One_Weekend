// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    AccelerationStructureBuildSizesInfo, Allocator, AsLoader, Blas, Buffer, BufferType, RtErr,
    RtError, VkState,
};
use ash::vk;
use glam::Mat4;
use std::{
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TlasType {
    Mesh,
    Aabb,
}

pub struct TlasInstanceData<'blas, T>
where
    T: Clone + Sized,
{
    data: Mat4,
    blas: &'blas Blas<T>,
    ty: TlasType,
    custom_index: u32,
}

pub struct Tlas {
    handle: vk::AccelerationStructureKHR,
    buffer: ManuallyDrop<Buffer<()>>,
    tlas_instance_buffer: ManuallyDrop<Buffer<vk::AccelerationStructureInstanceKHR>>,
    as_loader: Arc<AsLoader>,
}

impl TlasType {
    fn sbt_record_offset_and_flags(&self) -> vk::Packed24_8 {
        const MESH_OFFSET: u32 = 0;
        const AABB_OFFSET: u32 = 1;

        match self {
            TlasType::Mesh => {
                let flags_raw = vk::GeometryInstanceFlagsKHR::empty().as_raw() as u8;

                vk::Packed24_8::new(MESH_OFFSET, flags_raw)
            }
            TlasType::Aabb => {
                let flags_raw =
                    vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE.as_raw() as u8;

                vk::Packed24_8::new(AABB_OFFSET, flags_raw)
            }
        }
    }
}

impl<'blas, T> TlasInstanceData<'blas, T>
where
    T: Clone + Sized,
{
    #[inline]
    pub const fn new(
        col0: &[f32; 3],
        col1: &[f32; 3],
        col2: &[f32; 3],
        col3: &[f32; 4],
        blas: &'blas Blas<T>,
        ty: TlasType,
        custom_index: u32,
    ) -> Self {
        let col0 = glam::vec4(col0[0], col0[1], col0[2], 0f32);
        let col1 = glam::vec4(col1[0], col1[1], col1[2], 0f32);
        let col2 = glam::vec4(col2[0], col2[1], col2[2], 0f32);
        let col3 = glam::vec4(col3[0], col3[1], col3[2], 0f32);

        Self {
            data: glam::mat4(col0, col1, col2, col3),
            blas,
            ty,
            custom_index,
        }
    }
}

impl Tlas {
    pub fn new(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        loader: Arc<AsLoader>,
        max_instance_count: u32,
    ) -> RtErr<Self> {
        let size_info = Self::query_size_info(
            loader.clone(),
            std::slice::from_ref(&max_instance_count),
            None,
        );
        let buffer = ManuallyDrop::new(Buffer::<()>::new(
            state,
            allocator.clone(),
            BufferType::Tlas(size_info.acceleration_structure_size),
        )?);
        let tlas_instance_buffer =
            ManuallyDrop::new(Buffer::<vk::AccelerationStructureInstanceKHR>::new(
                state,
                allocator.clone(),
                BufferType::TlasInstance(max_instance_count as usize),
            )?);
        let handle = Self::create_tlas(loader.clone(), &buffer, &size_info)?;

        Ok(Self {
            as_loader: loader,
            buffer,
            tlas_instance_buffer,
            handle,
        })
    }

    pub fn update<T>(
        &self,
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        command_buffer: vk::CommandBuffer,
        instances: &[TlasInstanceData<T>],
    ) -> RtErr<()>
    where
        T: Clone + Sized,
    {
        if let Some(allocation) = self.tlas_instance_buffer.allocation()
            && let Some(alloc_ptr) = allocation.mapped_ptr()
        {
            let dst_ptr = alloc_ptr.as_ptr() as *mut vk::AccelerationStructureInstanceKHR;
            let instance_count = instances.len() as u32;
            let instance_geometry_data =
                vk::AccelerationStructureGeometryInstancesDataKHR::default()
                    .array_of_pointers(false)
                    .data(vk::DeviceOrHostAddressConstKHR {
                        device_address: self.tlas_instance_buffer.gpu_address()[0],
                    });
            let geometry_info = vk::AccelerationStructureGeometryKHR::default()
                .geometry_type(vk::GeometryTypeKHR::INSTANCES)
                .geometry(vk::AccelerationStructureGeometryDataKHR {
                    instances: instance_geometry_data,
                })
                .flags(vk::GeometryFlagsKHR::OPAQUE);
            let geometry_ptr = [&geometry_info];
            let mut build_geometry_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
                .ty(vk::AccelerationStructureTypeKHR::TOP_LEVEL)
                .flags(vk::BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE)
                .mode(vk::BuildAccelerationStructureModeKHR::UPDATE)
                .src_acceleration_structure(self.handle)
                .dst_acceleration_structure(self.handle)
                .geometries_ptrs(&geometry_ptr);
            let build_range_info = vk::AccelerationStructureBuildRangeInfoKHR::default()
                .primitive_count(instances.len() as u32)
                .primitive_offset(0)
                .first_vertex(0)
                .transform_offset(0);
            let size_info = Self::query_size_info(
                self.as_loader.clone(),
                std::slice::from_ref(&instance_count),
                Some(&build_geometry_info),
            );
            let scratch_buffer = Buffer::<()>::new(
                state,
                allocator,
                BufferType::Scratch(size_info.update_scratch_size),
            )?;

            build_geometry_info = build_geometry_info.scratch_data(vk::DeviceOrHostAddressKHR {
                device_address: scratch_buffer.gpu_address()[0],
            });

            instances.iter().enumerate().for_each(|(i, instance)| {
                let transpose = instance.data.transpose();
                let cols = transpose.to_cols_array_2d();
                let transform = vk::TransformMatrixKHR {
                    matrix: [
                        cols[0][0], cols[0][1], cols[0][2], cols[0][3], // Column 0
                        cols[1][0], cols[1][1], cols[1][2], cols[1][3], // Column 1
                        cols[2][0], cols[2][1], cols[2][2], cols[2][3], // Column 2
                    ],
                };
                let desc = vk::AccelerationStructureInstanceKHR {
                    transform,
                    instance_custom_index_and_mask: vk::Packed24_8::new(
                        instance.custom_index,
                        0xFF,
                    ),
                    instance_shader_binding_table_record_offset_and_flags: instance
                        .ty
                        .sbt_record_offset_and_flags(),
                    acceleration_structure_reference: vk::AccelerationStructureReferenceKHR {
                        device_handle: instance.blas.device_address(),
                    },
                };

                unsafe { *dst_ptr.add(i) = desc };
            });
            self.as_loader.build_acceleration_structures(
                command_buffer,
                std::slice::from_ref(&build_geometry_info),
                std::slice::from_ref(&std::slice::from_ref(&build_range_info)),
            );

            Ok(())
        } else {
            Err(RtError::NoAllocation)
        }
    }

    fn query_size_info(
        loader: Arc<AsLoader>,
        max_instance_counts: &[u32],
        build_info: Option<&vk::AccelerationStructureBuildGeometryInfoKHR>,
    ) -> AccelerationStructureBuildSizesInfo {
        let mock_instance_data = vk::AccelerationStructureGeometryInstancesDataKHR::default()
            .array_of_pointers(false)
            .data(vk::DeviceOrHostAddressConstKHR { device_address: 0 });
        let mock_geometry_info = vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::INSTANCES)
            .geometry(vk::AccelerationStructureGeometryDataKHR {
                instances: mock_instance_data,
            })
            .flags(vk::GeometryFlagsKHR::OPAQUE);
        let default_build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::TOP_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(std::slice::from_ref(&mock_geometry_info));
        let mut size_info = vk::AccelerationStructureBuildSizesInfoKHR::default();

        unsafe {
            loader.raw().get_acceleration_structure_build_sizes(
                vk::AccelerationStructureBuildTypeKHR::DEVICE,
                if let Some(build_info) = build_info {
                    build_info
                } else {
                    &default_build_info
                },
                max_instance_counts,
                &mut size_info,
            )
        };

        AccelerationStructureBuildSizesInfo::new(&size_info)
    }

    fn create_tlas(
        loader: Arc<AsLoader>,
        buffer: &Buffer<()>,
        size_info: &AccelerationStructureBuildSizesInfo,
    ) -> RtErr<vk::AccelerationStructureKHR> {
        let create_info = vk::AccelerationStructureCreateInfoKHR::default()
            .buffer(buffer.buffer())
            .offset(0)
            .size(size_info.acceleration_structure_size)
            .ty(vk::AccelerationStructureTypeKHR::TOP_LEVEL);

        unsafe {
            loader
                .raw()
                .create_acceleration_structure(&create_info, None)
        }
        .map_err(|err| RtError::CreateAccelerationStructure(err.into()))
    }
}

impl Drop for Tlas {
    fn drop(&mut self) {
        unsafe {
            self.as_loader
                .raw()
                .destroy_acceleration_structure(self.handle, None);
            ManuallyDrop::drop(&mut self.buffer);
            ManuallyDrop::drop(&mut self.tlas_instance_buffer);
        }
    }
}
