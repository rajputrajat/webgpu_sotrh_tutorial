use anyhow::Result;
use wgpu::{
    BackendBit, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    Limits, Operations, PowerPreference, Queue, RenderPassColorAttachmentDescriptor,
    RenderPassDescriptor, RequestAdapterOptions, Surface, SwapChain, SwapChainDescriptor,
    TextureUsage,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{MouseScrollDelta, WindowEvent},
    window::Window,
};

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
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
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
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
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
