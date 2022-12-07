pub fn create_image(
    settings: &crate::VkSettings,
    instance: &ash::Instance,
    physical_device: ash::vk::PhysicalDevice,
    device: &ash::Device,
    format: ash::vk::Format,
    usage: ash::vk::ImageUsageFlags,
) -> Result<crate::VkImage, String> {
    use crate::vk_init::find_memory_type_index;
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating VkImage");

    let create_info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::TYPE_2D)
        .format(format)
        .extent(vk::Extent3D {
            width: settings.window_width,
            height: settings.window_height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .build();

    log::info!("creating image");

    let image_sg = {
        let image = unsafe {
            device
                .create_image(&create_info, None)
                .map_err(|_| String::from("failed to create image"))?
        };

        guard(image, |image| {
            log::warn!("image scopeguard");

            unsafe {
                device.destroy_image(image, None);
            }
        })
    };

    log::info!("created image");

    let memory_requirement = unsafe { device.get_image_memory_requirements(*image_sg) };

    let memory_type_index = find_memory_type_index(
        instance,
        physical_device,
        memory_requirement.memory_type_bits,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;
    let alloc_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(memory_requirement.size)
        .memory_type_index(memory_type_index)
        .build();

    log::info!("allocating memory");

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

    log::info!("binding allocated memory to image");

    unsafe {
        device
            .bind_image_memory(*image_sg, *memory_sg, 0)
            .map_err(|_| String::from("failed to bind allocated memory to image"))?;
    }

    log::info!("bound allocated memory to image");
    log::info!("allocated memory");

    let image_view = create_image_view(device, *image_sg, format)?;

    log::info!("created VkImage");

    Ok(crate::VkImage {
        image_view,
        memory: ScopeGuard::into_inner(memory_sg),
        image: ScopeGuard::into_inner(image_sg),
    })
}

pub fn create_image_view(
    device: &ash::Device,
    image: ash::vk::Image,
    format: ash::vk::Format,
) -> Result<ash::vk::ImageView, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let create_info = vk::ImageViewCreateInfo::builder()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .build();

    log::info!("creating image view");

    let iv_sg = {
        let image_view = unsafe {
            device
                .create_image_view(&create_info, None)
                .map_err(|_| String::from("failed to create image view"))?
        };

        guard(image_view, |iv| {
            log::warn!("image view scopeguard");

            unsafe {
                device.destroy_image_view(iv, None);
            }
        })
    };

    log::info!("created image view");

    Ok(ScopeGuard::into_inner(iv_sg))
}
