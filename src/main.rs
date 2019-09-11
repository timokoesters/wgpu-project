use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Clone, Copy)]
struct Particle {
    pos: [f32; 3],
    vel: [f32; 3],
}

fn main() {
    run(vec![
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [1.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
        Particle {
            pos: [10.0, 0.0, 2.0],
            vel: [1.0, 0.0, 2.0],
        },
    ]);
}

fn run(particles: Vec<Particle>) {
    env_logger::init();
    let event_loop = EventLoop::new();

    #[cfg(not(feature = "gl"))]
    let (_window, instance, size, surface) = {
        use raw_window_handle::HasRawWindowHandle as _;

        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size().to_physical(window.hidpi_factor());

        let instance = wgpu::Instance::new();
        let surface = instance.create_surface(window.raw_window_handle());

        (window, instance, size, surface)
    };

    #[cfg(feature = "gl")]
    let (_window, instance, size, surface) = {
        let wb = winit::WindowBuilder::new();
        let cb = wgpu::glutin::ContextBuilder::new().with_vsync(true);
        let context = cb.build_windowed(wb, &event_loop).unwrap();

        let size = context
            .window()
            .get_inner_size()
            .unwrap()
            .to_physical(context.window().get_hidpi_factor());

        let (context, window) = unsafe { context.make_current().unwrap().split() };

        let instance = wgpu::Instance::new(context);
        let surface = instance.get_surface();

        (window, instance, size, surface)
    };

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
    });

    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    // Load vertex shader
    let vs = include_str!("shader.vert");
    let vs_module = device.create_shader_module(
        &wgpu::read_spirv(glsl_to_spirv::compile(vs, glsl_to_spirv::ShaderType::Vertex).unwrap())
            .unwrap(),
    );

    // Load fragment shader
    let fs = include_str!("shader.frag");
    let fs_module = device.create_shader_module(
        &wgpu::read_spirv(glsl_to_spirv::compile(fs, glsl_to_spirv::ShaderType::Fragment).unwrap())
            .unwrap(),
    );

    let particle_buf = device
        .create_buffer_mapped(
            particles.len(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        )
        .fill_from_slice(&particles);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[wgpu::BindGroupLayoutBinding {
            binding: 0,
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
        }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[
            // Pass particle data to vertex shader
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &particle_buf,
                    range: 0..particles.len() as u64,
                },
            },
        ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::PointList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Particle>() as u64,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                // Pos
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float3,
                    offset: 0,
                    shader_location: 0,
                },
                // Vel
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float3,
                    // 3 coordinate components with 4 bytes each
                    offset: 3 * 4,
                    shader_location: 1,
                },
            ],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        },
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            event::Event::WindowEvent { event, .. } => match event {
                event::WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            event::Event::EventsCleared => {
                let frame = swap_chain.get_next_texture();
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::BLACK,
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_vertex_buffers(0, &[(&particle_buf, 0)]);
                    rpass.draw(0..10, 0..1);
                }

                device.get_queue().submit(&[encoder.finish()]);
            }
            _ => (),
        }
    });
}
