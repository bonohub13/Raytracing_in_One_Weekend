pub struct DeviceMemory {
    pub device_memory: ash::vk::DeviceMemory,
}

impl DeviceMemory {
    pub fn new(
        instance: &crate::Instance,
        device: &crate::Device,
        size: u64,
        memory_type_bits: u32,
        allocate_flags: ash::vk::MemoryAllocateFlags,
        property_flags: ash::vk::MemoryPropertyFlags,
    ) -> Result<Self, String> {
        use ash::vk;

        log::info!("creating DeviceMemory");

        let memory_type_index =
            Self::find_memory_type(instance, device, memory_type_bits, property_flags)?;
        let mut flags_info = vk::MemoryAllocateFlagsInfo::builder()
            .flags(allocate_flags)
            .build();
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .push_next(&mut flags_info)
            .allocation_size(size)
            .memory_type_index(memory_type_index)
            .build();

        let device_memory = device.allocate_memory(&alloc_info, None)?;

        log::info!("created DeviceMemory");

        Ok(Self { device_memory })
    }

    pub fn map(
        &self,
        device: &crate::Device,
        offset: u64,
        size: u64,
    ) -> Result<*mut std::os::raw::c_void, String> {
        use ash::vk;
        use scopeguard::{guard, ScopeGuard};

        log::info!("mapping memory with offset: [{}], size: [{}]", offset, size);

        let data_sg = {
            let data = unsafe {
                device
                    .device
                    .map_memory(
                        self.device_memory,
                        offset,
                        size,
                        vk::MemoryMapFlags::empty(),
                    )
                    .map_err(|_| String::from("failed to map memory"))?
            };

            guard(data, |_| {
                log::warn!("map memory scopeguard");

                self.unmap(device);
            })
        };

        log::info!("mapped memory with offset: [{}], size: [{}]", offset, size);

        Ok(ScopeGuard::into_inner(data_sg))
    }

    pub fn unmap(&self, device: &crate::Device) {
        log::info!("unmapping memory");

        unsafe {
            device.device.unmap_memory(self.device_memory);
        }

        log::info!("unmapped memory");
    }

    fn find_memory_type(
        instance: &crate::Instance,
        device: &crate::Device,
        type_filter: u32,
        property_flags: ash::vk::MemoryPropertyFlags,
    ) -> Result<u32, String> {
        let mem_properties =
            instance.get_physical_device_memory_properties(device.physical_device());

        match (0..mem_properties.memory_type_count)
            .into_iter()
            .find(|&i| {
                type_filter & (1 << i) != 0
                    && (mem_properties.memory_types[i as usize]
                        .property_flags
                        .contains(property_flags))
            }) {
            Some(memory_type) => Ok(memory_type),
            None => Err(String::from("failed to find suitable memory type")),
        }
    }

    pub fn cleanup(device: &crate::Device, memory: &mut Self) {
        log::info!("performing cleanup for DeviceMemory");

        device.free_memory(memory.device_memory, None);
    }
}
