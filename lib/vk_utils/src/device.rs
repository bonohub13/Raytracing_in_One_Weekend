pub struct QueueFamilyIndices {
    pub graphics_family: u32,
    pub compute_family: u32,
    pub present_family: u32,
}

pub struct Device {
    physical_device: ash::vk::PhysicalDevice,
    pub device: ash::Device,
    queue_families: QueueFamilyIndices,
    graphics_queue: ash::vk::Queue,
    compute_queue: ash::vk::Queue,
    present_queue: ash::vk::Queue,
}

impl Device {
    pub fn new(
        instance: &crate::Instance,
        physical_device: ash::vk::PhysicalDevice,
        surface: &crate::Surface,
        required_extensions: &Vec<*const std::os::raw::c_char>,
        device_features: ash::vk::PhysicalDeviceFeatures,
        next_device_features: Option<*const std::os::raw::c_void>,
    ) -> Result<Self, String> {
        use ash::vk;
        use scopeguard::{guard, ScopeGuard};

        Self::check_required_extensions(instance, physical_device, required_extensions)?;

        let queue_families = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics_family_index = find_queue(
            &queue_families,
            "graphics",
            vk::QueueFlags::GRAPHICS,
            vk::QueueFlags::empty(),
        )?;
        let compute_family_index = find_queue(
            &queue_families,
            "compute",
            vk::QueueFlags::COMPUTE,
            vk::QueueFlags::GRAPHICS,
        )?;
        let present_family_index =
            match queue_families
                .iter()
                .enumerate()
                .find(|&(index, queue_family)| {
                    let present_support = surface
                        .get_physical_device_surface_support(physical_device, index as u32)
                        .unwrap_or(false);

                    queue_family.queue_count > 0 && present_support
                }) {
                Some((index, _)) => {
                    log::info!("found present family index");

                    index as u32
                }
                None => return Err(String::from("found no presentation queue")),
            };

        let queue_family_indices = QueueFamilyIndices {
            graphics_family: graphics_family_index,
            compute_family: compute_family_index,
            present_family: present_family_index,
        };

        let queue_priority = 1.0f32;
        let queue_create_infos = vec![
            // graphics queue
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(graphics_family_index)
                .queue_priorities(&[queue_priority])
                .build(),
            // compute queue
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(compute_family_index)
                .queue_priorities(&[queue_priority])
                .build(),
            // present queue
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(present_family_index)
                .queue_priorities(&[queue_priority])
                .build(),
        ];

        let mut create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_layer_names(&instance.validation_layers())
            .enabled_extension_names(&required_extensions)
            .build();

        if let Some(device_features) = next_device_features {
            create_info.p_next = device_features;
        }

        let device_sg = {
            let device = instance.create_device(physical_device, &create_info, None)?;

            guard(device, |device| {
                log::warn!("device scopeguard");

                unsafe {
                    device.destroy_device(None);
                }
            })
        };

        let graphics_queue = unsafe { device_sg.get_device_queue(graphics_family_index, 0) };
        let compute_queue = unsafe { device_sg.get_device_queue(compute_family_index, 0) };
        let present_queue = unsafe { device_sg.get_device_queue(present_family_index, 0) };

        Ok(Self {
            physical_device,
            device: ScopeGuard::into_inner(device_sg),
            queue_families: queue_family_indices,
            graphics_queue,
            compute_queue,
            present_queue,
        })
    }

    pub fn wait_idle(&self) -> Result<(), String> {
        unsafe {
            self.device
                .device_wait_idle()
                .map_err(|_| String::from("failed for device to wait idle"))
        }
    }

    pub fn physical_device(&self) -> ash::vk::PhysicalDevice {
        self.physical_device
    }

    pub fn graphics_queue_index(&self) -> u32 {
        self.queue_families.graphics_family
    }

    pub fn compute_queue_index(&self) -> u32 {
        self.queue_families.compute_family
    }

