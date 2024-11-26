#[macro_export]
macro_rules! None {
    ($value:expr) => {
        None
    };
}

#[macro_export]
macro_rules! Some {
    ($value:expr) => {
        Some($value)
    };
}

#[macro_export]
macro_rules! into {
    ($value:tt) => {
        impl Into<$value>
    };
}

#[macro_export]
macro_rules! ArcClone {
    ($($var:tt),+; $($function:expr);+;) => {
        {
            use std::sync::Arc;
            use std::any::type_name;
            $(assert_eq!(type_name::<Arc<_>>(), type_name::<$var>(), "ArcClone only works on Arc types!");)+
            $(let $var: Arc<_> = Arc::clone(&$var);)+
            $($function);*
        }
    }
}

// TODO: Turn this into a proc macro with texture and sampler support
#[macro_export]
macro_rules! impl_bind_group_state {
    ($bindgroup_label:expr, $struct_name:ident, $($uniform_label:expr, $uniform_name:ident: $uniform_type:ty, $location:expr),* $(,)?) => {
        #[profiling::all_functions]
        impl $crate::gfx::BindGroupState for $struct_name {
            fn get_name(&self) -> &'static str {
                return $bindgroup_label;
            }

            fn get_layout_descriptor(&self) -> wgpu::BindGroupLayoutDescriptor {
                // Borrow checker doesnt like when this isnt a static
                static ENTRIES: &[wgpu::BindGroupLayoutEntry] = &[$(
                    wgpu::BindGroupLayoutEntry {
                        binding: $location,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            has_dynamic_offset: false,
                            ty: wgpu::BufferBindingType::Uniform,
                            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<$uniform_type>() as u64),
                        },
                        count: None,
                    },)*
                ];
                wgpu::BindGroupLayoutDescriptor {
                    label: Some(concat!($bindgroup_label, " Layout")),
                    entries: ENTRIES,
                }
            }

            fn init(device: &wgpu::Device) -> Self {
                Self {
                    $($uniform_name: $crate::gfx::UniformBuffer::new(device, Some($uniform_label), <$uniform_type>::default()),)*
                }
            }

            fn create_bind_group(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some($bindgroup_label),
                    layout: layout,
                    entries: &[$(
                        wgpu::BindGroupEntry {
                            binding: $location,
                            resource: self.$uniform_name.as_entire_binding(),
                        },)*
                    ],
                })
            }

            fn update(&mut self, queue: &wgpu::Queue) {
                $(self.$uniform_name.update(queue);)*
            }
        }
    };
}
