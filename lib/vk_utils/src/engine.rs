pub struct Engine {
    instance: ash::Instance,
    surface: crate::VkSurface,
    physical_device: ash::vk::PhysicalDevice,
    queue_family_indices: crate::QueueFamilyIndices,
    device: ash::Device,
    compute_queue: ash::vk::Queue,
    graphics_queue: ash::vk::Queue,
    present_queue: ash::vk::Queue,
    swapchain: crate::VkSwapchain,

    present_complete: ash::vk::Semaphore,
    render_complete: ash::vk::Semaphore,

    command_pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,

    compute_command_pool: ash::vk::CommandPool,
    compute_command_buffer: ash::vk::CommandBuffer,
    compute_fence: ash::vk::Fence,
    compute_image: crate::VkImage,

    ubo: crate::types::UniformBufferObject,
    ubo_buffer: crate::VkBuffer,

    compute_descriptor_set_layout: ash::vk::DescriptorSetLayout,
    compute_descriptor_pool: ash::vk::DescriptorPool,
    compute_descriptor_set: ash::vk::DescriptorSet,

    compute_pipeline_layout: ash::vk::PipelineLayout,
    compute_pipeline: ash::vk::Pipeline,

    command_pools: Vec<ash::vk::CommandPool>,
    command_buffers: Vec<ash::vk::CommandBuffer>,

    render_pass: ash::vk::RenderPass,
}

impl Engine {
    // constants
    const REQUIRED_INSTANCE_EXTENSIONS: [*const i8; 2] = [
        ash::extensions::ext::DebugUtils::name().as_ptr(),
        ash::extensions::khr::Surface::name().as_ptr(),
    ];
    const REQUIRED_DEVICE_EXTENSIONS: [*const i8; 2] = [
        ash::extensions::khr::Swapchain::name().as_ptr(),
        ash::extensions::khr::Synchronization2::name().as_ptr(),
    ];

