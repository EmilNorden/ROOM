// #![feature(seek_stream_len)]

use winit::event::{Event, WindowEvent, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::game_context::{GameContext};
use crate::rendering::patch::Patch;
use crate::rendering::renderer::Renderer;
use crate::system::System;
use crate::wad::{By, LumpStore, WadHeader};
use crate::events::EventSystem;
use crate::events::Event::{KeyDown, KeyUp};
use std::ops::BitAnd;
use crate::rendering::types::{Angle, Point2D};
use fixed::types::I16F16;
use byteorder::ReadBytesExt;
use fixed::prelude::ToFixed;

mod system;
mod wad;
mod rendering;
mod level;
mod types;
mod game_context;
mod drawer;
mod menu;
mod events;
mod game_component;
mod page_component;
mod options;
mod random;
mod graphics;
mod tables;
mod map_object;
mod constants;
mod number;

fn main() {
    let a = 123.0f32;
    let b : I16F16 = a.to_fixed();
    env_logger::init();
    let mut lumps = LumpStore::new();
    lumps.add_file("/Users/emilnorden/doom/plutonia.wad");
    //lumps.add_file("/home/emil/doom_wads/plutonia.wad");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("rust-doom")
        .with_inner_size(winit::dpi::LogicalSize::new(320, 200))
        .build(&event_loop).unwrap();
    // let mut render_state = pollster::block_on(RenderState::new(&window));
    let mut renderer = pollster::block_on(rendering::renderer::WGPURenderer::new(&window));
    let mut events = EventSystem::new();

    renderer.set_palette(lumps.get_lump(By::Name("PLAYPAL")));

    let mut game_context = GameContext::new(lumps);

    let asd = I16F16::from_bits(1);
    println!("{}", asd);
    let system = System::new();

    'game_loop: loop {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::RedrawRequested(_) => {
                    match renderer.render() {
                        Ok(_) => {}
                        Err(wgpu::SwapChainError::Lost) => renderer.recreate_swapchain(),
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (outdated, timeout) should be resolved by next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    game_context.game_loop(&mut events, &mut renderer, &system); // &lumps);
                    window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            renderer.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            match input.state
                            {
                                ElementState::Pressed =>
                                    events.post_event(KeyDown {
                                        scancode: input.scancode,
                                        virtual_keycode: input.virtual_keycode,
                                    }),
                                ElementState::Released =>
                                    events.post_event(KeyUp {
                                        scancode: input.scancode,
                                        virtual_keycode: input.virtual_keycode,
                                    }),
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}

// D_DoomLoop
fn doom_loop() {}
