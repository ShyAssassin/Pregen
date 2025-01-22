#[derive(Debug)]
pub struct Device {
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub adapter: wgpu::Adapter,
    pub instance: wgpu::Instance,
}

impl Device {
    // TODO: consider just blocking on this function
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::from_build_config(),
            backend_options: wgpu::BackendOptions {
                gl: wgpu::GlBackendOptions::default(),
                dx12: wgpu::Dx12BackendOptions {
                    shader_compiler: wgpu::Dx12Compiler::DynamicDxc {
                        dxil_path: "bin/".into(),
                        dxc_path: "bin/".into()
                    }
                },
            }
        });

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            compatible_surface: None,
            force_fallback_adapter: false,
            power_preference: wgpu::PowerPreference::HighPerformance
        }).await.expect("Failed to get an adapter");

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("gfx Device"),
            required_limits: wgpu::Limits::default(),
            required_features: wgpu::Features::empty(),
            memory_hints: wgpu::MemoryHints::Performance,
        }, None).await.expect("Failed to get a device");

        return Self {
            queue: queue,
            device: device,
            adapter: adapter,
            instance: instance,
        };
    }
}