    pub fn present_queue_index(&self) -> u32 {
        self.queue_families.present_family
    }

    pub fn graphics_queue(&self) -> ash::vk::Queue {
        self.graphics_queue
    }

    pub fn compute_queue(&self) -> ash::vk::Queue {
        self.compute_queue
    }

    pub fn present_queue(&self) -> ash::vk::Queue {
        self.present_queue
    }

    pub fn get_image_memory_requirements(
        &self,
        image: ash::vk::Image,
    ) -> ash::vk::MemoryRequirements {
        unsafe { self.device.get_image_memory_requirements(image) }
    }

    pub fn get_buffer_memory_requirements(
        &self,
        buffer: ash::vk::Buffer,
    ) -> ash::vk::MemoryRequirements {
        unsafe { self.device.get_buffer_memory_requirements(buffer) }
    }

    pub fn get_buffer_device_address(
        &self,
        info: &ash::vk::BufferDeviceAddressInfo,
    ) -> ash::vk::DeviceAddress {
        unsafe { self.device.get_buffer_device_address(info) }
    }

    pub fn queue_submit(
        &self,
        queue: ash::vk::Queue,
        submit_infos: &[ash::vk::SubmitInfo],
        fence: ash::vk::Fence,
    ) -> Result<(), String> {
        unsafe {
            self.device
                .queue_submit(queue, submit_infos, fence)
                .map_err(|_| String::from("failed to submit queue"))
        }
    }

    pub fn queue_wait_idle(&self, queue: ash::vk::Queue) -> Result<(), String> {
        unsafe {
            self.device
                .queue_wait_idle(queue)
                .map_err(|_| String::from("queue failed to wait idle"))
        }
    }

    pub fn reset_fences(&self, fences: &[ash::vk::Fence]) -> Result<(), String> {
        unsafe {
            self.device
                .reset_fences(fences)
                .map_err(|_| String::from("failed to reset fences"))
        }
    }

    pub fn wait_for_fences(
        &self,
        fences: &[ash::vk::Fence],
        wait_all: bool,
        timeout: u64,
    ) -> Result<(), String> {
        unsafe {
            self.device
                .wait_for_fences(fences, wait_all, timeout)
                .map_err(|_| String::from("failed to wait for fences"))
        }
    }

