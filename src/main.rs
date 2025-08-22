mod gfx;
mod rend;
mod math;
mod asset;
mod macros;

use log::Level;
use asset::Image;
use std::sync::Arc;
use math::Transform;
use gfx::ShaderStage;
use futures_lite::future::block_on;
use rend::{CameraDescriptor, GlobalBindGroup, Model};
use window::{Key, Action, Window, WindowBackend, WindowEvent};

#[global_allocator]
#[cfg(feature = "profiling")]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> = tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

fn main() {
    logger::init().unwrap();
    logger::set_crate_log("wgpu", Level::Warn);
    logger::set_crate_log("naga", Level::Error);
    logger::set_crate_log("wgpu_hal", Level::Warn);
    logger::set_crate_log("wgpu_core", Level::Warn);

    if std::env::var("RUNNER").is_err() {
        std::env::set_var("RUNNER", "gfx-ne");
        log::warn!("No RUNNER environment variable set, defaulting to gfx-ne");
    }
    log::info!("Running with runner: {}", std::env::var("RUNNER").unwrap());

    match std::env::var("RUNNER").unwrap().to_lowercase().as_str() {
        "gfx" => {
            block_on(wgpu_test());
        }
        "gfx-ne" => {
            block_on(gfx_ne_test());
        }
        _ => {
            log::error!("Unknown runner specified, please set RUNNER to either 'gfx' or 'gfx-ne'");
            return;
        }
    }
}


struct FlyCamera {
    pub speed: f32,
    pub sensitivity: f32,
    pub camera: rend::Camera,
}

impl FlyCamera {
    pub fn new(camera: rend::Camera, speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            camera,
            sensitivity,
        }
    }

    pub fn update(&mut self, ctx: &mut gfx::RenderContext, window: &Window) {
        let mut direction = glam::Vec3::ZERO;
        if window.key_pressed(Key::W) {
            direction.z += 1.0;
        }
        if window.key_pressed(Key::S) {
            direction.z -= 1.0;
        }
        if window.key_pressed(Key::A) {
            direction.x -= 1.0;
        }
        if window.key_pressed(Key::D) {
            direction.x += 1.0;
        }
        if window.key_pressed(Key::Space) {
            direction.y += 1.0;
        }
        if window.key_pressed(Key::LShift) {
            direction.y -= 1.0;
        }

        let (dx, dy) = window.mouse_delta();
        let yaw = glam::Quat::from_rotation_y(dx * self.sensitivity);
        let pitch = glam::Quat::from_rotation_x(dy * self.sensitivity);
        self.camera.transform.rotation = yaw * self.camera.transform.rotation * pitch;
        self.camera.transform.translation += self.camera.transform.rotation * direction * self.speed;
        self.camera.update(&ctx.queue);
    }
}

