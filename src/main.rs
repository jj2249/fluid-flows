mod sentinel;
use crate::sentinel::instance;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    // instance
    instance::create_vulkan_instance(&event_loop);

    // surface

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
}
