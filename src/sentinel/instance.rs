use std::sync::Arc;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use winit::event_loop::EventLoop;

// in instance is the entry point to the Vulkan api for the program
// represents an initialised vulkan library
pub fn create_vulkan_instance(event_loop: &EventLoop<()>) -> Arc<Instance> {
    // load the default Vulkan library
    let library = vulkano::VulkanLibrary::new().expect("no local Vulkan lib/DLL found");
    // from docs: Returns the instance extensions required to create a surface from a window of the given event loop
    let required_extensions = Surface::required_extensions(event_loop);

    // create the instance with the required extension
    Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create an instance of vulkan")
}