// FIXME: consider combining render textures with textures, or have a trait for anything with a view
async fn create_post_pipeline(context: &mut gfx::RenderContext, target: &gfx::RenderTexture) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    let shader = context.create_shader("shaders/post.wgsl");

    let layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Post Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Post Bind Group"),
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&target.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&target.sampler.as_raw()),
            },
        ],
    });

    let pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Post pipeline"),
        cache: None,
        layout: Some!(&context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        })),
        vertex: wgpu::VertexState {
            buffers: &[],
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Vertex).unwrap()),
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
        },
        multisample: wgpu::MultisampleState{
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        fragment: Some(wgpu::FragmentState {
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Fragment).unwrap()),
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
            targets: &[Some(wgpu::ColorTargetState {
                format: context.config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        operation: wgpu::BlendOperation::Add,
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        depth_stencil: None,
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
    });

    return (pipeline, bind_group)
}

// TODO: not this make abstraction
async fn wgpu_test() {
    let mut window = Window::new("Pregen: Runtime", 800, 800, true, WindowBackend::preferred());
    let mut context = gfx::RenderContext::new(&mut window).await;
    let mut target = context.create_render_target(Some("Test Target"), gfx::SamplerMode::CLAMP, context.config.width, context.config.height);
    dbg!(&target);
    let image = Arc::new(Image::from_raw(include_bytes!("test.jpg").to_vec(), 5));
    let texture = context.create_texture(Some("Test Texture"), gfx::SamplerMode::REPEAT, gfx::TextureFormat::Rgba8Unorm, image);
    dbg!(&texture);
    let mut depth_texture = context.create_depth_texture(Some("Depth Texture"), gfx::SamplerMode::CLAMP, context.config.width, context.config.height);
    dbg!(&depth_texture);
    let mesh = gfx::Geometry::from_primitive(&context.device, Some("Test Mesh"), &gfx::Geometry::PYRAMID);
    dbg!(&mesh);

    let shader = context.create_shader("shaders/default.wgsl");
    dbg!(&shader);
    let mut camera = rend::Camera::new(&mut context, rend::CameraProjection::Perspective, CameraDescriptor {
        aspect_ratio: window.get_aspect_ratio(),
        z_near: 0.001,
        ..Default::default()
    });
    camera.transform.translation.z = 3.0;
    camera.look_at((0.0, 0.0, 0.0).into());
    let mut camera = FlyCamera::new(camera, 0.001, 0.001);
    let model = Model::from_path(&mut context, Some("Backpack"), "backpack/backpack.obj", Transform::default());
    let mut frame_bind = context.create_bind_group::<GlobalBindGroup>(None);
    let lights = (0..8).map(|_| {
        rend::LightingUniform {
            _padding: 0.0,
            position: glam::Vec3::new(
                rand::random::<f32>() * 10.0 - 5.0,
                rand::random::<f32>() * 10.0 - 5.0,
                rand::random::<f32>() * 10.0 - 5.0
            ),
            intensity: rand::random::<f32>().abs(),
            color: glam::Vec3::new(rand::random::<f32>().abs(), rand::random::<f32>().abs(), rand::random::<f32>().abs()),
        }
    }).collect::<Vec<_>>();
    frame_bind.u_lights.set(lights.try_into().expect("Expected exactly 8 lights"));
    let pipeline = context.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        cache: None,
        layout: Some(&context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&frame_bind.layout, &camera.camera.group.layout, &model.group.layout, &model.meshes[0].material.group.layout],
        })),
        vertex: wgpu::VertexState {
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Vertex).unwrap()),
            buffers: &[<rend::Vertex as gfx::VertexArrayObject>::VERTEX_BUFFER_LAYOUT],
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
        },
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        fragment: Some(wgpu::FragmentState {
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Fragment).unwrap()),
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
            targets: &[Some(wgpu::ColorTargetState {
                format: context.config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        operation: wgpu::BlendOperation::Add,
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    },
                    alpha: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::One,
                        operation: wgpu::BlendOperation::Add,
                    },
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        depth_stencil: Some(wgpu::DepthStencilState {
            bias: Default::default(),
            depth_write_enabled: true,
            stencil: Default::default(),
            format: wgpu::TextureFormat::Depth32Float,
            depth_compare: wgpu::CompareFunction::Less,
        }),
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None!(wgpu::Face::Back),
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
    });

    let mut frame: f32 = 0.0;
    dbg!(&model.transform);
    let mut capture_mouse = false;
    dbg!(&camera.camera.transform);
    camera.update(&mut context, &window);
    let (mut post_pipeline, mut post_bind_group) = create_post_pipeline(&mut context, &target).await;
    while !window.should_close() {
        profiling::finish_frame!();

        for event in window.poll() {
            match event {
                WindowEvent::FramebufferResize { width, height } => {
                    if (width, height) != (0, 0) {
                        context.config.width = width;
                        context.config.height = height;
                        log::info!("Resizing to {}x{}", width, height);
                        camera.camera.aspect_ratio = window.get_aspect_ratio();
                        context.surface.configure(&context.device, &context.config);
                        target.resize(&context.device, context.config.width, context.config.height);
                        depth_texture.resize(&context.device, context.config.width, context.config.height);
                        (post_pipeline, post_bind_group) = create_post_pipeline(&mut context, &target).await;
                    }
                }
                WindowEvent::KeyboardInput(Key::Escape, _, Action::Pressed) => {
                    window.set_should_close(true);
                }
                WindowEvent::KeyboardInput(Key::P, _, Action::Pressed) => {
                    shader.reload(&context.device);
                    match camera.camera.projection {
                        rend::CameraProjection::Perspective => {
                            camera.camera.projection = rend::CameraProjection::Orthographic;
                        }
                        rend::CameraProjection::Orthographic => {
                            camera.camera.projection = rend::CameraProjection::Perspective;
                        }
                    }
                }
                WindowEvent::KeyboardInput(Key::Tab, _, Action::Pressed) => {
                    capture_mouse = !capture_mouse;
                    window.lock_cursor(capture_mouse);
                }
                WindowEvent::KeyboardInput(Key::Minus, _, Action::Pressed) => {
                    camera.camera.fov -= 1.0;
                }
                WindowEvent::KeyboardInput(Key::Equals, _, Action::Pressed) => {
                    camera.camera.fov += 1.0;
                }
                _ => {
                    if let WindowEvent::CursorPosition { .. } = event {
                        continue;
                    }
                    if let WindowEvent::Resize { .. } = event {
                        continue;
                    }
                    // if let WindowEvent::KeyboardInput { .. } = event {
                    //     continue;
                    // }
                    dbg!(&event);
                }
            }
        }

        frame_bind.u_time.set(frame);
        frame_bind.update(&context.queue);
        // camera.camera.transform.translation.x = frame.cos() * 3.0;
        // camera.camera.transform.translation.z = frame.sin() * 3.0;
        // camera.camera.look_at((0.0, 0.0, 0.0).into());
        if capture_mouse {
            camera.update(&mut context, &window);
            window.set_cursor_position(context.config.width / 2, context.config.height / 2);
        }
        let swapchain = match context.surface.get_current_texture() {
            Ok(swapchain) => swapchain,
            // In theory this should never happen unless the surface is changed but not reconfigured
            // But for some reason under xwayland the third frame invalidates the surface????
            Err(_) => {
                log::info!("Reconfiguring surface");
                context.surface.configure(&context.device, &context.config);
                continue;
            }
        };

        let swap_view = swapchain.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None
        });
        // Object rendering
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        store: wgpu::StoreOp::Store,
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        store: wgpu::StoreOp::Store,
                        load: wgpu::LoadOp::Clear(1.0),
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            rpass.set_pipeline(&pipeline);
            rpass.set_bind_group(0, frame_bind.as_raw(), &[]);
            rpass.set_bind_group(2, model.group.as_raw(), &[]);
            rpass.set_bind_group(1, camera.camera.group.as_raw(), &[]);
            for mesh in &model.meshes {
                rpass.set_bind_group(3, mesh.material.group.as_raw(), &[]);
                rpass.set_vertex_buffer(0, mesh.geometry.vertex_buffer.slice(..));
                rpass.set_index_buffer(mesh.geometry.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                rpass.draw_indexed(mesh.geometry.range(), 0, 0..1);
            }
        }
        {
            // Post procceesing
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &swap_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        store: wgpu::StoreOp::Store,
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    }
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
            rpass.set_pipeline(&post_pipeline);
            rpass.set_bind_group(0, &post_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }
        context.queue.submit([encoder.finish()]);
        swapchain.present();
        frame += 0.001;
    };
}

