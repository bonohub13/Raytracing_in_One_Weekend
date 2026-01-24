use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum RtLabel<'label> {
    RenderEncoder(Option<&'label str>),
    RenderPass(Option<&'label str>),
    Shader(Option<&'label str>),
    PipelineLayout(Option<&'label str>),
    RenderPipeline(Option<&'label str>),
    ComputePipeline(Option<&'label str>),
    Buffer(Option<&'label str>),
    Texture(Option<&'label str>),
    Sampler(Option<&'label str>),
    BindGroupLayout(Option<&'label str>),
    BindGroup(Option<&'label str>),
}

impl Display for RtLabel<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::RenderEncoder(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Render Encoder")
                } else {
                    write!(f, "Render Encoder")
                }
            }
            Self::RenderPass(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Render Pass")
                } else {
                    write!(f, "Render Pass")
                }
            }
            Self::Shader(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Shader")
                } else {
                    write!(f, "Shader")
                }
            }
            Self::PipelineLayout(label) => {
                if let Some(label) = label {
                    write!(f, "Pipeline {label} Layout")
                } else {
                    write!(f, "Pipeline Layout")
                }
            }
            Self::RenderPipeline(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Render Pipeline")
                } else {
                    write!(f, "Render Pipeline")
                }
            }
            Self::ComputePipeline(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Compute Pipeline")
                } else {
                    write!(f, "Compute Pipeline")
                }
            }
            Self::Buffer(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Buffer")
                } else {
                    write!(f, "Buffer")
                }
            }
            Self::Texture(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Texture")
                } else {
                    write!(f, "Texture")
                }
            }
            Self::Sampler(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Sampler")
                } else {
                    write!(f, "Sampler")
                }
            }
            Self::BindGroupLayout(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Bind Group Layout")
                } else {
                    write!(f, "Bind Group Layout")
                }
            }
            Self::BindGroup(label) => {
                if let Some(label) = label {
                    write!(f, "{label} Bind Group")
                } else {
                    write!(f, "Bind Group")
                }
            }
        }
    }
}
