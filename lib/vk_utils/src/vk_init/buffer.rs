pub fn create_buffer(
    device: &ash::Device,
    memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
    size: ash::vk::DeviceSize,
    usage_flags: ash::vk::BufferUsageFlags,
    memory_property_flags: ash::vk::MemoryPropertyFlags,
) -> Result<crate::VkBuffer, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating VkBuffer");

    let create_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(usage_flags)
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
        memory_requirements.memory_type_bits,
        memory_properties,
        memory_property_flags,
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

    log::info!("binding buffer memory");

    unsafe {
        device
            .bind_buffer_memory(*buffer_sg, *memory_sg, 0)
            .map_err(|_| String::from("failed to bind buffer memory"))?;
    }

    log::info!("bound buffer memory");

    log::info!("created VkBuffer");

    Ok(crate::VkBuffer {
        memory: ScopeGuard::into_inner(memory_sg),
        buffer: ScopeGuard::into_inner(buffer_sg),
        size,
    })
}

pub fn map_buffer(
    device: &ash::Device,
    buffer: &crate::VkBuffer,
) -> Result<*mut std::ffi::c_void, String> {
    use ash::vk;

    log::info!("mapping memory for buffer");

    let data = unsafe {
        device
            .map_memory(buffer.memory, 0, buffer.size, vk::MemoryMapFlags::empty())
            .map_err(|_| String::from("failed to map memory for buffer"))?
    };

    log::info!("mapped memory for buffer");

    Ok(data)
}

pub fn copy_to_mapped_memory<T>(pointer: *mut T, data: T) -> *mut T {
    use std::mem::size_of_val;

    log::info!("copying data to mapped memory");

    let data_size = size_of_val(&data);
    let data_ptr = Box::new(data);

    unsafe {
        Box::<T>::into_raw(data_ptr).copy_to(pointer as *mut T, 1);
    }

    log::info!("copied data to mapped memory");

    return pointer as *mut T;
}
