mod gfx;
mod rend;
mod math;
mod asset;
mod macros;

use wgpu as wg;
use asset::Image;
use std::sync::Arc;
use math::Transform;
use gfx::ShaderStage;
use futures_lite::future::block_on;
use rend::{CameraDescriptor, GlobalBindGroup, Model};
use window::{Key, Action, Window, WindowBackend, WindowEvent};

async fn create_post_pipeline(context: &mut gfx::RenderContext, target: &gfx::RenderTexture) -> (wgpu::RenderPipeline, wgpu::BindGroup) {
    context.device.push_error_scope(wg::ErrorFilter::Validation);
    let shader = context.create_shader("shaders/post.wgsl");
    let pap = context.device.pop_error_scope().await;
    dbg!(&pap);

    let layout = context.device.create_bind_group_layout(&wg::BindGroupLayoutDescriptor {
        label: Some("Post Bind Group Layout"),
        entries: &[
            wg::BindGroupLayoutEntry {
                binding: 0,
                visibility: wg::ShaderStages::FRAGMENT,
                ty: wg::BindingType::Texture {
                    sample_type: wg::TextureSampleType::Float { filterable: true },
                    view_dimension: wg::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wg::BindGroupLayoutEntry {
                binding: 1,
                visibility: wg::ShaderStages::FRAGMENT,
                ty: wg::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let bind_group = context.device.create_bind_group(&wg::BindGroupDescriptor {
        label: Some("Post Bind Group"),
        layout: &layout,
        entries: &[
            wg::BindGroupEntry {
                binding: 0,
                resource: wg::BindingResource::TextureView(&target.view),
            },
            wg::BindGroupEntry {
                binding: 1,
                resource: wg::BindingResource::Sampler(&target.sampler.as_raw()),
            },
        ],
    });

    let pipeline = context.device.create_render_pipeline(&wg::RenderPipelineDescriptor {
        label: Some("Post pipeline"),
        cache: None,
        layout: Some!(&context.device.create_pipeline_layout(&wg::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        })),
        vertex: wg::VertexState {
            buffers: &[],
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Vertex).unwrap()),
            compilation_options: wg::PipelineCompilationOptions {
                ..Default::default()
            },
        },
        multisample: wgpu::MultisampleState{
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        fragment: Some(wg::FragmentState {
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Fragment).unwrap()),
            compilation_options: wg::PipelineCompilationOptions {
                ..Default::default()
            },
            targets: &[Some(wg::ColorTargetState {
                format: context.config.format,
                blend: Some(wg::BlendState {
                    color: wg::BlendComponent {
                        operation: wg::BlendOperation::Add,
                        src_factor: wg::BlendFactor::SrcAlpha,
                        dst_factor: wg::BlendFactor::OneMinusSrcAlpha,
                    },
                    alpha: wg::BlendComponent {
                        src_factor: wg::BlendFactor::One,
                        dst_factor: wg::BlendFactor::One,
                        operation: wg::BlendOperation::Add,
                    },
                }),
                write_mask: wg::ColorWrites::ALL,
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

fn main() {
    // env_logger::init();
    // use winapi::um::winuser;
    logger::init().unwrap();
    logger::ignore_crate("wgpu");
    logger::ignore_crate("wgpu_hal");
    // let old_hook = std::panic::take_hook();
    // std::panic::set_hook(Box::new(move |info| {
    //     println!("{:?}", info);
    //     std::io::stdin().read_line(&mut String::new()).unwrap();
    //     old_hook(info);
    // }));
    // panic!("nya~");
    block_on(wgpu_test());
}

fn window_test() {
    let mut window = Window::new("Pregen: Runtime", 800, 800, true, WindowBackend::preferred());

    while !window.should_close() {
        for event in window.poll() {
            match event {
                WindowEvent::Resize { width, height } => {
                    println!("Resized to: {}x{}", width, height);
                }
                WindowEvent::KeyInput(key, scancode, action) => {
                    println!("Key: {:?} Scancode: {:?} Action: {:?}", key, scancode, action);
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }
    }
}

// TODO: not this make abstraction
async fn wgpu_test() {
    let mut window = Window::new("Pregen: Runtime", 800, 800, true, WindowBackend::preferred());
    let mut context = gfx::RenderContext::new(&mut window).await;
    let image = Arc::new(Image::from_raw(include_bytes!("test.jpg").to_vec(), 5));
    let model = Model::from_path(&mut context, Some("Backpack"), "backpack/backpack.obj", Transform::default());
    let mut target = context.create_render_target(Some("Test Target"), gfx::SamplerMode::CLAMP, context.config.width, context.config.height);
    dbg!(&target);
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
        ..Default::default()
    });
    let model = Model::from_path(&mut context, Some("Backpack"), "backpack/backpack.obj", Transform::default());
    let mut frame_bind = context.create_bind_group::<GlobalBindGroup>(None);
    let pipeline = context.device.create_render_pipeline(&wg::RenderPipelineDescriptor {
        label: Some("Render pipeline"),
        cache: None,
        layout: Some(&context.device.create_pipeline_layout(&wg::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[&frame_bind.layout, &camera.group.layout, &model.group.layout, &model.meshes[0].material.group.layout],
        })),
        vertex: wg::VertexState {
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Vertex).unwrap()),
            buffers: &[<rend::Vertex as gfx::VertexArrayObject>::VERTEX_BUFFER_LAYOUT],
            compilation_options: wg::PipelineCompilationOptions {
                ..Default::default()
            },
        },
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        fragment: Some(wg::FragmentState {
            module: &shader.as_raw(),
            entry_point: Some(shader.get_entry(ShaderStage::Fragment).unwrap()),
            compilation_options: wg::PipelineCompilationOptions {
                ..Default::default()
            },
            targets: &[Some(wg::ColorTargetState {
                format: context.config.format,
                blend: Some(wg::BlendState {
                    color: wg::BlendComponent {
                        operation: wg::BlendOperation::Add,
                        src_factor: wg::BlendFactor::SrcAlpha,
                        dst_factor: wg::BlendFactor::OneMinusSrcAlpha,
                    },
                    alpha: wg::BlendComponent {
                        src_factor: wg::BlendFactor::One,
                        dst_factor: wg::BlendFactor::One,
                        operation: wg::BlendOperation::Add,
                    },
                }),
                write_mask: wg::ColorWrites::ALL,
            })],
        }),
        depth_stencil: Some(wgpu::DepthStencilState {
            bias: Default::default(),
            depth_write_enabled: true,
            stencil: Default::default(),
            format: wg::TextureFormat::Depth32Float,
            depth_compare: wgpu::CompareFunction::Less,
        }),
        primitive: wgpu::PrimitiveState {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
    });

    let (mut post_pipeline, mut post_bind_group) = create_post_pipeline(&mut context, &target).await;
    let mut frame: f32 = 0.0;
    while !window.should_close() {
        profiling::finish_frame!();

        for event in window.poll() {
            match event {
                WindowEvent::Resize { width, height } => {
                    if (width, height) != (0, 0) {
                        context.config.width = width;
                        context.config.height = height;
                        camera.aspect_ratio = width as f32 / height as f32;
                        context.surface.configure(&context.device, &context.config);
                        target.resize(&context.device, context.config.width, context.config.height);
                        depth_texture.resize(&context.device, context.config.width, context.config.height);
                        (post_pipeline, post_bind_group) = create_post_pipeline(&mut context, &target).await;
                    }
                }
                WindowEvent::KeyInput(Key::Escape, _, Action::Pressed) => {
                    window.set_should_close(true);
                }
                WindowEvent::KeyInput(Key::Space, _, Action::Pressed) => {
                    shader.reload(&context.device);
                    match camera.projection {
                        rend::CameraProjection::Perspective => {
                            camera.projection = rend::CameraProjection::Orthographic;
                        }
                        rend::CameraProjection::Orthographic => {
                            camera.projection = rend::CameraProjection::Perspective;
                        }
                    }
                }
                WindowEvent::KeyInput(Key::Minus, _, Action::Pressed) => {
                    camera.fov -= 1.0;
                }
                WindowEvent::KeyInput(Key::Equals, _, Action::Pressed) => {
                    camera.fov += 1.0;
                }
                _ => {
                    // dbg!(&event);
                }
            }
        }

        frame_bind.u_time.set(frame);
        frame_bind.update(&context.queue);
        camera.transform.translation.x = frame.cos() * 3.0;
        camera.transform.translation.z = frame.sin() * 3.0;
        camera.look_at((0.0, 0.0, 0.0).into());
        camera.update(&context.queue);
        let swapchain = match context.surface.get_current_texture() {
            Ok(swapchain) => swapchain,
            // In theory this should never happen unless the surface is changed but not reconfigured
            // But for some reason under xwayland the third frame invalidates the surface????
            Err(_) => {
                // dbg!("Reconfiguring surface");
                context.surface.configure(&context.device, &context.config);
                context.surface.get_current_texture().unwrap()
            }
        };
        let swap_view = swapchain.texture.create_view(&wg::TextureViewDescriptor::default());
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
            rpass.set_bind_group(1, camera.group.as_raw(), &[]);
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
