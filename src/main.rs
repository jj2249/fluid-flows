mod sentinel;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
fn main() {
    let sentinel_surface = sentinel::surface::SentinelSurface::new();
    // physical device
    // logical device
    // queue creation

    // swapchain

    // render pass
    // frambuffers
    // vertex buffer
    // shaders
    // viewport
    // pipeline
    // command buffers

    // event loop
    sentinel_surface
        .event_loop
        // event loop takes a Fn as an argument, event_handler
        // here we use a closure to define it inline
        // the closure must be a move closure in order to capture each events environment
        // the event_handler fn dispatches each event to a control flow
        .run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        })
}
