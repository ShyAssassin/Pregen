use std::sync::Arc;
use std::fmt::Debug;
use std::path::PathBuf;
use crate::asset::Image;
use crate::window::Window;
use std::collections::HashMap;
use super::{BindGroup, BindGroupState, PipelineDescriptor};
use super::{RenderTargetFormat, TextureFormat};
use super::{RenderTexture, Texture};
use super::{RenderPipeline, Sampler, SamplerMode, Shader};

#[derive(Debug)]
pub struct RenderContext {
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub _adapter: wgpu::Adapter,
    pub _instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,

    pub shaders: HashMap<PathBuf, Arc<Shader>>,
    pub samplers: HashMap<SamplerMode, Arc<Sampler>>,
    pub pipeline_layouts: HashMap<String, Arc<wgpu::PipelineLayout>>,
    pub bindgroup_layouts: HashMap<Vec<wgpu::BindGroupLayoutEntry>, Arc<wgpu::BindGroupLayout>>,
}

#[profiling::all_functions]
impl RenderContext {
    pub async fn new(window: &mut Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            flags: wgpu::InstanceFlags::DEBUG,
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
            // FIXME: this makes NSight crash because why not, cant use bitflags i guess
            // backends: wgpu::Backends::METAL | wgpu::Backends::VULKAN | wgpu::Backends::DX12,
            backends: wgpu::Backends::VULKAN,
            dx12_shader_compiler: wgpu::Dx12Compiler::Dxc {
                dxil_path: Some("bin/".into()),
                dxc_path: Some("bin/".into())
            }
        });

        // let surface = instance.create_surface(window.get_surface_target()).unwrap();
        let surface = unsafe {instance.create_surface_unsafe(window.get_surface_target_unsafe()).unwrap()};
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            power_preference: wgpu::PowerPreference::HighPerformance
        }).await.expect("Failed to get an adapter");

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Pregen Device"),
            required_limits: wgpu::Limits{
                max_bind_groups: 4,
                ..Default::default()
            },
            required_features: wgpu::Features::empty(),
            memory_hints: wgpu::MemoryHints::Performance,
        }, None).await.expect("Failed to get device");

        // on high DPI displays the framebuffer is larger than the window
        let buffer_size = window.get_framebuffer_size();
        let surface_format = surface.get_capabilities(&adapter).formats.iter()
            .find(|format| format.is_srgb())
            .copied().expect("Surface does not support srgb");
        let config = wgpu::SurfaceConfiguration {
            format: surface_format,
            width: buffer_size.0 as u32,
            height: buffer_size.1 as u32,
            desired_maximum_frame_latency: 0,
            present_mode: wgpu::PresentMode::Immediate,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: vec![surface_format, surface_format.remove_srgb_suffix()],
        };
        surface.configure(&device, &config);

        return Self {
            queue: queue,
            device: device,
            config: config,
            surface: surface,
            _adapter: adapter,
            _instance: instance,
            shaders: HashMap::new(),
            samplers: HashMap::new(),
            pipeline_layouts: HashMap::new(),
            bindgroup_layouts: HashMap::new(),
        };
    }

    pub fn create_pipeline(&mut self, desc: PipelineDescriptor) -> RenderPipeline {
        return RenderPipeline::new(&self.device, desc.name, desc.shader.clone());
    }

    pub fn create_shader(&mut self, path: impl Into<PathBuf>) -> Arc<Shader> {
        let path = path.into();
        return self.shaders.get(&path).cloned().unwrap_or_else(|| {
            println!("Creating shader from {:?}", path);
            let shader = Arc::new(Shader::new(&self.device, None, &path));
            self.shaders.insert(path.clone(), shader.clone());
            return shader;
        });
    }

    pub fn create_texture(&mut self, name: Option<&str>, mode: SamplerMode, format: TextureFormat, image: Arc<Image>) -> Texture {
        let sampler = self.create_sampler(name, mode);
        return Texture::new(&self.device, &self.queue, name, format, sampler, image);
    }

    pub fn create_sampler(&mut self, name: Option<&str>, mode: SamplerMode) -> Arc<Sampler> {
        return self.samplers.get(&mode).cloned().unwrap_or_else(|| {
            println!("Creating sampler {:?}", mode);
            let sampler = Arc::new(Sampler::new(&self.device, name, mode));
            self.samplers.insert(mode, sampler.clone());
            return sampler;
        });
    }

    pub fn create_render_target(&mut self, name: Option<&str>, mode: SamplerMode, width: u32, height: u32) -> RenderTexture {
        let sampler = self.create_sampler(name, mode);
        return RenderTexture::new(&self.device, name, sampler, self.config.format, width, height);
    }

    pub fn create_depth_texture(&mut self, name: Option<&str>, mode: SamplerMode, width: u32, height: u32) -> RenderTexture {
        let sampler = self.create_sampler(name, mode);
        return RenderTexture::new(&self.device, name, sampler, RenderTargetFormat::Depth32Float, width, height);
    }

    pub fn create_bind_group_layout(&mut self, descriptor: wgpu::BindGroupLayoutDescriptor) -> Arc<wgpu::BindGroupLayout> {
        return self.bindgroup_layouts.get(descriptor.entries).cloned().unwrap_or_else(|| {
            println!("Creating bind group layout for {:?}", descriptor.label);
            let layout = Arc::new(self.device.create_bind_group_layout(&descriptor));
            self.bindgroup_layouts.insert(descriptor.entries.to_vec(), layout.clone());
            return layout;
        });
    }

    pub fn create_bind_group<T: BindGroupState>(&mut self, initial_state: Option<T>) -> BindGroup<T> {
        let state = initial_state.unwrap_or_else(|| T::init(&self.device));
        let layout = self.create_bind_group_layout(state.get_layout_descriptor());
        return BindGroup::new(&self.device, Some(state.get_name()), state, layout);
    }
}
