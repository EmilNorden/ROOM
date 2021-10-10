#![feature(seek_stream_len)]

use clap::{AppSettings, Clap};
use winit::event::{Event, WindowEvent, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::game_context::{GameContext, GameState, DemoState};
use crate::rendering::init_rendering;
use crate::rendering::patch::Patch;
use crate::rendering::renderer::Renderer;
use crate::system::System;
use crate::wad::{By, LumpStore, WadHeader};
use crate::events::EventSystem;
use crate::events::Event::{KeyDown, KeyUp};
use crate::types::real;
use std::ops::BitAnd;

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

fn main() {
    env_logger::init();
    let mut lumps = LumpStore::new();
    lumps.add_file("/Users/emilnorden/doom/plutonia.wad");
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


    let system = System::new();

    let ndx = real(32);
    let ndy = real(-64);

    let dx = real(128);
    let dy = real(32);

    let r = ndy ^ ndx ^ dx ^ dy;

    // let render_data = init_rendering(&lumps);


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
