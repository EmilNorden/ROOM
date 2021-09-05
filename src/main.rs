mod video;
mod texture;

use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};
use crate::video::RenderState;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rust-doom")
        .build(&event_loop).unwrap();
    let mut render_state = pollster::block_on(RenderState::new(&window));

    'game_loop: loop {
        event_loop.run(move |event, _, control_flow| {
           *control_flow = ControlFlow::Poll;

            match event {
                Event::RedrawRequested(_) => {
                    match render_state.render() {
                        Ok(_) => {},
                        Err(wgpu::SwapChainError::Lost) => render_state.recreate_swapchain(),
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (outdated, timeout) should be resolved by next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            render_state.resize(*physical_size)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            render_state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}
