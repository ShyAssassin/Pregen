use wgpu::Device;
use crate::rend::Vertex;
use crate::gfx::{VertexBuffer, IndexBuffer};
type Primitive = (&'static [Vertex], &'static [u32]);

pub struct Geometry {
    pub name: String,
    pub indices: Vec<u32>,
    pub vertices: Vec<Vertex>,
    pub index_buffer: IndexBuffer,
    pub vertex_buffer: VertexBuffer<Vertex>,
}

impl Geometry {
    // TODO: Make vertex buffer input generic
    pub fn new(device: &Device, name: Option<&str>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let name = name.unwrap_or("Unnamed Geometry");

        let index_buffer = IndexBuffer::new(device, Some(format!("{} Index Buffer", name).as_str()), indices.clone());
        let vertex_buffer = VertexBuffer::new(device, Some(format!("{} Vertex Buffer", name).as_str()), vertices.clone());

        return Geometry {
            name: name.to_string(),
            indices,
            vertices,
            index_buffer,
            vertex_buffer,
        }
    }

    pub fn range(&self) -> std::ops::Range<u32> {
        0..self.indices.len() as u32
    }
}

impl std::fmt::Debug for Geometry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Geometry")
            .field("name", &self.name)
            .field("indices", &self.indices.len())
            .field("vertices", &self.vertices.len())
        .finish()
    }
}

impl Geometry {
    pub fn from_primitive(device: &Device, name: Option<&str>, primitive: &Primitive) -> Self {
        let vertices = primitive.0.to_vec();
        let indices = primitive.1.to_vec();
        return Self::new(device, name, vertices, indices);
    }

    #[allow(dead_code)]
    pub const TRIANGLE: Primitive = (
        &[
            Vertex {
                position: glam::Vec3::new(0.0, 0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(1.0, 0.0, 0.0),
                uv: glam::Vec2::new(0.5, 0.0),
            },
            Vertex {
                position: glam::Vec3::new(-0.5, -0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(0.0, 1.0, 0.0),
                uv: glam::Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: glam::Vec3::new(0.5, -0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(0.0, 0.0, 1.0),
                uv: glam::Vec2::new(1.0, 1.0),
            }
        ],
        &[
            0, 1, 2
        ]
    );

    #[allow(dead_code)]
    pub const QUAD: Primitive = (
        &[
            Vertex {
                position: glam::Vec3::new(-0.5, 0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(1.0, 0.0, 0.0),
                uv: glam::Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: glam::Vec3::new(-0.5, -0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(0.0, 1.0, 0.0),
                uv: glam::Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: glam::Vec3::new(0.5, 0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(0.0, 0.0, 1.0),
                uv: glam::Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: glam::Vec3::new(0.5, -0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(1.0, 0.0, 0.0),
                uv: glam::Vec2::new(1.0, 1.0),
            }
        ],
        &[
            0, 1, 2,
            1, 3, 2,
        ]
    );

    #[allow(dead_code)]
    pub const PYRAMID: Primitive = (
        &[
            Vertex {
                position: glam::Vec3::new(0.0, 0.5, 0.0),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(1.0, 0.0, 0.0),
                uv: glam::Vec2::new(0.5, 0.0),
            },
            Vertex {
                position: glam::Vec3::new(-0.5, -0.5, -0.5),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(0.0, 1.0, 0.0),
                uv: glam::Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: glam::Vec3::new(0.5, -0.5, -0.5),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(0.0, 0.0, 1.0),
                uv: glam::Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: glam::Vec3::new(0.5, -0.5, 0.5),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(1.0, 0.0, 0.0),
                uv: glam::Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: glam::Vec3::new(-0.5, -0.5, 0.5),
                normal: glam::Vec3::new(0.0, 0.0, 1.0),
                color: glam::Vec3::new(1.0, 0.0, 0.0),
                uv: glam::Vec2::new(0.0, 1.0),
            }
        ],
        &[
            0, 1, 2,
            0, 2, 3,
            0, 3, 4,
            0, 4, 1,
            3, 2, 1,
            3, 1, 4,
        ]
    );
}
