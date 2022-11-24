pub struct Buffer {
    pub buffer: ash::vk::Buffer,
}

impl Buffer {
    pub fn new(
        device: &crate::Device,
        size: u64,
        usage: ash::vk::BufferUsageFlags,
    ) -> Result<Self, String> {
        use ash::vk;

        let buffer_info = vk::BufferCreateInfo::builder()
            .size(size)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();

        let buffer = device.create_buffer(&buffer_info, None)?;

        Ok(Self { buffer })
    }

    pub fn allocate_memory(
        &self,
        instance: &crate::Instance,
        device: &crate::Device,
        allocate_flags: Option<ash::vk::MemoryAllocateFlags>,
        property_flags: ash::vk::MemoryPropertyFlags,
    ) -> Result<crate::DeviceMemory, String> {
        use ash::vk;

        let requirements = self.get_memory_requirements(device);
        let memory = crate::DeviceMemory::new(
            instance,
            device,
            requirements.size,
            requirements.memory_type_bits,
            match allocate_flags {
                Some(flags) => flags,
                None => vk::MemoryAllocateFlags::empty(),
            },
            property_flags,
        )?;

        device.bind_buffer_memory(self.buffer, memory.device_memory, 0)?;

        Ok(memory)
    }

    pub fn get_memory_requirements(&self, device: &crate::Device) -> ash::vk::MemoryRequirements {
        device.get_buffer_memory_requirements(self.buffer)
    }

    pub fn get_device_address(&self, device: &crate::Device) -> ash::vk::DeviceAddress {
        use ash::vk;

        let info = vk::BufferDeviceAddressInfo::builder()
            .buffer(self.buffer)
            .build();

        device.get_buffer_device_address(&info)
    }

    pub fn copy_from(
        &self,
        device: &crate::Device,
        pool: &crate::CommandPool,
        src: &Self,
        size: ash::vk::DeviceSize,
    ) -> Result<(), String> {
        crate::utils::SingleTimeCommands::submit(device, pool, |command_buffer| {
            use ash::vk;

            let copy_region = vk::BufferCopy::builder()
                .src_offset(0)
                .dst_offset(0)
                .size(size)
                .build();

            device.cmd_copy_buffer(command_buffer, src.buffer, self.buffer, &[copy_region]);

            Ok(())
        })
    }

    pub fn cleanup(device: &crate::Device, buffer: &mut Self) {
        log::info!("performing cleanup for Buffer");

        device.destroy_buffer(buffer.buffer, None);
    }
}
