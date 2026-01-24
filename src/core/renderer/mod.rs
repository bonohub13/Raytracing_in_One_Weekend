mod buffer;
mod camera;
mod shader;

use crate::core::{RtLabel, RtResult, Surface};
pub use buffer::*;
pub use camera::*;
pub use shader::*;
use wgpu::{ComputePass, ComputePipeline, Device, Queue, RenderPass, RenderPipeline};

#[derive(Debug, Clone)]
pub struct PathTracerDescriptor<'data, 'label> {
    pub render_pipeline_label: RtLabel<'label>,
    pub render_shader_desc: RenderShaderDescriptor<'label>,
    pub compute_pipeline_label: RtLabel<'label>,
    pub compute_shader_desc: ComputeShaderDescriptor<'label>,
    pub buffer_desc: PathTracerBufferDescriptor<'data, 'label>,
}

#[derive(Debug, Clone)]
pub struct PathTracer {
    camera: Camera,
    _buffer: PathTracerBuffer,
    render_bind_group: BindGroup,
    compute_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    compute_pipeline: ComputePipeline,
}

impl PathTracer {
    pub fn new(device: &Device, surface: &Surface, desc: &PathTracerDescriptor) -> RtResult<Self> {
        let buffer = PathTracerBuffer::new(device, surface, &desc.buffer_desc);
        let render_bind_group = buffer.create_render_bind_group(device, Some("Ray Display"));
        let compute_bind_group = buffer.create_compute_bind_group(device, Some("Ray Compute"));
        let render_shader = RenderShader::new(device, &desc.render_shader_desc)?;
        let render_pipeline =
            Self::create_render_pipeline(device, surface, &render_bind_group, &render_shader, desc);
        let compute_shader = ComputeShader::new(device, &desc.compute_shader_desc)?;
        let compute_pipeline =
            Self::create_compute_pipeline(device, &compute_bind_group, &compute_shader, desc);

        Ok(Self {
            camera: desc.buffer_desc.camera,
            _buffer: buffer,
            render_bind_group,
            compute_bind_group,
            render_pipeline,
            compute_pipeline,
        })
    }

    pub fn update(&mut self, _queue: &Queue) {}

    pub fn compute(&self, compute_pass: &mut ComputePass) {
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, self.compute_bind_group.bind_group(), &[]);
        compute_pass.dispatch_workgroups(
            (self.camera.resolution.x as u32).div_ceil(8),
            (self.camera.resolution.y as u32).div_ceil(8),
            1,
        );
    }

    pub fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, self.render_bind_group.bind_group(), &[]);
        render_pass.draw(0..3, 0..1);
    }

    fn create_render_pipeline(
        device: &Device,
        surface: &Surface,
        bind_group: &BindGroup,
        shader: &RenderShader,
        desc: &PathTracerDescriptor,
    ) -> RenderPipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&RtLabel::PipelineLayout(Some("Render")).to_string()),
            bind_group_layouts: &[bind_group.layout()],
            immediate_size: 0,
        });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&desc.render_pipeline_label.to_string()),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader.vertex(),
                entry_point: Some(desc.render_shader_desc.vertex_entry_point),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader.fragment(),
                entry_point: Some(desc.render_shader_desc.fragment_entry_point),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface.config().format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        })
    }

    fn create_compute_pipeline(
        device: &Device,
        bind_group: &BindGroup,
        shader: &ComputeShader,
        desc: &PathTracerDescriptor,
    ) -> ComputePipeline {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&RtLabel::PipelineLayout(Some("Compute")).to_string()),
            bind_group_layouts: &[bind_group.layout()],
            immediate_size: 0,
        });
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(&desc.compute_pipeline_label.to_string()),
            layout: Some(&pipeline_layout),
            module: shader.compute(),
            entry_point: Some(desc.compute_shader_desc.compute_entry_point),
            compilation_options: Default::default(),
            cache: Default::default(),
        })
    }
}