    pub fn new(
        app_base: &crate::AppBase,
        window: &crate::window::Window,
    ) -> Result<Engine, String> {
        use crate::vk_init;
        use ash::vk;
        use std::ffi::CStr;

        let validation_layers: Vec<*const i8> =
            vec!["VK_LAYER_KHRONOS_validation\0", "VK_LAYER_LUNARG_monitor\0"]
                .iter()
                .map(|layer| unsafe {
                    CStr::from_bytes_with_nul_unchecked(layer.as_bytes()).as_ptr()
                })
                .collect();

        let instance = vk_init::create_instance(
            app_base,
            window,
            &validation_layers,
            &Self::REQUIRED_INSTANCE_EXTENSIONS,
        )?;

        let surface = vk_init::create_surface(app_base, window, &instance)?;

        let physical_device = vk_init::pick_physical_device(&instance)?;

        let queue_family_indices =
            vk_init::find_queue_families(&instance, &surface, physical_device)?;

        let (device, queues) = vk_init::create_logical_device(
            &instance,
            physical_device,
            &queue_family_indices,
            &Self::REQUIRED_DEVICE_EXTENSIONS,
        )?;

        let memory_properties =
            vk_init::get_physical_device_memory_properties(&instance, physical_device);
        let present_modes = surface.get_physical_device_surface_present_modes(physical_device)?;
        let present_mode = vk_init::choose_swapchain_present_mode(&present_modes);
        let surface_formats = surface.get_physical_device_surface_formats(physical_device)?;
        let surface_format = vk_init::choose_swapchain_format(&surface_formats)?;
        let format_properties = vk_init::get_physical_device_format_properties(
            &instance,
            physical_device,
            surface_format.format,
        );
        let surface_capabilities =
            surface.get_physical_device_surface_capabilities(physical_device)?;
        let swapchain = vk_init::create_swapchain(
            &instance,
            physical_device,
            &surface,
            &device,
            crate::constants::WIDTH,
            crate::constants::HEIGHT,
            &surface_capabilities,
            present_mode,
            &surface_format,
        )?;

        let present_complete = vk_init::create_semaphore(&device, "present complete")?;
        let render_complete = vk_init::create_semaphore(&device, "render complete")?;

        let command_pool = vk_init::create_command_pool(
            &device,
            queue_family_indices.graphics_family_index,
            "graphics family",
        )?;
        let command_buffer = vk_init::create_command_buffer(&device, &command_pool)?;

        let compute_command_pool = vk_init::create_command_pool(
            &device,
            queue_family_indices.compute_family_index,
            "compute family",
        )?;
        let compute_command_buffer =
            vk_init::create_command_buffer(&device, &compute_command_pool)?;
        let compute_fence = vk_init::create_fence(&device, Some(vk::FenceCreateFlags::SIGNALED))?;
        let compute_image = vk_init::create_image(
            &device,
            surface_format.format,
            &format_properties,
            &memory_properties,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            1024,
            1024,
        )?;

        vk_init::transition_image(
            &device,
            queues.graphics,
            command_buffer,
            compute_image.image,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::GENERAL,
        )?;

        let (ubo_buffer, ubo) = Self::create_ubo(&device, &memory_properties)?;

        let (compute_descriptor_set_layout, compute_descriptor_pool, compute_descriptor_set) =
            Self::create_compute_descriptor(&device)?;
        Self::update_compute_descriptor_sets(
            &device,
            &compute_image,
            &ubo_buffer,
            &ubo,
            compute_descriptor_set,
        );

        let compute_pipeline_layout =
            vk_init::create_pipeline_layout(&device, compute_descriptor_set_layout)?;

        let mut shader = vk_init::create_shader_module(
            &device,
            "shaders/spv/raytrace.comp.spv",
            vk::ShaderStageFlags::COMPUTE,
        )?;
        let compute_pipeline =
            vk_init::create_compute_pipeline(&device, compute_pipeline_layout, &shader)?;
        crate::VkShaderModule::cleanup(&device, &mut shader);

        let command_pools = vk_init::create_command_pools(
            &device,
            queue_family_indices.graphics_family_index,
            swapchain.images.len(),
        )?;
        let command_buffers = vk_init::create_command_buffers(&device, &command_pools)?;

        let render_pass = vk_init::create_render_pass(&device, surface_format.format)?;

        Ok(Self {
            instance,
            surface,
            physical_device,
            queue_family_indices,
            device,
            compute_queue: queues.compute,
            graphics_queue: queues.graphics,
            present_queue: queues.present,
            swapchain,
            present_complete,
            render_complete,
            command_pool,
            command_buffer,
            compute_command_pool,
            compute_command_buffer,
            compute_fence,
            compute_image,
            ubo_buffer,
            ubo,
            compute_descriptor_set_layout,
            compute_descriptor_pool,
            compute_descriptor_set,
            compute_pipeline_layout,
            compute_pipeline,
            command_pools,
            command_buffers,
            render_pass,
        })
    }

