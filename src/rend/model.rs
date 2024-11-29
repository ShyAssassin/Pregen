use glam::Mat4;
use std::sync::Arc;
use crate::math::Transform;
use super::{Material, ModelBindGroup};
use crate::{asset::Image, gfx::{BindGroup, RenderContext, SamplerMode, TextureFormat}};

use super::{Mesh, Vertex};

pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
    pub transform: Transform,
    pub group: BindGroup<ModelBindGroup>,
}

impl Model {
    pub fn new(ctx: &mut RenderContext, name: Option<&str>, meshes: Vec<Mesh>, transform: Transform) -> Self {
        let mut group = ctx.create_bind_group::<ModelBindGroup>(None);
        group.state.u_transform.set(transform);

        let mut model = Self {
            name: name.unwrap_or("Unnamed Model").to_string(),
            transform: transform,
            meshes: meshes,
            group: group,
        };
        model.update(ctx);

        return model;
    }

    pub fn update(&mut self, ctx: &RenderContext) {
        let mut model_matrix = Mat4::from_translation(self.transform.translation);
        model_matrix = model_matrix * Mat4::from_quat(self.transform.rotation);
        model_matrix = model_matrix * Mat4::from_scale(self.transform.scale);
        self.group.u_transform.set(self.transform);
        self.group.u_model.set(model_matrix);
        self.group.update(&ctx.queue);
    }

    pub fn from_path(ctx: &mut RenderContext, name: Option<&str>, path: &str, transform: Transform) -> Self {
        let (models, materials) = tobj::load_obj(path, &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        }).unwrap();
        let base_path = std::path::Path::new(path).parent().unwrap();
        let materials = materials.unwrap().iter().map(|m| {
            let mat = m.clone();
            let normal_path = base_path.join(mat.normal_texture.clone().unwrap());
            let albedo_path = base_path.join(mat.diffuse_texture.clone().unwrap());
            let ambient_path = base_path.join(mat.ambient_texture.clone().unwrap());

            let albedo_image = Arc::new(Image::from_path(&albedo_path.to_str().unwrap().into(), 5));
            let normal_image = Arc::new(Image::from_path(&normal_path.to_str().unwrap().into(), 5));
            let ambient_image = Arc::new(Image::from_path(&ambient_path.to_str().unwrap().into(), 5));

            let normal = Arc::new(ctx.create_texture(None, SamplerMode::REPEAT, TextureFormat::Rgba8Unorm, normal_image));
            let albedo = Arc::new(ctx.create_texture(None, SamplerMode::REPEAT, TextureFormat::Rgba8UnormSrgb, albedo_image));
            let ambient = Arc::new(ctx.create_texture(None, SamplerMode::REPEAT, TextureFormat::Rgba8UnormSrgb, ambient_image));
            Arc::new(Material::new(ctx, None, albedo, normal, ambient))
        }).collect::<Vec<Arc<Material>>>();

        let mut geo = Vec::new();
        for model in models {
            let mesh = model.mesh;
            let mut indices = Vec::new();
            let mut vertices = Vec::new();
            for i in 0..mesh.positions.len() / 3 {
                vertices.push(Vertex::new(
                    [mesh.positions[3 * i], mesh.positions[3 * i + 1], mesh.positions[3 * i + 2]].into(),
                    [mesh.normals[3 * i], mesh.normals[3 * i + 1], mesh.normals[3 * i + 2]].into(),
                    [mesh.texcoords[2 * i], mesh.texcoords[2 * i + 1]].into(),
                    None
                ));
            }
            for i in (0..mesh.indices.len()).rev() {
                indices.push(mesh.indices[i]);
            }
            let id = mesh.material_id.unwrap_or(0);
            geo.push(Mesh::new(ctx, name, indices, vertices, materials[id].clone()));
        }
        return Self::new(ctx, name, geo, transform);
    }
}
