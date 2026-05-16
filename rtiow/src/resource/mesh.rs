use crate::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh {
    _vertices: Vec<Vertex>,
    _indices: Option<Vec<u32>>,
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
            _vertices: VERTICES.to_vec(),
            _indices: Some(INDICES.to_vec()),
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
            _vertices: VERTICES.to_vec(),
            _indices: Some(INDICES.to_vec()),
        }
    }
}
