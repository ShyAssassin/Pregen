// use std::sync::Arc;
// use crate::gfx::{PipelineState, Shader, ShaderStage};


// pub struct GenericPipelineDescriptor<'a> {
//     pub name: Option<&'a str>,
//     pub shader: &'a Arc<Shader>,
//     pub enable_depth_stencil: bool,
//     pub bind_group_layouts: &'a [&'a Arc<wgpu::BindGroupLayout>],
// }

// pub struct GenericPipeline {
//     pub name: String,
//     pub shader: Arc<Shader>,
//     pub enable_depth_stencil: bool,
//     pub bind_group_layouts: Vec<Arc<wgpu::BindGroupLayout>>,
// }

// impl GenericPipeline {
//     pub fn new(descriptor: &GenericPipelineDescriptor) -> Self {
//         return Self {
//             name: descriptor.name.unwrap_or("Unnamed GenericPipeline").to_string(),
//             enable_depth_stencil: descriptor.enable_depth_stencil,
//             shader: descriptor.shader.clone(),
//             bind_group_layouts: descriptor.bind_group_layouts.iter().map(|layout|
//                 (*layout).clone()
//             ).collect::<Vec<Arc<wgpu::BindGroupLayout>>>(),
//         };
//     }
// }

// impl PipelineState for GenericPipeline {
//     fn get_name(&self) -> &str {
//         return self.name.as_str();
//     }

//     fn get_shader_path(&self) -> &str {
//         return self.shader.src_path.to_str().unwrap();
//     }

//     fn get_layout_descriptor(&self, device: &wgpu::Device) -> &wgpu::RenderPipelineDescriptor {
//         todo!()
//         // return &wgpu::RenderPipelineDescriptor {
//         //     label: Some(&self.name.as_str()),
//         //     cache: None,
//         //     layout: None,
//         //     fragment: Some(wgpu::FragmentState {
//         //         targets: &[None],
//         //         module: &self.shader.module,
//         //         entry_point: self.shader.get_entry(ShaderStage::Fragment),
//         //         compilation_options: wgpu::PipelineCompilationOptions::default(),
//         //     }),
//         //     depth_stencil: None,
//         // }
//     }
// }
