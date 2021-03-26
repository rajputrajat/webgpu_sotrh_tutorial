use anyhow::Result;
use wgpu::{Device, Queue, Surface, SwapChain, SwapChainDescriptor, SwapChainError};
use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    size: PhysicalSize<u32>,
}

impl State {
    async fn new(window: &Window) -> Self {
        todo!();
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        todo!();
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!();
    }

    fn update(&mut self) {
        todo!();
    }

    fn render(&mut self) -> Result<SwapChainError> {
        todo!();
    }
}
