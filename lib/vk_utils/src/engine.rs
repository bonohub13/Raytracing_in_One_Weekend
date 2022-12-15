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
    framebuffers: Vec<ash::vk::Framebuffer>,

    vertex_buffer: crate::VkBuffer,
    index_buffer: crate::VkBuffer,

    descriptor_set_layout: ash::vk::DescriptorSetLayout,
    descriptor_pool: ash::vk::DescriptorPool,
    descriptor_set: ash::vk::DescriptorSet,

    pipeline_layout: ash::vk::PipelineLayout,
    graphics_pipeline: ash::vk::Pipeline,
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
        use std::mem::size_of_val;

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

        let framebuffers = vk_init::create_framebuffers(
            &device,
            render_pass,
            &swapchain.image_views,
            crate::constants::WIDTH,
            crate::constants::HEIGHT,
        )?;

        let max_size = size_of_val(&crate::constants::RAINBOW_RECTANGLE)
            .max(size_of_val(&crate::constants::RAINBOW_RECTANGLE_INDICES))
            as u64;
        let mut staging_buffer = vk_init::create_buffer(
            &device,
            &memory_properties,
            max_size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;
        let staging_buffer_ptr = vk_init::map_buffer(&device, &staging_buffer)?;
        let vertex_buffer = Self::create_vertex_buffer(
            &device,
            &memory_properties,
            &crate::constants::RAINBOW_RECTANGLE,
            staging_buffer_ptr,
        )?;
        vk_init::copy_buffer_to(
            &device,
            queues.graphics,
            command_buffer,
            &staging_buffer,
            &vertex_buffer,
            size_of_val(&crate::constants::RAINBOW_RECTANGLE) as u64,
        )?;
        let index_buffer = Self::create_index_buffer(
            &device,
            &memory_properties,
            &crate::constants::RAINBOW_RECTANGLE_INDICES,
            staging_buffer_ptr,
        )?;
        vk_init::copy_buffer_to(
            &device,
            queues.graphics,
            command_buffer,
            &staging_buffer,
            &index_buffer,
            size_of_val(&crate::constants::RAINBOW_RECTANGLE_INDICES) as u64,
        )?;
        crate::VkBuffer::unmap_buffer(&device, &mut staging_buffer);
        crate::VkBuffer::cleanup(&device, &mut staging_buffer);

        let (descriptor_set_layout, descriptor_pool, descriptor_set) =
            Self::create_graphics_descriptor(&device)?;
        Self::update_graphics_descriptor_sets(&device, &compute_image, descriptor_set);

        let pipeline_layout = vk_init::create_pipeline_layout(&device, descriptor_set_layout)?;
        let graphics_pipeline = Self::create_graphics_pipeline(
            &device,
            render_pass,
            pipeline_layout,
            swapchain.extent,
        )?;

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
            framebuffers,
            vertex_buffer,
            index_buffer,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            pipeline_layout,
            graphics_pipeline,
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

    fn create_vertex_buffer(
        device: &ash::Device,
        memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
        vertex: &[f32],
        staging_buffer_ptr: *mut std::os::raw::c_void,
    ) -> Result<crate::VkBuffer, String> {
        use crate::vk_init;
        use ash::vk;
        use std::mem::size_of_val;

        log::info!("creating vertex buffer");

        let vertex_buffer = vk_init::create_buffer(
            device,
            memory_properties,
            size_of_val(&crate::constants::RAINBOW_RECTANGLE) as u64,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;
        unsafe {
            (staging_buffer_ptr as *mut f32)
                .copy_from_nonoverlapping(vertex.as_ptr(), vertex.len());
        }

        log::info!("created vertex buffer");

        Ok(vertex_buffer)
    }

    fn create_index_buffer(
        device: &ash::Device,
        memory_properties: &ash::vk::PhysicalDeviceMemoryProperties,
        index: &[u32],
        staging_buffer_ptr: *mut std::os::raw::c_void,
    ) -> Result<crate::VkBuffer, String> {
        use crate::vk_init;
        use ash::vk;
        use std::mem::size_of_val;

        log::info!("creating index buffer");

        let index_buffer = vk_init::create_buffer(
            device,
            memory_properties,
            size_of_val(&crate::constants::RAINBOW_RECTANGLE_INDICES) as u64,
            vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;
        unsafe {
            (staging_buffer_ptr as *mut u32).copy_from_nonoverlapping(index.as_ptr(), index.len());
        }

        log::info!("created index buffer");

        Ok(index_buffer)
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

    fn create_graphics_descriptor(
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

        log::info!("creating graphics descriptor");

        let layout_set_bindings = vec![vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .build()];

        let descriptor_set_layout =
            vk_init::create_descriptor_set_layout(device, &layout_set_bindings).map_err(|err| {
                log::error!("{}", err);

                String::from("failed to create descriptor set layout for graphics descriptor")
            })?;

        let pool_sizes = vec![vk::DescriptorPoolSize::builder()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(3)
            .build()];

        let descriptor_pool =
            vk_init::create_descriptor_pool(device, &pool_sizes, 4).map_err(|err| {
                log::error!("{}", err);

                String::from("failed to create descriptor pool for graphics descriptor")
            })?;

        let descriptor_set =
            vk_init::create_descriptor_set(device, descriptor_set_layout, descriptor_pool)
                .map_err(|err| {
                    log::error!("{}", err);

                    String::from("failed to create descriptor set for graphics descriptor")
                })?;

        log::info!("created graphics descriptor");

        Ok((descriptor_set_layout, descriptor_pool, descriptor_set))
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

    fn update_graphics_descriptor_sets(
        device: &ash::Device,
        image: &crate::VkImage,
        graphics_descriptor_set: ash::vk::DescriptorSet,
    ) {
        use crate::vk_init;
        use ash::vk;

        let image_info = vk::DescriptorImageInfo::builder()
            .sampler(image.sampler)
            .image_view(image.image_view)
            .image_layout(vk::ImageLayout::GENERAL)
            .build();
        let descriptor_writes = vec![vk::WriteDescriptorSet::builder()
            .dst_set(graphics_descriptor_set)
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&[image_info])
            .build()];

        vk_init::update_descriptor_sets(device, &descriptor_writes);
    }

    fn create_graphics_pipeline(
        device: &ash::Device,
        render_pass: ash::vk::RenderPass,
        pipeline_layout: ash::vk::PipelineLayout,
        swapchain_extent: ash::vk::Extent2D,
    ) -> Result<ash::vk::Pipeline, String> {
        use crate::vk_init;
        use ash::vk;
        use std::mem::size_of;

        let vertex_shader = vk_init::create_shader_module(
            &device,
            "shaders/spv/rt.vert.spv",
            vk::ShaderStageFlags::VERTEX,
        )?;
        let fragment_shader = vk_init::create_shader_module(
            &device,
            "shaders/spv/rt.frag.spv",
            vk::ShaderStageFlags::FRAGMENT,
        )?;
        let mut shaders = vec![vertex_shader, fragment_shader];

        let binding_description = vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride((size_of::<f32>() as u32) * 5)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build();
        let attribute_descriptions = vec![
            vk::VertexInputAttributeDescription::builder()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(0)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(size_of::<f32>() as u32 * 3)
                .build(),
        ];

        let pipeline = vk_init::create_graphics_pipeline(
            device,
            render_pass,
            pipeline_layout,
            swapchain_extent,
            &shaders,
            binding_description,
            &attribute_descriptions,
        )?;

        for shader_module in shaders.iter_mut() {
            crate::VkShaderModule::cleanup(device, shader_module);
        }

        Ok(pipeline)
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        log::info!("performing cleanup for Engine");

        unsafe {
            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device
                .destroy_descriptor_pool(self.descriptor_pool, None);
            self.device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
        crate::VkBuffer::cleanup(&self.device, &mut self.index_buffer);
        crate::VkBuffer::cleanup(&self.device, &mut self.vertex_buffer);
        unsafe {
            for &fb in self.framebuffers.iter() {
                self.device.destroy_framebuffer(fb, None);
            }
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
