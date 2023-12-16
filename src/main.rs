mod sentinel;
use sentinel::engine;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

fn main() {
    let engine = engine::Engine::new();
    
    // render pass
    // frambuffers
    // vertex buffer
    // shaders
    // viewport
    // pipeline
    // command buffers

    // event loop
    engine
        .event_loop
        .run(move |event, _, control_flow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        })
}
