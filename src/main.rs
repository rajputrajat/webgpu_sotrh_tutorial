// use env_logger;
use futures::executor::block_on;
use wgpu::{self, SwapChainError};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod buffers;
mod swapchain;

fn main() {
    // env_logger::init();
    let e_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&e_loop).unwrap();
    let mut state = block_on(swapchain::State::new(&window));

    e_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size)
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(e) => match e.downcast_ref::<SwapChainError>() {
                    Some(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    Some(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Some(e) => eprintln!("{:?}", e),
                    None => {}
                },
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
