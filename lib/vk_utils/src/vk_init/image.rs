pub fn create_image(
    device: &ash::Device,
    format: ash::vk::Format,
    format_properties: &ash::vk::FormatProperties,
    memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
    memory_property_flags: ash::vk::MemoryPropertyFlags,
    width: u32,
    height: u32,
) -> Result<crate::VkImage, String> {
    use crate::vk_init::find_memory_type_index;
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    log::info!("creating VkImage");

    let create_info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::TYPE_2D)
        .format(format)
        .extent(vk::Extent3D {
            width,
            height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::STORAGE)
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
        memory_requirement.memory_type_bits,
        memory_properties,
        memory_property_flags,
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

    log::info!("creating image view");

    let image_view = create_image_view(device, format, *image_sg)?;

    log::info!("created image view");

    let sampler_create_info = vk::SamplerCreateInfo::builder()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_BORDER)
        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_BORDER)
        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_BORDER)
        .mip_lod_bias(0.0)
        .max_anisotropy(1.0)
        .compare_op(vk::CompareOp::NEVER)
        .min_lod(0.0)
        .max_lod(0.0)
        .border_color(vk::BorderColor::FLOAT_OPAQUE_WHITE)
        .build();

    log::info!("creating sampler");

    let sampler_sg = {
        let sampler = unsafe {
            device
                .create_sampler(&sampler_create_info, None)
                .map_err(|_| String::from("failed to create sampler"))?
        };

        guard(sampler, |sampler| {
            log::warn!("sampler scopeguard");

            unsafe {
                device.destroy_sampler(sampler, None);
            }
        })
    };

    log::info!("created sampler");

    log::info!("created VkImage");

    Ok(crate::VkImage {
        image_view,
        sampler: ScopeGuard::into_inner(sampler_sg),
        memory: ScopeGuard::into_inner(memory_sg),
        image: ScopeGuard::into_inner(image_sg),
    })
}

pub fn create_image_view(
    device: &ash::Device,
    format: ash::vk::Format,
    image: ash::vk::Image,
) -> Result<ash::vk::ImageView, String> {
    use ash::vk;
    use scopeguard::{guard, ScopeGuard};

    let create_info = vk::ImageViewCreateInfo::builder()
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .components(vk::ComponentMapping {
            r: vk::ComponentSwizzle::R,
            g: vk::ComponentSwizzle::G,
            b: vk::ComponentSwizzle::B,
            a: vk::ComponentSwizzle::A,
        })
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .image(image)
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

pub fn transition_image(
    device: &ash::Device,
    queue: ash::vk::Queue,
    command_buffer: ash::vk::CommandBuffer,
    image: ash::vk::Image,
    old_layout: ash::vk::ImageLayout,
    new_layout: ash::vk::ImageLayout,
) -> Result<(), String> {
    use ash::vk;

    log::info!("trasitioning image");

    let (src_access_mask, dst_access_mask, src_stage, dst_stage) = if old_layout
        == vk::ImageLayout::UNDEFINED
        && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
    {
        (
            vk::AccessFlags::empty(),
            vk::AccessFlags::TRANSFER_WRITE,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
        )
    } else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
        && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
    {
        (
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
        )
    } else {
        (
            vk::AccessFlags::default(),
            vk::AccessFlags::default(),
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::ALL_COMMANDS,
        )
    };
    let barrier = vk::ImageMemoryBarrier::builder()
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(vk::ImageSubresourceRange {
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
            aspect_mask: vk::ImageAspectFlags::COLOR,
        })
        .src_access_mask(src_access_mask)
        .dst_access_mask(dst_access_mask)
        .build();

    let begin_info = vk::CommandBufferBeginInfo::default();

    unsafe {
        device
            .begin_command_buffer(command_buffer, &begin_info)
            .map_err(|_| String::from("failed to begin command buffer"))?;
    }

    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            src_stage,
            dst_stage,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier],
        );

        device
            .end_command_buffer(command_buffer)
            .map_err(|_| String::from("failed to end command buffer"))?;
    }

    let submit_info = vk::SubmitInfo::builder()
        .command_buffers(&[command_buffer])
        .build();
    let fence = crate::vk_init::create_fence(device, None).map_err(|err| {
        log::error!("{}", err);

        String::from("failed to create fence while trasitioning image")
    })?;

    unsafe {
        device
            .queue_submit(queue, &[submit_info], fence)
            .map_err(|_| String::from("failed to submit queue"))?;
        device
            .wait_for_fences(&[fence], true, u64::MAX)
            .map_err(|_| String::from("failed to wait for fences"))?;

        device.destroy_fence(fence, None);
    }

    log::info!("trasitioned image");

    Ok(())
}
