use crate::core::{
    RtError, RtResult,
    device::Device,
    params::MAX_FRAMES_IN_FLIGHT,
    renderer::{path_tracer, swapchain::Swapchain},
    surface::Surface,
    util as rt_util,
};
use ash::vk;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct CommandDescriptor {
    pub device: Arc<Device>,
    pub surface: Arc<Surface>,
    pub swapchain: Arc<Mutex<Swapchain>>,
}

pub struct Command {
    device: Arc<Device>,
    swapchain: Arc<Mutex<Swapchain>>,
    pool: vk::CommandPool,
    buffers: Vec<vk::CommandBuffer>,
}

impl Command {
    pub fn new(desc: &CommandDescriptor) -> RtResult<Self> {
        let pool = Self::create_command_pool(desc)?;
        let buffers: Vec<vk::CommandBuffer> = (0..MAX_FRAMES_IN_FLIGHT)
            .map(|_| Self::create_command_buffer(pool, desc))
            .collect::<RtResult<_>>()?;

        Ok(Self {
            device: desc.device.clone(),
            swapchain: desc.swapchain.clone(),
            pool,
            buffers,
        })
    }

    #[inline]
    pub const fn buffers(&self) -> &[vk::CommandBuffer] {
        self.buffers.as_slice()
    }

    pub fn record_command_buffer(
        &self,
        pipeline: path_tracer::Pipeline,
        image_index: usize,
        current_frame: usize,
    ) -> RtResult<()> {
        self.begin_command_buffer(current_frame)?;
        self.transition_to_color_attachment(image_index, current_frame)?;
        self.begin_rendering(image_index, current_frame)?;
        self.bind_pipeline(pipeline, current_frame);
        self.set_viewport_and_scissor(current_frame)?;
        unsafe {
            self.device
                .device()
                .cmd_draw(self.buffers[current_frame], 3, 1, 0, 0)
        };
        self.end_rendering(current_frame);
        self.transition_to_present(image_index, current_frame)?;
        self.end_command_buffer(current_frame)
    }

    pub fn reset_command_buffer(&self, current_frame: usize) -> RtResult<()> {
        if let Err(err) = unsafe {
            self.device.device().reset_command_buffer(
                self.buffers[current_frame],
                vk::CommandBufferResetFlags::empty(),
            )
        } {
            Err(RtError::ResetCommandBuffer(err.into()))
        } else {
            Ok(())
        }
    }

