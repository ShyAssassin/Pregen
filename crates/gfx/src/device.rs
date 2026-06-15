#[derive(Debug)]
pub struct Device {
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub adapter: wgpu::Adapter,
    pub instance: wgpu::Instance,
}

impl Device {
    const WANTED_FEATURES: &[wgpu::Features] = &[
        wgpu::Features::TIMESTAMP_QUERY,
        wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES,
        wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
    ];

    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY.with_env(),
            flags: wgpu::InstanceFlags::from_env_or_default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions {
                gl: wgpu::GlBackendOptions::from_env_or_default(),
                noop: wgpu::NoopBackendOptions::from_env_or_default(),
                dx12: wgpu::Dx12BackendOptions {
                    shader_compiler: wgpu::Dx12Compiler::DynamicDxc {
                        dxc_path: "bin/dxcompiler.dll".into(),
                        max_shader_model: wgpu::DxcShaderModel::V6_5,
                    },
                    presentation_system: wgpu::wgt::Dx12SwapchainKind::DxgiFromHwnd,
                    latency_waitable_object: wgpu::wgt::Dx12UseFrameLatencyWaitableObject::Wait,
                }.with_env(),
            },
        });

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            compatible_surface: None,
            force_fallback_adapter: false,
            power_preference: wgpu::PowerPreference::HighPerformance
        }).await.expect("Failed to get an adapter");
        let info = adapter.get_info();
        log::info!("Using {:?} {} ({:?})", info.device_type, info.name, info.backend);

        let adapter_features = adapter.features();
        let mut supported_features = wgpu::Features::empty();
        for feature in Self::WANTED_FEATURES {
            if adapter_features.contains(*feature) {
                supported_features |= *feature;
            } else {
                log::warn!("{} is not supported by adapter", feature);
            }
        }

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            trace: wgpu::Trace::Off,
            label: Some("Pregen GFX Device"),
            required_features: supported_features,
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
        }).await.expect("Failed to acquire a suitable device");
        log::info!("Enabled features: [{}]", device.features());

        return Self {
            queue: queue,
            device: device,
            adapter: adapter,
            instance: instance,
        };
    }
}