    pub fn create_command_pool(
        &self,
        create_info: &ash::vk::CommandPoolCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::CommandPool, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating command pool");

        let command_pool_sg = {
            let command_pool = unsafe {
                self.device
                    .create_command_pool(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed create command pool"))?
            };

            guard(command_pool, |pool| {
                log::warn!("command pool scopeguard");

                self.destroy_command_pool(pool, None);
            })
        };

        log::info!("created command pool");

        Ok(ScopeGuard::into_inner(command_pool_sg))
    }

    pub fn create_image_view(
        &self,
        create_info: &ash::vk::ImageViewCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::ImageView, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating image view");

        let image_view_sg = {
            let image_view = unsafe {
                self.device
                    .create_image_view(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create image view"))?
            };

            guard(image_view, |iv| {
                log::warn!("image view scopeguard");

                self.destroy_image_view(iv, None);
            })
        };

        log::info!("created image view");

        Ok(ScopeGuard::into_inner(image_view_sg))
    }

    pub fn create_image(
        &self,
        create_info: &ash::vk::ImageCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::Image, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating image");

        let image_sg = {
            let image = unsafe {
                self.device
                    .create_image(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create image"))?
            };

            guard(image, |image| {
                log::warn!("image scopeguard");

                self.destroy_image(image, None);
            })
        };

        log::info!("created image");

        Ok(ScopeGuard::into_inner(image_sg))
    }

    pub fn create_buffer(
        &self,
        create_info: &ash::vk::BufferCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::Buffer, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating buffer");

        let buffer_sg = {
            let buffer = unsafe {
                self.device
                    .create_buffer(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create buffer"))?
            };

            guard(buffer, |buffer| {
                log::warn!("buffer scopeguard");

                self.destroy_buffer(buffer, None);
            })
        };

        log::info!("created buffer");

        Ok(ScopeGuard::into_inner(buffer_sg))
    }

    pub fn create_semaphore(
        &self,
        create_info: &ash::vk::SemaphoreCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::Semaphore, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating semaphore");

        let semaphore_sg = {
            let semaphore = unsafe {
                self.device
                    .create_semaphore(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create semaphore"))?
            };

            guard(semaphore, |semaphore| {
                log::warn!("semaphore scopeguard");

                self.destroy_semaphore(semaphore, None);
            })
        };

        log::info!("created semaphore");

        Ok(ScopeGuard::into_inner(semaphore_sg))
    }

    pub fn create_fence(
        &self,
        create_info: &ash::vk::FenceCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::Fence, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating fence");

        let fence_sg = {
            let fence = unsafe {
                self.device
                    .create_fence(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create fence"))?
            };

            guard(fence, |fence| {
                log::info!("fence scopeguard");

                self.destroy_fence(fence, None);
            })
        };

        log::info!("created fence");

        Ok(ScopeGuard::into_inner(fence_sg))
    }

    pub fn create_descriptor_pool(
        &self,
        create_info: &ash::vk::DescriptorPoolCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::DescriptorPool, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating descriptor pool");

        let descriptor_pool_sg = {
            let descriptor_pool = unsafe {
                self.device
                    .create_descriptor_pool(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create descriptor pool"))?
            };

            guard(descriptor_pool, |pool| {
                log::info!("descriptor pool scopeguard");

                self.destroy_descriptor_pool(pool, None);
            })
        };

        log::info!("created descriptor pool");

        Ok(ScopeGuard::into_inner(descriptor_pool_sg))
    }

    pub fn create_descriptor_set_layout(
        &self,
        create_info: &ash::vk::DescriptorSetLayoutCreateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::DescriptorSetLayout, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating descriptor set layout");

        let layout_sg = {
            let layout = unsafe {
                self.device
                    .create_descriptor_set_layout(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to create descriptor set layout"))?
            };

            guard(layout, |layout| {
                log::warn!("descriptor set layout scopeguard");

                self.destroy_descriptor_set_layout(layout, None);
            })
        };

        log::info!("creating descriptor set layout");

        Ok(ScopeGuard::into_inner(layout_sg))
    }

    pub fn allocate_command_buffer(
        &self,
        command_pool: &crate::CommandPool,
        create_info: &ash::vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<ash::vk::CommandBuffer>, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("creating command buffers");

        let command_buffers_sg = {
            let command_buffers = unsafe {
                self.device
                    .allocate_command_buffers(create_info)
                    .map_err(|_| String::from("failed to allocate command buffers"))?
            };

            guard(command_buffers, |buffers| {
                log::warn!("command buffers scopeguard");

                self.free_command_buffers(command_pool, &buffers);
            })
        };

        log::info!("created command buffers");

        Ok(ScopeGuard::into_inner(command_buffers_sg))
    }

    pub fn allocate_memory(
        &self,
        create_info: &ash::vk::MemoryAllocateInfo,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) -> Result<ash::vk::DeviceMemory, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("allocating device memory");

        let memory_sg = {
            let memory = unsafe {
                self.device
                    .allocate_memory(create_info, allocation_callbacks)
                    .map_err(|_| String::from("failed to allocate device memory"))?
            };

            guard(memory, |mem| {
                log::warn!("device memory scopeguard");

                self.free_memory(mem, None);
            })
        };

        log::info!("allocated device memory");

        Ok(ScopeGuard::into_inner(memory_sg))
    }

    pub fn allocate_descriptor_sets(
        &self,
        pool: ash::vk::DescriptorPool,
        create_info: &ash::vk::DescriptorSetAllocateInfo,
    ) -> Result<Vec<ash::vk::DescriptorSet>, String> {
        use scopeguard::{guard, ScopeGuard};

        log::info!("allocating descriptor sets");

        let descriptor_sets_sg = {
            let descriptor_sets = unsafe {
                self.device
                    .allocate_descriptor_sets(create_info)
                    .map_err(|_| String::from("failed to allocate descriptor sets"))?
            };

            guard(descriptor_sets, |sets| {
                log::warn!("descriptor sets scopeguard");

                match self.free_descriptor_sets(pool, &sets) {
                    Ok(_) => (),
                    Err(err) => {
                        log::error!(
                            "failed to free descriptor sets in descriptor sets scopeguard: {}",
                            err
                        )
                    }
                }
            })
        };

        log::info!("allocated descriptor sets");

        Ok(ScopeGuard::into_inner(descriptor_sets_sg))
    }

    pub fn bind_image_memory(
        &self,
        image: ash::vk::Image,
        device_memory: ash::vk::DeviceMemory,
        offset: ash::vk::DeviceSize,
    ) -> Result<(), String> {
        unsafe {
            self.device
                .bind_image_memory(image, device_memory, offset)
                .map_err(|_| String::from("failed to bind image memory"))
        }
    }

    pub fn bind_buffer_memory(
        &self,
        buffer: ash::vk::Buffer,
        device_memory: ash::vk::DeviceMemory,
        offset: ash::vk::DeviceSize,
    ) -> Result<(), String> {
        unsafe {
            self.device
                .bind_buffer_memory(buffer, device_memory, offset)
                .map_err(|_| String::from("failed to bind buffer memory"))
        }
    }

    pub fn begin_command_buffer(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        begin_info: &ash::vk::CommandBufferBeginInfo,
    ) -> Result<(), String> {
        unsafe {
            self.device
                .begin_command_buffer(command_buffer, begin_info)
                .map_err(|_| String::from("failed to begin command buffer"))
        }
    }

    pub fn end_command_buffer(&self, command_buffer: ash::vk::CommandBuffer) -> Result<(), String> {
        unsafe {
            self.device
                .end_command_buffer(command_buffer)
                .map_err(|_| String::from("failed to end command buffer"))
        }
    }

    pub fn cmd_pipeline_barrier(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        src_stage_mask: ash::vk::PipelineStageFlags,
        dst_stage_mask: ash::vk::PipelineStageFlags,
        dependency_flags: ash::vk::DependencyFlags,
        memory_barriers: &[ash::vk::MemoryBarrier],
        buffer_memory_barriers: &[ash::vk::BufferMemoryBarrier],
        image_memory_barriers: &[ash::vk::ImageMemoryBarrier],
    ) {
        unsafe {
            self.device.cmd_pipeline_barrier(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barriers,
                buffer_memory_barriers,
                image_memory_barriers,
            );
        }
    }

    pub fn cmd_copy_buffer(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        src_buffer: ash::vk::Buffer,
        dst_buffer: ash::vk::Buffer,
        regions: &[ash::vk::BufferCopy],
    ) {
        unsafe {
            self.device
                .cmd_copy_buffer(command_buffer, src_buffer, dst_buffer, regions);
        }
    }

    pub fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        src_buffer: ash::vk::Buffer,
        dst_image: ash::vk::Image,
        dst_image_layout: ash::vk::ImageLayout,
        regions: &[ash::vk::BufferImageCopy],
    ) {
        unsafe {
            self.device.cmd_copy_buffer_to_image(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions,
            );
        }
    }

    pub fn destroy_command_pool(
        &self,
        pool: ash::vk::CommandPool,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device.destroy_command_pool(pool, allocation_callbacks);
        }
    }

    pub fn destroy_image_view(
        &self,
        image_view: ash::vk::ImageView,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device
                .destroy_image_view(image_view, allocation_callbacks);
        }
    }

    pub fn destroy_image(
        &self,
        image: ash::vk::Image,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device.destroy_image(image, allocation_callbacks);
        }
    }

    pub fn destroy_buffer(
        &self,
        buffer: ash::vk::Buffer,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device.destroy_buffer(buffer, allocation_callbacks);
        }
    }

    pub fn destroy_semaphore(
        &self,
        semaphore: ash::vk::Semaphore,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device
                .destroy_semaphore(semaphore, allocation_callbacks);
        }
    }

    pub fn destroy_fence(
        &self,
        fence: ash::vk::Fence,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device.destroy_fence(fence, allocation_callbacks);
        }
    }

