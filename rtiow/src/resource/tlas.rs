// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use crate::{
    AccelerationStructureBuildSizesInfo, Allocator, AsLoader, Buffer, BufferType, RtErr, RtError,
    VkState,
};
use ash::vk;
use std::{
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};

pub struct Tlas {
    handle: vk::AccelerationStructureKHR,
    buffer: ManuallyDrop<Buffer<()>>,
    as_loader: Arc<AsLoader>,
    size_info: AccelerationStructureBuildSizesInfo,
}

impl Tlas {
    pub fn new(
        state: &VkState,
        allocator: Arc<Mutex<Allocator>>,
        loader: Arc<AsLoader>,
        max_instance_count: u32,
    ) -> RtErr<Self> {
        let size_info =
            Self::query_size_info(loader.clone(), std::slice::from_ref(&max_instance_count));
        let buffer = ManuallyDrop::new(Buffer::<()>::new(
            state,
            allocator.clone(),
            BufferType::Tlas(size_info.acceleration_structure_size),
        )?);
        let handle = Self::create_tlas(loader.clone(), &buffer, &size_info)?;

        Ok(Self {
            as_loader: loader,
            buffer,
            handle,
            size_info,
        })
    }

    fn query_size_info(
        loader: Arc<AsLoader>,
        max_instance_counts: &[u32],
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
        let build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(vk::AccelerationStructureTypeKHR::TOP_LEVEL)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(std::slice::from_ref(&mock_geometry_info));
        let mut size_info = vk::AccelerationStructureBuildSizesInfoKHR::default();

        unsafe {
            loader.raw().get_acceleration_structure_build_sizes(
                vk::AccelerationStructureBuildTypeKHR::DEVICE,
                &build_info,
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
        }
    }
}
