use std::sync::Arc;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct SentinelSurface {
    instance: Arc<Instance>,
    pub event_loop: EventLoop<()>,
    window: Arc<Window>,
    surface: Arc<Surface>,
}

impl SentinelSurface {
    pub fn new() -> SentinelSurface {
        // an instance is the entry point to the Vulkan api for the program
        // represents an initialised vulkan library
        // load the default Vulkan library
        let library = vulkano::VulkanLibrary::new().expect("no local Vulkan lib/DLL found");

        let event_loop = EventLoop::new();
        // from docs: Returns the instance extensions required to create a surface from a window of the given event loop
        let required_extensions = Surface::required_extensions(&event_loop);

        // create the instance with the required extension
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create an instance of vulkan");

        let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

        SentinelSurface {
            instance,
            event_loop,
            window,
            surface,
        }
    }
}
