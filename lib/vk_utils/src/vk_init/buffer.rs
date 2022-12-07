pub fn create_buffer(
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    device: &ash::Device,
    size: ash::vk::DeviceSize,
    usage: ash::vk::BufferUsageFlags,
    memory_property: ash::vk::MemoryPropertyFlags,
) -> Result<crate::VkBuffer, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating VkBuffer");

    let create_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .build();

    log::info!("creating buffer");

    let buffer_sg = {
        let buffer = unsafe {
            device
                .create_buffer(&create_info, None)
                .map_err(|_| String::from("failed to create buffer"))?
        };

        guard(buffer, |buffer| {
            log::warn!("buffer scopeguard");

            unsafe {
                device.destroy_buffer(buffer, None);
            }
        })
    };

    log::info!("created buffer");

    let memory_requirements = unsafe { device.get_buffer_memory_requirements(*buffer_sg) };
    let memory_type_index = crate::vk_init::find_memory_type_index(
        instance,
        physical_device,
        memory_requirements.memory_type_bits,
        memory_property,
    )?;
    let alloc_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(memory_requirements.size)
        .memory_type_index(memory_type_index)
        .build();

    log::info!("creating memory");

    let memory_sg = {
        let memory = unsafe {
            device
                .allocate_memory(&alloc_info, None)
                .map_err(|_| String::from("failed to allocate memory"))?
        };

        guard(memory, |mem| {
            log::warn!("memory scopeguard");

            unsafe {
                device.free_memory(mem, None);
            }
        })
    };

    log::info!("created memory");

    log::info!("created VkBuffer");

    Ok(crate::VkBuffer {
        memory: ScopeGuard::into_inner(memory_sg),
        buffer: ScopeGuard::into_inner(buffer_sg),
    })
}
