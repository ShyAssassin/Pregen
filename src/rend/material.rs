use std::sync::Arc;
use super::MaterialBindGroup;
use crate::gfx::{BindGroup, RenderContext, Texture};

pub struct Material {
    pub name: String,
    pub albedo: Arc<Texture>,
    pub normal: Arc<Texture>,
    pub ambient: Arc<Texture>,
    pub group: BindGroup<MaterialBindGroup>,
}

impl Material {
    pub fn new(context: &mut RenderContext, name: Option<&str>, albedo: Arc<Texture>, normal: Arc<Texture>, ambient: Arc<Texture>) -> Self {
        let name = name.unwrap_or("Unnamed Material");
        let group = context.create_bind_group(Some(MaterialBindGroup::new(albedo.clone(), normal.clone(), ambient.clone())));
        return Self {
            name: name.to_string(),
            albedo: albedo,
            normal: normal,
            ambient: ambient,
            group: group,
        };
    }
}
