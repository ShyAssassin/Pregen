use std::sync::Arc;
use crate::gfx::Shader;

#[derive(Debug)]
pub struct PipelineDescriptor<'a> {
    pub name: Option<&'a str>,
    pub shader: &'a Arc<Shader>,
    pub enable_depth_stencil: bool,
    pub bind_group_layouts: &'a [&'a Arc<wgpu::BindGroupLayout>],
}

#[derive(Debug)]
pub struct RenderPipeline {
    pub name: String,
    pub shader: Arc<Shader>,
    pub enable_depth_stencil: bool,
    pub layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(device: &wgpu::Device, name: Option<&str>, shader: Arc<Shader>) -> Self {
        let name = name.unwrap_or("Unnamed RenderPipeline").to_string();
        todo!()
    }
}
