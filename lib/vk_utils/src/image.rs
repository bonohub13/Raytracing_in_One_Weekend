pub struct Image {
    pub image: ash::vk::Image,
    extent: ash::vk::Extent2D,
    format: ash::vk::Format,
    image_layout: ash::vk::ImageLayout,
}

impl Image {
    pub fn new(
        device: &crate::Device,
        extent: ash::vk::Extent2D,
        format: ash::vk::Format,
        tiling: Option<ash::vk::ImageTiling>,
        usage_flags: Option<ash::vk::ImageUsageFlags>,
    ) -> Result<Self, String> {
        use ash::vk;

        let image_tiling = match tiling {
            Some(image_tiling) => image_tiling,
            None => vk::ImageTiling::OPTIMAL,
        };
        let image_usage_flags = match usage_flags {
            Some(image_usage_flags) => image_usage_flags,
            None => vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        };
        let image_layout = vk::ImageLayout::UNDEFINED;

        let image_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .extent(vk::Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(image_tiling)
            .initial_layout(image_layout)
            .usage(image_usage_flags)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .samples(vk::SampleCountFlags::TYPE_1)
            .flags(vk::ImageCreateFlags::empty())
            .build();

        let image = device.create_image(&image_info, None)?;

        Ok(Self {
            image,
            image_layout,
            extent,
            format,
        })
    }

    pub fn extent(&self) -> ash::vk::Extent2D {
        self.extent
    }

    pub fn format(&self) -> ash::vk::Format {
        self.format
    }

    pub fn allocate_memory(
        &self,
        instance: &crate::Instance,
        device: &crate::Device,
        properties: ash::vk::MemoryPropertyFlags,
    ) -> Result<crate::DeviceMemory, String> {
        use ash::vk;

        log::info!("allocating memory");

        let requirements = self.get_memory_requirements(device);
        let memory = crate::DeviceMemory::new(
            instance,
            device,
            requirements.size,
            requirements.memory_type_bits,
            vk::MemoryAllocateFlags::empty(),
            properties,
        )?;

        device.bind_image_memory(self.image, memory.device_memory, 0)?;

        log::info!("allocated memory");

        Ok(memory)
    }

    pub fn get_memory_requirements(&self, device: &crate::Device) -> ash::vk::MemoryRequirements {
        device.get_image_memory_requirements(self.image)
    }

    pub fn transition_image_layout(
        &mut self,
        device: &crate::Device,
        pool: &crate::CommandPool,
        new_layout: ash::vk::ImageLayout,
    ) -> Result<(), String> {
        use crate::utils::SingleTimeCommands;

        SingleTimeCommands::submit(device, pool, |command_buffer| {
            use ash::vk;

            let (src_access_flags, dst_access_flags, source_stage, destination_stage) =
                if self.image_layout == vk::ImageLayout::UNDEFINED
                    && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
                {
                    Ok((
                        vk::AccessFlags::empty(),
                        vk::AccessFlags::TRANSFER_WRITE,
                        vk::PipelineStageFlags::TOP_OF_PIPE,
                        vk::PipelineStageFlags::TRANSFER,
                    ))
                } else if self.image_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
                    && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                {
                    Ok((
                        vk::AccessFlags::TRANSFER_WRITE,
                        vk::AccessFlags::SHADER_READ,
                        vk::PipelineStageFlags::TRANSFER,
                        vk::PipelineStageFlags::FRAGMENT_SHADER,
                    ))
                } else if self.image_layout == vk::ImageLayout::UNDEFINED
                    && new_layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
                {
                    Ok((
                        vk::AccessFlags::empty(),
                        vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                        vk::PipelineStageFlags::TOP_OF_PIPE,
                        vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
                    ))
                } else {
                    Err(String::from("unsupported layout transition"))
                }?;

            let barrier = vk::ImageMemoryBarrier::builder()
                .old_layout(self.image_layout)
                .new_layout(new_layout)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(self.image)
                .subresource_range(vk::ImageSubresourceRange {
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                    aspect_mask: if new_layout == vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
                    {
                        if crate::DepthBuffer::has_stencil_component(self.format) {
                            vk::ImageAspectFlags::STENCIL | vk::ImageAspectFlags::DEPTH
                        } else {
                            vk::ImageAspectFlags::DEPTH
                        }
                    } else {
                        vk::ImageAspectFlags::COLOR
                    },
                })
                .src_access_mask(src_access_flags)
                .dst_access_mask(dst_access_flags)
                .build();

            device.cmd_pipeline_barrier(
                command_buffer,
                source_stage,
                destination_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[barrier],
            );

            Ok(())
        })?;

        self.image_layout = new_layout;

        Ok(())
    }

    pub fn copy_from(
        &self,
        device: &crate::Device,
        pool: &crate::CommandPool,
        buffer: &crate::Buffer,
    ) -> Result<(), String> {
        crate::utils::SingleTimeCommands::submit(device, pool, |command_buffer| {
            use ash::vk;

            let region = vk::BufferImageCopy::builder()
                .buffer_offset(0)
                .buffer_row_length(0)
                .buffer_image_height(0)
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                    ..Default::default()
                })
                .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
                .image_extent(vk::Extent3D {
                    width: self.extent.width,
                    height: self.extent.height,
                    depth: 1,
                })
                .build();

            device.cmd_copy_buffer_to_image(
                command_buffer,
                buffer.buffer,
                self.image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );

            Ok(())
        })
    }

    pub fn cleanup(device: &crate::Device, image: &mut Self) {
        log::info!("performing cleanup for Image");

        device.destroy_image(image.image, None);
    }
}
