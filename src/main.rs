extern crate captrs;

use winit::{
    dpi::{PhysicalPosition},
    event::{ElementState, ModifiersState, VirtualKeyCode, MouseButton},
    event_loop::EventLoop,
    window::WindowBuilder
};
use pixels::{Pixels, SurfaceTexture};
use captrs::{Capturer, Bgr8};

// local modules
mod graphics;
use graphics::draw_frame;
mod win_events_manager;
use win_events_manager::{manage, ManagedEvent};
mod data_struct;
use data_struct::Memory;

fn main() -> Result<(), &'static str> {
    let mut capturer = Capturer::new(0).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));  // Wait for capturer to correctly initialize
    capturer.capture_store_frame().unwrap();
    
    let mut memory = Memory::default();
    let mut modifiers = ModifiersState::default();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(memory.window_size)
        .with_position(memory.window_pos)
        .with_always_on_top(true)
        .with_title("Atom")
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        
        use ManagedEvent as E;
        match manage(event) {
            E::CloseRequested | E::Destroyed => control_flow.set_exit(),

            // Input events
            E::MouseInput { state, button } => {
                use ElementState::*;
                use MouseButton::*;
                match (state, button) {
                    (Pressed, Left) => {
                        let (cx, cy): (f64, f64) = memory.cur_pos.into();
                        let (width, height): (f64, f64) = window.inner_size().cast::<f64>().into();
                        if 0.0 < cx && cx < width && 0.0 < cy && cy < height {
                            memory.is_grabbed = true;
                            memory.grab_pos = memory.cur_pos;
                        }
                    },
                    (Released, Left) => memory.is_grabbed = false,
                    (Pressed, Middle) => (),
                    (Released, Middle) => (),
                    (Pressed, Right) => (),
                    (Released, Right) => (),
                    _ => (),
                }
            },
            E::CursorEntered => memory.is_cur_inside = true,
            E::CursorLeft => memory.is_cur_inside = false,
            E::CursorMoved(curpos) => {
                memory.cur_pos = curpos;
                if memory.is_grabbed {
                    let (cx, cy): (f64, f64) = memory.cur_pos.into();
                    let (gx, gy): (f64, f64) = memory.grab_pos.into();
                    let (wx, wy): (f64, f64) = window.outer_position().unwrap().cast::<f64>().into();
                    window.set_outer_position(PhysicalPosition::new(wx + (cx-gx), wy + (cy-gy)));
                }
            },
            E::ModifiersChanged(m) => modifiers = m,
            E::KeyboardInput { state, vk } => {
                use ElementState::*;
                use VirtualKeyCode::*;
                match (state, vk) {
                    (Pressed, Escape) => control_flow.set_exit(),
                    (Pressed, Q) if modifiers.shift() => control_flow.set_exit(),
                    _ => ()
                };
            },

            E::ReceivedCharacter(..) => (),
            E::MouseMotion(..) | E::Button { .. } => (),
            E::MouseWheelLines(..) | E::MouseWheelPixels(..) => (),

            // Graphics-related events
            E::MainEventsCleared => (),
            E::RedrawEventsCleared => (),
            E::RedrawRequested => {
                memory.frame_count = (memory.frame_count + 1) % 64;
                match memory.frame_count {
                    0 => pixels.get_frame().fill(0),  // Clear window
                    1 | 2 => (),
                    3 => (),
                    4 => (),
                    5 => {
                        match capturer.capture_store_frame() { _ => () };
                        draw_frame(pixels.get_frame(), &capturer, &memory)
                    },
                    _ => draw_frame(pixels.get_frame(), &capturer, &memory)
                };
                match pixels.render() {
                    Ok(()) => (),
                    Err(_) => control_flow.set_exit()
                };
            },
            E::Moved(pos) => memory.window_pos = pos,
            E::Resized(size) => memory.window_size = size.cast::<i32>(),

            // Ignored events
            E::Resumed | E::Suspended | E::LoopDestroyed => (),
            E::Ignored => (),
            E::NotImplemented => ()
        }
        window.request_redraw();
    });
}
