use std::sync::Arc;
use super::{Vertex, Material};
use crate::gfx::{Geometry, RenderContext};

pub struct Mesh {
    pub name: String,
    pub geometry: Geometry,
    pub material: Arc<Material>,
}

impl Mesh {
    pub fn new(ctx: &RenderContext, name: Option<&str>, indices: Vec<u32>, vertices: Vec<Vertex>, material: Arc<Material>) -> Self {
        let geometry = Geometry::new(&ctx.device, name, vertices, indices);
        return Mesh::from_geometry(name, geometry, material);
    }

    pub fn from_geometry(name: Option<&str>, geometry: Geometry, material: Arc<Material>) -> Self {
        let name = name.unwrap_or("Unnamed Mesh");
        Mesh {
            name: name.to_string(),
            geometry: geometry,
            material: material
        }
    }
}
