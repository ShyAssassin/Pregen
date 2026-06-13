mod index; mod vertex;
mod storage; mod uniform;

#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
pub enum BufferType {
    Index, Vertex,
    Storage, Uniform,
}

pub trait Buffer {
    fn size(&self) -> usize;
    fn label(&self) -> String;
    fn generation(&self) -> u64;
    fn r#type(&self) -> BufferType;
    fn buffer(&self) -> &wgpu::Buffer;
    fn usage(&self) -> wgpu::BufferUsages;
}

pub use index::IndexBuffer;
pub use vertex::VertexBuffer;
pub use storage::StorageBuffer;
pub use uniform::{UniformBuffer};
