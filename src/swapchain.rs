use anyhow::Result;
use wgpu::{
    include_spirv, BackendBit, BlendState, Color, ColorTargetState, ColorWrite,
    CommandEncoderDescriptor, CullMode, Device, DeviceDescriptor, Features, FragmentState,
    FrontFace, Instance, Limits, MultisampleState, Operations, PipelineLayoutDescriptor,
    PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology, Queue,
    RenderPassColorAttachmentDescriptor, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, Surface, SwapChain, SwapChainDescriptor,
    TextureUsage, VertexState,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    window::Window,
};

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    pub size: PhysicalSize<u32>,
    render_pipeline: RenderPipeline,
    challenge_render_pipeline: RenderPipeline,
    use_challenge: bool,
    game_local: GameLocal,
}

struct GameLocal {
    mouse_input: MouseInputs,
    color: Color,
}

struct MouseInputs {
    mouse_pointer_position: Option<PhysicalPosition<f64>>,
    scroll: Option<MouseScrollDelta>,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        println!("{:?}", adapter.get_info());
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        let sc_desc = SwapChainDescriptor {
            usage: TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        let (render_pipeline, challenge_render_pipeline) =
            State::create_render_pipeline(&device, &sc_desc);
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            challenge_render_pipeline,
            use_challenge: false,
            game_local: GameLocal {
                mouse_input: MouseInputs {
                    mouse_pointer_position: None,
                    scroll: None,
                },
                color: Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                },
            },
        }
    }

    fn create_render_pipeline(
        device: &Device,
        sc_desc: &SwapChainDescriptor,
    ) -> (RenderPipeline, RenderPipeline) {
        let render_pipeline;
        {
            let vs_module = device.create_shader_module(&include_spirv!("shader.vert.spv"));
            let fs_module = device.create_shader_module(&include_spirv!("shader.frag.spv"));
            // create pipeline layout
            let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
            // create render pipeline
            render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[ColorTargetState {
                        format: sc_desc.format,
                        alpha_blend: BlendState::REPLACE,
                        color_blend: BlendState::REPLACE,
                        write_mask: ColorWrite::ALL,
                    }],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: CullMode::Back,
                    polygon_mode: PolygonMode::Fill,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });
        }
        let challenge_render_pipeline;
        {
            let vs_module = device.create_shader_module(&include_spirv!("challenge.vert.spv"));
            let fs_module = device.create_shader_module(&include_spirv!("challenge.frag.spv"));
            // create pipeline layout
            let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
            // create render pipeline
            challenge_render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[ColorTargetState {
                        format: sc_desc.format,
                        alpha_blend: BlendState::REPLACE,
                        color_blend: BlendState::REPLACE,
                        write_mask: ColorWrite::ALL,
                    }],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: CullMode::Back,
                    polygon_mode: PolygonMode::Fill,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });
        }
        (render_pipeline, challenge_render_pipeline)
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => {
                self.game_local.mouse_input.scroll = Some(*delta);
                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.game_local.mouse_input.mouse_pointer_position = Some(*position);
                true
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    state: ElementState::Released,
                    ..
                } => {
                    self.use_challenge ^= true;
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn update(&mut self) {
        if let Some(pos) = self.game_local.mouse_input.mouse_pointer_position {
            self.game_local.color.r = pos.x / self.size.width as f64;
            self.game_local.color.g = pos.y / self.size.height as f64;
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: wgpu::LoadOp::Clear(self.game_local.color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            if self.use_challenge {
                render_pass.set_pipeline(&self.challenge_render_pipeline);
            } else {
                render_pass.set_pipeline(&self.render_pipeline);
            }
            render_pass.draw(0..3, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
