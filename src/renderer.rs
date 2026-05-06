use rtiow::{GraphicsPipeline, PipelineLayout, RtErr, VkState};

pub struct Renderer {
    graphics_pipeline: GraphicsPipeline,
    pipeline_layout: PipelineLayout,
}

impl Renderer {
    pub fn new(state: &VkState) -> RtErr<Self> {
        let pipeline_layout = PipelineLayout::new(state)?;
        let graphics_pipeline = GraphicsPipeline::new(state, &pipeline_layout)?;

        Ok(Self {
            pipeline_layout,
            graphics_pipeline,
        })
    }
}