    pub fn destroy_descriptor_pool(
        &self,
        descriptor_pool: ash::vk::DescriptorPool,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device
                .destroy_descriptor_pool(descriptor_pool, allocation_callbacks);
        }
    }

    pub fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: ash::vk::DescriptorSetLayout,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device
                .destroy_descriptor_set_layout(descriptor_set_layout, allocation_callbacks);
        }
    }

    pub fn free_command_buffers(
        &self,
        command_pool: &crate::CommandPool,
        command_buffers: &Vec<ash::vk::CommandBuffer>,
    ) {
        unsafe {
            self.device
                .free_command_buffers(command_pool.command_pool, command_buffers);
        }
    }

    pub fn free_memory(
        &self,
        memory: ash::vk::DeviceMemory,
        allocation_callbacks: Option<&ash::vk::AllocationCallbacks>,
    ) {
        unsafe {
            self.device.free_memory(memory, allocation_callbacks);
        }
    }

    pub fn free_descriptor_sets(
        &self,
        pool: ash::vk::DescriptorPool,
        descriptor_sets: &[ash::vk::DescriptorSet],
    ) -> Result<(), String> {
        unsafe {
            self.device
                .free_descriptor_sets(pool, descriptor_sets)
                .map_err(|_| String::from("failed to free descriptor sets"))
        }
    }

    pub fn cleanup(device: &mut Self) {
        log::info!("performing cleanup for Device");

        unsafe {
            device.device.destroy_device(None);
        }
    }

    fn check_required_extensions(
        instance: &crate::Instance,
        physical_device: ash::vk::PhysicalDevice,
        required_extensions: &Vec<*const std::os::raw::c_char>,
    ) -> Result<(), String> {
        use crate::utils::vk_to_string;

        let available_extensions =
            instance.enumerate_device_extension_properties(physical_device)?;
        let required: Vec<[i8; 256]> = available_extensions
            .iter()
            .filter(|&extension| !required_extensions.contains(&extension.extension_name.as_ptr()))
            .map(|extension_name| extension_name.extension_name)
            .collect();

        if !required.is_empty() {
            let mut extensions = String::from("");

            for extension in required.iter() {
                let extension_name = vk_to_string(extension)?;

                if extensions != "" {
                    extensions = format!("{}, {}", extensions, extension_name);
                } else {
                    extensions = extension_name;
                }
            }

            Err(format!("missing required extensions: {}", extensions))
        } else {
            Ok(())
        }
    }
}

fn find_queue(
    queue_families: &Vec<ash::vk::QueueFamilyProperties>,
    name: &str,
    required_flags: ash::vk::QueueFlags,
    excluded_flags: ash::vk::QueueFlags,
) -> Result<u32, String> {
    log::info!("finding queue: {}", name);

    let family = queue_families
        .iter()
        .enumerate()
        .find(|&(_, queue_family)| {
            queue_family.queue_count > 0
                && queue_family.queue_flags.contains(required_flags)
                && !queue_family.queue_flags.contains(excluded_flags)
        });

    match family {
        Some((index, _)) => {
            log::info!("found matching {} queue", name);

            Ok(index as u32)
        }
        None => Err(format!("found no matching {} queue", name)),
    }
}
