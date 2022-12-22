mod state;

use crate::state::*;
use winit::{dpi::PhysicalSize, event::*, event_loop::EventLoop, window::WindowBuilder};

pub async fn run() {
    env_logger::init();

    // Create the event loop that will handle window events
    let event_loop = EventLoop::new();

    // Build and create the window and pass in the event loop
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .with_title("WGPU Test Application")
        .build(&event_loop)
        .unwrap();

    // Initialize WGPU and attach it to our window
    let mut state = State::new(&window).await;

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            // Handle window events
            Event::WindowEvent { ref event, .. } => {
                // Input the event into the application state
                // If it isn't process continue handling it
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => control_flow.set_exit(),
                        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size)
                        }
                        _ => (),
                    }
                }
            }
            // Redraw the screen
            Event::RedrawRequested(_) => {
                state.update();

                match state.render() {
                    Ok(_) => (),
                    // Reconfigure if the surface is lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should quit
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                    // All other errors
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => (),
        }
    });
}