    fn create_ubo(
        device: &ash::Device,
        memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
    ) -> Result<(crate::VkBuffer, crate::types::UniformBufferObject), String> {
        use crate::{types::UniformBufferObject, vk_init};
        use ash::vk;
        use std::mem::size_of_val;

        let ubo = UniformBufferObject {
            image_width: 1024.0,
            image_height: 1024.0,
            viewport_width: 2.0,
            viewport_height: 2.0,
            focal_length: 1.0,
        };
        let ubo_buffer = vk_init::create_buffer(
            device,
            memory_properties,
            size_of_val(&ubo) as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;
        let mut ubo_ptr = vk_init::map_buffer(device, &ubo_buffer)? as *mut UniformBufferObject;
        ubo_ptr = vk_init::copy_to_mapped_memory::<UniformBufferObject>(ubo_ptr, ubo);

        Ok((ubo_buffer, unsafe { *ubo_ptr }))
    }

    fn create_compute_descriptor(
        device: &ash::Device,
    ) -> Result<
        (
            ash::vk::DescriptorSetLayout,
            ash::vk::DescriptorPool,
            ash::vk::DescriptorSet,
        ),
        String,
    > {
        use crate::vk_init;
        use ash::vk;

        let compute_layout_set_bindings = vec![
            vk::DescriptorSetLayoutBinding::builder()
                .binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .build(),
            vk::DescriptorSetLayoutBinding::builder()
                .binding(1)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .build(),
        ];
        let compute_descriptor_set_layout =
            vk_init::create_descriptor_set_layout(device, &compute_layout_set_bindings)?;

        let compute_pool_sizes = vec![
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(3)
                .build(),
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(3)
                .build(),
        ];
        let compute_descriptor_pool =
            vk_init::create_descriptor_pool(device, &compute_pool_sizes, 4)?;
        let compute_descriptor_set = vk_init::create_descriptor_set(
            device,
            compute_descriptor_set_layout,
            compute_descriptor_pool,
        )?;

        Ok((
            compute_descriptor_set_layout,
            compute_descriptor_pool,
            compute_descriptor_set,
        ))
    }

    fn update_compute_descriptor_sets(
        device: &ash::Device,
        image: &crate::VkImage,
        buffer: &crate::VkBuffer,
        ubo: &crate::types::UniformBufferObject,
        compute_descriptor_sets: ash::vk::DescriptorSet,
    ) {
        use crate::vk_init;
        use ash::vk;
        use std::mem::size_of_val;

        let image_info = vk::DescriptorImageInfo::builder()
            .sampler(image.sampler)
            .image_view(image.image_view)
            .image_layout(vk::ImageLayout::GENERAL)
            .build();
        let buffer_info = vk::DescriptorBufferInfo::builder()
            .buffer(buffer.buffer)
            .range(size_of_val(ubo) as u64)
            .build();
        let compute_descriptor_writes = vec![
            vk::WriteDescriptorSet::builder()
                .dst_set(compute_descriptor_sets)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .image_info(&[image_info])
                .build(),
            vk::WriteDescriptorSet::builder()
                .dst_set(compute_descriptor_sets)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&[buffer_info])
                .build(),
        ];

        vk_init::update_descriptor_sets(device, &compute_descriptor_writes);
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        log::info!("performing cleanup for Engine");

        unsafe {
            self.device.destroy_render_pass(self.render_pass, None);
            for (index, &buffer) in self.command_buffers.iter().enumerate() {
                self.device
                    .free_command_buffers(self.command_pools[index], &[buffer]);
            }
            for &pool in self.command_pools.iter() {
                self.device.destroy_command_pool(pool, None);
            }

            self.device.destroy_pipeline(self.compute_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.compute_pipeline_layout, None);
            self.device
                .destroy_descriptor_pool(self.compute_descriptor_pool, None);
            self.device
                .destroy_descriptor_set_layout(self.compute_descriptor_set_layout, None);
        }
        crate::VkBuffer::unmap_buffer(&self.device, &mut self.ubo_buffer);
        crate::VkBuffer::cleanup(&self.device, &mut self.ubo_buffer);
        crate::VkImage::cleanup(&self.device, &mut self.compute_image);
        unsafe {
            self.device.destroy_fence(self.compute_fence, None);
            self.device
                .free_command_buffers(self.compute_command_pool, &[self.compute_command_buffer]);
            self.device
                .destroy_command_pool(self.compute_command_pool, None);
            self.device
                .free_command_buffers(self.command_pool, &[self.command_buffer]);
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_semaphore(self.render_complete, None);
            self.device.destroy_semaphore(self.present_complete, None);
        }
        crate::VkSwapchain::cleanup(&self.device, &mut self.swapchain);
        unsafe {
            self.device.destroy_device(None);
        }
        crate::VkSurface::cleanup(&mut self.surface);
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
