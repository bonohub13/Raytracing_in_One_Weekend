use ash::vk;
use glam::Vec3A;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Vec3A,
}

impl Vertex {
    pub const fn new(position: &[f32; 3]) -> Self {
        Self {
            position: glam::vec3a(position[0], position[1], position[2]),
        }
    }

    #[inline]
    pub fn dynamic_binding<'desc>() -> vk::VertexInputBindingDescription2EXT<'desc> {
        vk::VertexInputBindingDescription2EXT::default()
            .binding(0)
            .stride(size_of::<Self>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .divisor(1)
    }

    #[inline]
    pub fn dynamic_attribute<'desc>() -> vk::VertexInputAttributeDescription2EXT<'desc> {
        vk::VertexInputAttributeDescription2EXT::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)
    }
}
