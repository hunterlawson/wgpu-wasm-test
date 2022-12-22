mod state;

use crate::state::*;
use winit::{dpi::PhysicalSize, event::*, event_loop::EventLoop, window::WindowBuilder};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    // Create the event loop that will handle window events
    let event_loop = EventLoop::new();

    // Build and create the window and pass in the event loop
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1000, 600))
        .with_resizable(false)
        .with_title("WGPU Test Application")
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        //window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

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
