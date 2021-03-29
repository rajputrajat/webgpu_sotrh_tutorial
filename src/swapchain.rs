use crate::buffers;
use anyhow::Result;
use buffers::{Vertex, INCICES, VERTICES};
use log::info;
use wgpu::{
    include_spirv,
    util::{BufferInitDescriptor, DeviceExt},
    BackendBit, BlendState, Buffer, BufferUsage, Color, ColorTargetState, ColorWrite,
    CommandEncoderDescriptor, CullMode, Device, DeviceDescriptor, Features, FragmentState,
    FrontFace, IndexFormat, Instance, Limits, MultisampleState, Operations,
    PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology,
    Queue, RenderPassColorAttachmentDescriptor, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, Surface, SwapChain, SwapChainDescriptor,
    TextureUsage, VertexState,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    window::Window,
};

struct BufferRelatedData {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
}

struct SpecificRender {
    render_pipeline: RenderPipeline,
    buffer_related: Option<BufferRelatedData>,
}

struct Renders {
    renders: Vec<SpecificRender>,
    current_render: usize,
}

impl Renders {
    fn next(&mut self) {
        self.current_render += 1;
        if self.renders.len() == self.current_render {
            self.current_render = 0;
        }
    }
}

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    render: Renders,
    pub size: PhysicalSize<u32>,
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
        let num_indices = INCICES.len() as u32;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        info!("{:?}", adapter.get_info());
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
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("index buffer"),
            contents: bytemuck::cast_slice(INCICES),
            usage: BufferUsage::INDEX,
        });
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render: Renders {
                renders: vec![
                    SpecificRender {
                        render_pipeline: challenge_render_pipeline,
                        buffer_related: None,
                    },
                    SpecificRender {
                        render_pipeline,
                        buffer_related: Some(BufferRelatedData {
                            vertex_buffer,
                            index_buffer,
                            num_indices,
                        }),
                    },
                ],
                current_render: 0,
            },
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
                    buffers: &[Vertex::desc()],
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
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.render.next();
                true
            }
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
            let render = &self.render.renders[self.render.current_render];
            render_pass.set_pipeline(&render.render_pipeline);
            if let Some(buf_related) = &render.buffer_related {
                render_pass.set_vertex_buffer(0, buf_related.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(buf_related.index_buffer.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..buf_related.num_indices, 0, 0..1);
            } else {
                render_pass.draw(0..3, 0..1);
            }
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
