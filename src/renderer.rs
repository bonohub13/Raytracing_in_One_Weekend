// Copyright 2026 Kensuke Saito
// SPDX-License-Identifier: MIT

use rtiow::{
    Aabb, Allocator, AsLoader, Blas, BufferType, Encoder, GraphicsPipeline, PipelineLayout, RtErr,
    Sphere, Swapchain, SyncObject, Tlas, VkState,
};
use std::sync::{Arc, Mutex};
use winit::window::Window;

pub struct Renderer {
    aabb_tlas: Tlas,
    aabb_blas: Blas<Aabb>,
    as_loader: Arc<AsLoader>,
    graphics_pipeline: GraphicsPipeline,
    pipeline_layout: Arc<PipelineLayout>,
    swapchain: Swapchain,
    allocator: Arc<Mutex<Allocator>>,
    sync: SyncObject,
    encoder: Encoder,
    image_index: usize,
    current_frame: usize,
}

impl Renderer {
    const MAX_FRAMES_IN_FLIGHT: usize = 2;

    pub fn new(window: Arc<Window>, state: &VkState) -> RtErr<Self> {
        let max_frames_in_flight = Self::MAX_FRAMES_IN_FLIGHT as u32;
        let encoder = Encoder::new(state, max_frames_in_flight)?;
        let sync = SyncObject::new(state, max_frames_in_flight)?;
        let allocator = Arc::new(Mutex::new(Allocator::new(state)?));
        let swapchain = Swapchain::new(window, &encoder, state, allocator.clone())?;
        let pipeline_layout = Arc::new(PipelineLayout::new(state, &[])?);
        let graphics_pipeline = GraphicsPipeline::new(state, &swapchain, pipeline_layout.clone())?;
        let as_loader = Arc::new(AsLoader::new(state));
        let unit_sphere = Sphere::new(&[0f32, 0f32, 0f32], 1f32);
        let aabb_blas = Blas::new(
            state,
            allocator.clone(),
            &encoder,
            as_loader.clone(),
            BufferType::Aabb(&unit_sphere.aabb_data()),
        )?;
        let aabb_tlas = Tlas::new(state, allocator.clone(), as_loader.clone(), 1)?;

        Ok(Self {
            encoder,
            sync,
            allocator,
            swapchain,
            pipeline_layout,
            graphics_pipeline,
            as_loader,
            aabb_blas,
            aabb_tlas,
            image_index: 0,
            current_frame: 0,
        })
    }

    pub fn resize(
        &mut self,
        window: Arc<Window>,
        state: &VkState,
        physical_size: winit::dpi::PhysicalSize<u32>,
    ) -> RtErr<()> {
        self.sync.wait_for_fences(self.current_frame)?;
        self.swapchain
            .resize(window, state, &self.encoder, physical_size)
    }

    pub fn draw_frame(&mut self, window: Arc<Window>, state: &VkState) -> RtErr<()> {
        self.sync.wait_for_fences(self.current_frame)?;
        if let Some((image_index, _is_suboptimal)) = self.swapchain.acquire_next_image(
            window,
            state,
            &self.encoder,
            &self.sync,
            self.current_frame,
        )? {
            self.image_index = image_index;
        }
        self.sync.reset_fences(self.current_frame)?;
        self.encoder
            .record_frame(self.current_frame, |command_buffer| {
                self.swapchain
                    .transition_surface_to_color_attachment(command_buffer, self.image_index);
                self.encoder.render(
                    &self.swapchain,
                    self.image_index,
                    self.current_frame,
                    |command_buffer| {
                        self.graphics_pipeline.bind_pipeline(command_buffer);
                        self.graphics_pipeline
                            .set_viewport(command_buffer, self.swapchain.extent());
                        self.graphics_pipeline
                            .set_scissor(command_buffer, self.swapchain.extent());
                        self.graphics_pipeline.bind_set_input_ext(command_buffer);
                        self.graphics_pipeline.draw(command_buffer, 3, 1, 0, 0);
                    },
                );
                self.swapchain
                    .transition_surface_to_present(command_buffer, self.image_index);

                Ok(())
            })?;
        self.sync
            .submit_graphics_queue(&self.encoder, self.current_frame)?;
        self.swapchain
            .present_queue(&self.sync, self.image_index as u32, self.current_frame)?;

        self.current_frame = (self.current_frame + 1) % Self::MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }
}
