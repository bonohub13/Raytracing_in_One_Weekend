pub struct Engine {
    instance: ash::Instance,
    surface: crate::VkSurface,
    physical_device: ash::vk::PhysicalDevice,
    queue_family_indices: crate::QueueFamilyIndices,
    device: ash::Device,
    compute_queue: ash::vk::Queue,
    graphics_queue: ash::vk::Queue,
    present_queue: ash::vk::Queue,
    pub swapchain: crate::VkSwapchain,

    pub present_complete: ash::vk::Semaphore,
    pub render_complete: ash::vk::Semaphore,

    compute_command_pool: ash::vk::CommandPool,
    compute_command_buffers: Vec<ash::vk::CommandBuffer>,
    compute_fences: Vec<ash::vk::Fence>,
    compute_image: crate::VkImage,

    ubo: crate::types::UniformBufferObject,
    ubo_buffer: crate::VkBuffer,

    compute_descriptor_set_layout: ash::vk::DescriptorSetLayout,
    compute_descriptor_pool: ash::vk::DescriptorPool,
    compute_descriptor_set: ash::vk::DescriptorSet,

    compute_pipeline_layout: ash::vk::PipelineLayout,
    compute_pipeline: ash::vk::Pipeline,

    command_pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,
    command_buffers: Vec<ash::vk::CommandBuffer>,

    render_fences: Vec<ash::vk::Fence>,
    pub render_pass: ash::vk::RenderPass,
    framebuffers: Vec<ash::vk::Framebuffer>,

    vertex_buffer: crate::VkBuffer,
    index_buffer: crate::VkBuffer,

    descriptor_set_layout: ash::vk::DescriptorSetLayout,
    descriptor_pool: ash::vk::DescriptorPool,
    descriptor_set: ash::vk::DescriptorSet,

    pipeline_layout: ash::vk::PipelineLayout,
    graphics_pipeline: ash::vk::Pipeline,

    is_framebuffer_resized: bool,
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
        let mut compute_command_buffers: Vec<vk::CommandBuffer> = Vec::new();
        let mut compute_fences: Vec<vk::Fence> = Vec::new();
        for _ in 0..swapchain.images.len() {
            let compute_command_buffer =
                vk_init::create_command_buffer(&device, &compute_command_pool)?;
            let compute_fence =
                vk_init::create_fence(&device, Some(vk::FenceCreateFlags::SIGNALED))?;

            compute_command_buffers.push(compute_command_buffer);
            compute_fences.push(compute_fence);
        }
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

        let command_buffers =
            vk_init::create_command_buffers(&device, &command_pool, swapchain.images.len())?;

        let mut render_fences: Vec<vk::Fence> = Vec::new();
        for _ in 0..swapchain.images.len() {
            let render_fence =
                vk_init::create_fence(&device, Some(vk::FenceCreateFlags::SIGNALED))?;

            render_fences.push(render_fence);
        }
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
            compute_command_buffers,
            compute_fences,
            compute_image,
            ubo_buffer,
            ubo,
            compute_descriptor_set_layout,
            compute_descriptor_pool,
            compute_descriptor_set,
            compute_pipeline_layout,
            compute_pipeline,
            command_buffers,
            render_fences,
            render_pass,
            framebuffers,
            vertex_buffer,
            index_buffer,
            descriptor_set_layout,
            descriptor_pool,
            descriptor_set,
            pipeline_layout,
            graphics_pipeline,
            is_framebuffer_resized: false,
        })
    }

    pub fn device_wait_idle(&self) -> Result<(), String> {
        unsafe {
            self.device
                .device_wait_idle()
                .map_err(|_| String::from("logical device failed to wait at idle"))
        }
    }

    pub fn queue_submit(
        &self,
        queue: ash::vk::Queue,
        submits: &[ash::vk::SubmitInfo],
        fence: ash::vk::Fence,
    ) -> Result<(), String> {
        log::info!("submitting command buffer or semaphore to a queue");

        unsafe {
            self.device
                .queue_submit(queue, submits, fence)
                .map_err(|_| {
                    String::from("failed to submit command buffer or semaphore to a queue")
                })
        }
    }

    pub fn begin_command_buffer(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        begin_info: &ash::vk::CommandBufferBeginInfo,
    ) -> Result<(), String> {
        log::info!("beginning command buffer");

        unsafe {
            self.device
                .begin_command_buffer(command_buffer, begin_info)
                .map_err(|_| String::from("failed to begin command buffer"))
        }
    }

    pub fn cmd_begin_render_pass(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        create_info: &ash::vk::RenderPassBeginInfo,
        contents: ash::vk::SubpassContents,
    ) {
        unsafe {
            self.device
                .cmd_begin_render_pass(command_buffer, create_info, contents)
        }
    }

    pub fn cmd_bind_pipeline(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        pipeline: ash::vk::Pipeline,
    ) {
        log::info!("binding pipeline to command buffer");

        unsafe {
            self.device
                .cmd_bind_pipeline(command_buffer, pipeline_bind_point, pipeline)
        }
    }

    pub fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        first_binding: u32,
        buffers: &[ash::vk::Buffer],
        offsets: &[ash::vk::DeviceSize],
    ) {
        log::info!("binding vertex buffers to command buffer");

        unsafe {
            self.device
                .cmd_bind_vertex_buffers(command_buffer, first_binding, buffers, offsets)
        }
    }

    pub fn cmd_bind_index_buffer(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        buffer: ash::vk::Buffer,
        offset: ash::vk::DeviceSize,
        index_type: ash::vk::IndexType,
    ) {
        log::info!("binding index buffer to command buffer");

        unsafe {
            self.device
                .cmd_bind_index_buffer(command_buffer, buffer, offset, index_type)
        }
    }

    pub fn cmd_draw_indexed(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            self.device.cmd_draw_indexed(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }

    pub fn cmd_end_render_pass(&self, command_buffer: ash::vk::CommandBuffer) {
        unsafe { self.device.cmd_end_render_pass(command_buffer) }
    }

    pub fn end_command_buffer(&self, command_buffer: ash::vk::CommandBuffer) -> Result<(), String> {
        unsafe {
            self.device
                .end_command_buffer(command_buffer)
                .map_err(|_| String::from("failed to end command buffer"))
        }
    }

    pub fn queue_wait_idle(&self, queue: ash::vk::Queue) -> Result<(), String> {
        unsafe {
            self.device
                .queue_wait_idle(queue)
                .map_err(|_| String::from("queue failed to wait idle"))
        }
    }

    pub fn cmd_dispatch(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            self.device
                .cmd_dispatch(command_buffer, group_count_x, group_count_y, group_count_z)
        }
    }

    pub fn cmd_draw(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.device.cmd_draw(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }

    pub fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: ash::vk::PipelineLayout,
        first_set: u32,
        descriptor_sets: &[ash::vk::DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            self.device.cmd_bind_descriptor_sets(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets,
                dynamic_offsets,
            )
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

    pub fn reset_fences(&self, fences: &[ash::vk::Fence]) -> Result<(), String> {
        unsafe {
            self.device
                .reset_fences(fences)
                .map_err(|_| String::from("failed to reset fences"))
        }
    }

    pub fn render_loop(
        &mut self,
        index: &mut u32,
        frame_count: &mut u32,
        begin_info: &ash::vk::CommandBufferBeginInfo,
        offsets: &[ash::vk::DeviceSize],
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        use ash::vk;

        let compute_index = (*frame_count % self.swapchain.images.len() as u32) as usize;

        let compute_fence = self.compute_fences[compute_index];
        self.wait_for_fences(&[compute_fence], true, u64::MAX)?;
        self.reset_fences(&[compute_fence])?;

        let compute_command_buffer = self.compute_command_buffers[compute_index];
        self.begin_command_buffer(compute_command_buffer, begin_info)?;
        self.cmd_bind_pipeline(
            compute_command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            self.compute_pipeline,
        );
        self.cmd_bind_descriptor_sets(
            compute_command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            self.compute_pipeline_layout,
            0,
            &[self.compute_descriptor_set],
            &[],
        );
        self.cmd_dispatch(compute_command_buffer, 1024 / 16, 1024 / 24, 1);
        self.end_command_buffer(compute_command_buffer)?;

        let compute_info = vk::SubmitInfo::builder()
            .command_buffers(&[compute_command_buffer])
            .build();
        self.queue_submit(self.compute_queue, &[compute_info], compute_fence)?;

        (*index, _) = self.swapchain.acquire_next_image(
            u64::MAX,
            self.present_complete,
            vk::Fence::null(),
        )?;
        let current_index = index.clone() as usize;
        let current_fence = self.render_fences[current_index];
        self.wait_for_fences(&[current_fence], true, u64::MAX)?;
        self.reset_fences(&[current_fence])?;

        let cmd = self.command_buffers[current_index];

        self.begin_command_buffer(cmd, begin_info)?;

        let clear_values = vec![vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 0.0],
            },
        }];
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D {
                    width: crate::constants::WIDTH,
                    height: crate::constants::HEIGHT,
                },
            })
            .clear_values(&clear_values)
            .framebuffer(self.framebuffers[current_index])
            .build();
        self.cmd_begin_render_pass(cmd, &render_pass_begin_info, vk::SubpassContents::INLINE);
        self.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.graphics_pipeline);
        self.cmd_bind_descriptor_sets(
            cmd,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline_layout,
            0,
            &[self.descriptor_set],
            &[],
        );
        self.cmd_bind_vertex_buffers(cmd, 0, &[self.vertex_buffer.buffer], offsets);
        self.cmd_bind_index_buffer(cmd, self.index_buffer.buffer, 0, vk::IndexType::UINT32);
        self.cmd_draw_indexed(cmd, 6, 1, 0, 0, 0);
        self.cmd_end_render_pass(cmd);
        self.end_command_buffer(cmd)?;

        let submit_info = vk::SubmitInfo::builder()
            .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .wait_semaphores(&[self.present_complete])
            .signal_semaphores(&[self.render_complete])
            .command_buffers(&[cmd])
            .build();
        self.queue_submit(self.graphics_queue, &[submit_info], current_fence)?;

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&[self.render_complete])
            .swapchains(&[self.swapchain.swapchain])
            .image_indices(&[*index])
            .build();
        let is_resized = match self
            .swapchain
            .queue_present(self.present_queue, &present_info)
        {
            Ok(_) => self.is_framebuffer_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => return Err(String::from("failed to present swapchain image")),
            },
        };

        if is_resized {
            self.is_framebuffer_resized = false;
            self.recreate_swapchain(width, height).map_err(|err| {
                log::error!("{}", err);

                String::from("failure occurred while updating swapchain")
            })?;
        }

        *frame_count += 1;

        Ok(())
    }

    fn recreate_swapchain(&mut self, width: u32, height: u32) -> Result<(), String> {
        use crate::vk_init;

        self.device_wait_idle()?;

        // cleanup swapchain
        unsafe {
            for &fb in self.framebuffers.iter() {
                self.device.destroy_framebuffer(fb, None);
            }
            self.device.destroy_render_pass(self.render_pass, None);
            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device
                .free_command_buffers(self.command_pool, &self.command_buffers);
            self.device
                .free_command_buffers(self.command_pool, &[self.command_buffer]);
        }
        crate::VkSwapchain::cleanup(&self.device, &mut self.swapchain);

        let surface_capabilities = self
            .surface
            .get_physical_device_surface_capabilities(self.physical_device)?;
        let present_modes = self
            .surface
            .get_physical_device_surface_present_modes(self.physical_device)?;
        let present_mode = vk_init::choose_swapchain_present_mode(&present_modes);
        let surface_formats = self
            .surface
            .get_physical_device_surface_formats(self.physical_device)?;
        let surface_format = vk_init::choose_swapchain_format(&surface_formats)?;

        self.swapchain = vk_init::create_swapchain(
            &self.instance,
            self.physical_device,
            &self.surface,
            &self.device,
            width,
            height,
            &surface_capabilities,
            present_mode,
            &surface_format,
        )?;

        self.render_pass = vk_init::create_render_pass(&self.device, surface_format.format)?;

        self.framebuffers = vk_init::create_framebuffers(
            &self.device,
            self.render_pass,
            &self.swapchain.image_views,
            self.swapchain.extent.width,
            self.swapchain.extent.height,
        )?;

        self.pipeline_layout =
            vk_init::create_pipeline_layout(&self.device, self.descriptor_set_layout)?;
        self.graphics_pipeline = Self::create_graphics_pipeline(
            &self.device,
            self.render_pass,
            self.pipeline_layout,
            self.swapchain.extent,
        )?;

        self.command_buffer = vk_init::create_command_buffer(&self.device, &self.command_pool)?;
        self.command_buffers = vk_init::create_command_buffers(
            &self.device,
            &self.command_pool,
            self.swapchain.image_views.len(),
        )?;

        Ok(())
    }

    pub fn update_framebuffer(&mut self) {
        self.is_framebuffer_resized = true;
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
            match self.device_wait_idle() {
                Ok(_) => (),
                Err(err) => log::error!("[ERROR] {}", err),
            };
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
            for &render_fence in self.render_fences.iter() {
                self.device.destroy_fence(render_fence, None);
            }
            self.device
                .free_command_buffers(self.command_pool, &self.command_buffers);

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
            for &compute_fence in self.compute_fences.iter() {
                self.device.destroy_fence(compute_fence, None);
            }
            self.device
                .free_command_buffers(self.compute_command_pool, &self.compute_command_buffers);
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