    fn transition_to_color_attachment(
        &self,
        image_index: usize,
        current_frame: usize,
    ) -> RtResult<()> {
        const SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let barrier = {
            let guard = rt_util::lock_mutex!(self.swapchain);
            vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::NONE)
                .src_access_mask(vk::AccessFlags2::NONE)
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    vk::AccessFlags2::COLOR_ATTACHMENT_WRITE
                        | vk::AccessFlags2::COLOR_ATTACHMENT_READ,
                )
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(guard.images()[image_index])
                .subresource_range(SUBRESOURCE_RANGE)
        };
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .device()
                .cmd_pipeline_barrier2(self.buffers[current_frame], &dependency_info)
        };

        Ok(())
    }

    fn transition_to_present(&self, image_index: usize, current_frame: usize) -> RtResult<()> {
        const SUBRESOURCE_RANGE: vk::ImageSubresourceRange = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        let barrier = {
            let guard = rt_util::lock_mutex!(self.swapchain);
            vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::NONE)
                .dst_access_mask(vk::AccessFlags2::NONE)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .image(guard.images()[image_index])
                .subresource_range(SUBRESOURCE_RANGE)
        };
        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(&barrier));

        unsafe {
            self.device
                .device()
                .cmd_pipeline_barrier2(self.buffers[current_frame], &dependency_info)
        };

        Ok(())
    }

    fn begin_command_buffer(&self, current_frame: usize) -> RtResult<()> {
        let begin_info = vk::CommandBufferBeginInfo::default();

        if let Err(err) = unsafe {
            self.device
                .device()
                .begin_command_buffer(self.buffers[current_frame], &begin_info)
        } {
            Err(RtError::BeginCommandBuffer(err.into()))
        } else {
            Ok(())
        }
    }

    fn end_command_buffer(&self, current_frame: usize) -> RtResult<()> {
        if let Err(err) = unsafe {
            self.device
                .device()
                .end_command_buffer(self.buffers[current_frame])
        } {
            Err(RtError::EndCommandBuffer(err.into()))
        } else {
            Ok(())
        }
    }

    fn begin_rendering(&self, image_index: usize, current_frame: usize) -> RtResult<()> {
        const BLACK: vk::ClearValue = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0f32, 0f32, 0f32, 1f32],
            },
        };

        let guard = rt_util::lock_mutex!(self.swapchain);
        let attachment_info = vk::RenderingAttachmentInfo::default()
            .image_view(guard.image_view()[image_index])
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(BLACK);
        let rendering_info = vk::RenderingInfo::default()
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: guard.extent(),
            })
            .layer_count(1)
            .color_attachments(std::slice::from_ref(&attachment_info));

        drop(guard);

        unsafe {
            self.device
                .device()
                .cmd_begin_rendering(self.buffers[current_frame], &rendering_info)
        };

        Ok(())
    }

    fn end_rendering(&self, current_frame: usize) {
        unsafe {
            self.device
                .device()
                .cmd_end_rendering(self.buffers[current_frame])
        }
    }

    fn bind_pipeline(&self, pipeline: path_tracer::Pipeline, current_frame: usize) {
        let (pipeline_bind_point, pipeline) = match pipeline {
            path_tracer::Pipeline::Graphics(pipeline) => {
                (vk::PipelineBindPoint::GRAPHICS, pipeline)
            }
        };

        unsafe {
            self.device.device().cmd_bind_pipeline(
                self.buffers[current_frame],
                pipeline_bind_point,
                pipeline,
            )
        }
    }

    fn set_viewport_and_scissor(&self, current_frame: usize) -> RtResult<()> {
        let guard = rt_util::lock_mutex!(self.swapchain);
        let swapchain_extent = guard.extent();
        let viewport = vk::Viewport {
            x: 0f32,
            y: 0f32,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0f32,
            max_depth: 1f32,
        };
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        };

        drop(guard);

        unsafe {
            self.device.device().cmd_set_viewport(
                self.buffers[current_frame],
                0,
                std::slice::from_ref(&viewport),
            );
            self.device.device().cmd_set_scissor(
                self.buffers[current_frame],
                0,
                std::slice::from_ref(&scissor),
            );
        }

        Ok(())
    }

    fn create_command_pool(desc: &CommandDescriptor) -> RtResult<vk::CommandPool> {
        let queue_family_indices = desc.device.find_queue_families(desc.surface.clone())?;
        let create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_indices.graphics_family.unwrap());

        match unsafe { desc.device.device().create_command_pool(&create_info, None) } {
            Ok(command_pool) => Ok(command_pool),
            Err(err) => Err(RtError::CreateCommandPool(err.into())),
        }
    }

    fn create_command_buffer(
        command_pool: vk::CommandPool,
        desc: &CommandDescriptor,
    ) -> RtResult<vk::CommandBuffer> {
        let alloc_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        match unsafe { desc.device.device().allocate_command_buffers(&alloc_info) } {
            Ok(command_buffers) => {
                if let Some(command_buffer) = command_buffers.first() {
                    Ok(*command_buffer)
                } else {
                    Err(RtError::AllocateCommandBuffers(None))
                }
            }
            Err(err) => Err(RtError::AllocateCommandBuffers(Some(err.into()))),
        }
    }
}

impl Drop for Command {
    fn drop(&mut self) {
        unsafe {
            self.device.device().destroy_command_pool(self.pool, None);
        }
    }
}
