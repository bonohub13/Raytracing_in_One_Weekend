use crate::{Buffer, StagingData, Vertex, util};
use ash::vk;

#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    pub(crate) indices: Option<Vec<u32>>,
}

impl Mesh {
    pub fn unit_rectangle() -> Self {
        static VERTICES: [Vertex; 4] = [
            Vertex::new(&[-0.5, -0.5, 0f32]), // Upper left
            Vertex::new(&[-0.5, 0.5, 0f32]),  // Bottom left
            Vertex::new(&[0.5, 0.5, 0f32]),   // Bottom right
            Vertex::new(&[0.5, -0.5, 0f32]),  // Upper right
        ];
        static INDICES: [u32; 6] = [
            0, 1, 2, // Bottom left triangle
            3, 2, 1, // Upper right triangle
        ];

        Self {
            vertices: VERTICES.to_vec(),
            indices: Some(INDICES.to_vec()),
        }
    }

    pub fn unit_cube() -> Self {
        static VERTICES: [Vertex; 8] = [
            Vertex::new(&[-0.5, -0.5, 0f32]), // Upper left (Close)
            Vertex::new(&[-0.5, 0.5, 0f32]),  // Bottom left (Close)
            Vertex::new(&[0.5, 0.5, 0f32]),   // Bottom right (Close)
            Vertex::new(&[0.5, -0.5, 0f32]),  // Upper right (Close)
            Vertex::new(&[-0.5, -0.5, 1f32]), // Upper left (Far)
            Vertex::new(&[-0.5, 0.5, 1f32]),  // Bottom left (Far)
            Vertex::new(&[0.5, 0.5, 1f32]),   // Bottom right (Far)
            Vertex::new(&[0.5, -0.5, 1f32]),  // Upper right (Far)
        ];
        static INDICES: [u32; 36] = [
            0, 3, 2, 2, 1, 0, // Front (Z=0)
            3, 7, 6, 6, 2, 3, // Right (X=0.5)
            7, 4, 5, 5, 6, 7, // Back (Z=1)
            4, 0, 1, 1, 5, 4, // Left (X=-0.5)
            4, 7, 3, 3, 0, 4, // Bottom (Y=-0.5)
            1, 2, 6, 6, 5, 1, // Top (Y=0.5)
        ];

        Mesh {
            vertices: VERTICES.to_vec(),
            indices: Some(INDICES.to_vec()),
        }
    }

    pub const fn vertex_size(&self) -> vk::DeviceSize {
        size_of_val(self.vertices.as_slice()) as vk::DeviceSize
    }

    pub const fn index_offset(&self) -> Option<vk::DeviceSize> {
        const SIZE_ALIGNMENT: vk::DeviceSize = 0x10;

        let vertex_size = self.vertex_size();

        if self.indices.is_some() {
            Some(util::align_up!(vertex_size, SIZE_ALIGNMENT))
        } else {
            None
        }
    }

    pub const fn index_size(&self) -> Option<vk::DeviceSize> {
        if let Some(indices) = self.indices.as_ref()
            && !indices.is_empty()
        {
            Some(size_of_val(indices.as_slice()) as vk::DeviceSize)
        } else {
            None
        }
    }

    pub fn geometry_data<T>(
        &self,
        buffer: &Buffer<T>,
    ) -> vk::AccelerationStructureGeometryTrianglesDataKHR<'_>
    where
        T: Clone + Sized,
    {
        const SIZE: vk::DeviceSize = size_of::<Vertex>() as vk::DeviceSize;
        const INDEX_BUFFER_ADDRESS_INDEX: usize = 1;

        let index_data = if buffer.gpu_address().len() == (INDEX_BUFFER_ADDRESS_INDEX + 1) {
            buffer.gpu_address()[INDEX_BUFFER_ADDRESS_INDEX]
        } else {
            0
        };

        vk::AccelerationStructureGeometryTrianglesDataKHR::default()
            .vertex_format(vk::Format::R32G32B32_SFLOAT)
            .vertex_data(vk::DeviceOrHostAddressConstKHR {
                device_address: buffer.gpu_address()[0],
            })
            .vertex_stride(SIZE)
            .max_vertex(self.vertices.len() as u32)
            .index_type(vk::IndexType::UINT32)
            .index_data(vk::DeviceOrHostAddressConstKHR {
                device_address: index_data,
            })
            .transform_data(vk::DeviceOrHostAddressConstKHR { device_address: 0 })
    }
}

impl StagingData for Mesh {
    fn size(&self) -> vk::DeviceSize {
        if let Some(index_offset) = self.index_offset()
            && let Some(index_size) = self.index_size()
        {
            index_offset + index_size
        } else {
            self.vertex_size()
        }
    }

    fn copy_regions<'a>(&'a self) -> Vec<vk::BufferCopy2<'a>> {
        let vertex_size = self.vertex_size();

        if let Some(index_offset) = self.index_offset()
            && let Some(index_size) = self.index_size()
        {
            vec![
                vk::BufferCopy2::default()
                    .src_offset(0)
                    .dst_offset(0)
                    .size(vertex_size),
                vk::BufferCopy2::default()
                    .src_offset(vertex_size)
                    .dst_offset(index_offset)
                    .size(index_size),
            ]
        } else {
            vec![
                vk::BufferCopy2::default()
                    .src_offset(0)
                    .dst_offset(0)
                    .size(vertex_size),
            ]
        }
    }

    fn copy_barrier<'a>(&self) -> vk::MemoryBarrier2<'a> {
        if self.indices.is_some() {
            vk::MemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(
                    vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR
                        | vk::PipelineStageFlags2::VERTEX_INPUT
                        | vk::PipelineStageFlags2::INDEX_INPUT,
                )
                .dst_access_mask(
                    vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR
                        | vk::AccessFlags2::VERTEX_ATTRIBUTE_READ
                        | vk::AccessFlags2::INDEX_READ,
                )
        } else {
            vk::MemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(
                    vk::PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR
                        | vk::PipelineStageFlags2::VERTEX_INPUT,
                )
                .dst_access_mask(
                    vk::AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR
                        | vk::AccessFlags2::VERTEX_ATTRIBUTE_READ,
                )
        }
    }

    fn write(&self, dst_slice: &mut [u8]) {
        let vertex_size = self.vertex_size() as usize;
        let vertices = Vertex::bytes(&self.vertices);

        dst_slice[0..vertex_size].copy_from_slice(&vertices);
        if let Some(indices) = self.indices.as_ref()
            && let Some(index_offset) = self.index_offset()
        {
            let index_offset = index_offset as usize;
            let indices = bytemuck::cast_slice(indices);

            dst_slice[index_offset..].copy_from_slice(indices);
        }
    }
}
