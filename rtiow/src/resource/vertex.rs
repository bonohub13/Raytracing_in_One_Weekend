use ash::vk;
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex(pub(crate) Vec3);

pub trait StagingData {
    fn size(&self) -> vk::DeviceSize;
    fn copy_regions<'a>(&'a self) -> Vec<vk::BufferCopy2<'a>>;
    fn copy_barrier<'a>(&self) -> vk::MemoryBarrier2<'a>;
    fn write(&self, dst_slice: &mut [u8]);
}

impl Vertex {
    pub const fn new(position: &[f32; 3]) -> Self {
        Self(glam::vec3(position[0], position[1], position[2]))
    }

    pub fn bytes(vertices: &[Self]) -> Vec<u8> {
        bytemuck::cast_slice(
            vertices
                .iter()
                .map(|vertex| vertex.0)
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .to_vec()
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
