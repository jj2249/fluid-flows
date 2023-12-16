use std::cmp::{max, min};
use std::sync::Arc;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::Queue;
use vulkano::device::QueueCreateInfo;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueFlags};
use vulkano::image::Image;
use vulkano::image::ImageUsage;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainCreateInfo;
use vulkano::VulkanLibrary;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
pub struct Engine {
    // limit the complexity of the interface by interacting through high level
    //  vulkan objects and getters
    pub event_loop: EventLoop<()>,
    swapchain: Arc<Swapchain>
}
impl Engine {
    pub fn new() -> Engine {
        // new event_loop
        let event_loop = EventLoop::new();
        let surface = _create_surface(&event_loop);

        // assert the device is swapchain compatible
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        // create device needs to know which surface it's drawing to so it can check compatibility
        let (device, mut queues) = _create_device(surface.clone(), &device_extensions);
        let queue = queues.next().unwrap();

        let (swapchain, images) = _create_swapchain(surface, device);
        Engine {
            event_loop,
            swapchain
        }
    }
}

fn _create_surface(event_loop: &EventLoop<()>) -> Arc<Surface> {
    // an instance is the entry point to the Vulkan api for the program
    // represents an initialised vulkan library
    // load the default Vulkan library
    let library = VulkanLibrary::new().expect("no local Vulkan lib/DLL found");

    // from docs: Returns the instance extensions required to create a surface from a window of the given event loop
    let required_extensions = Surface::required_extensions(event_loop);

    // create the instance with the required extension
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create an instance of vulkan");

    let window = Arc::new(WindowBuilder::new().build(event_loop).unwrap());

    Surface::from_window(instance.clone(), window.clone()).unwrap()
}

fn _select_physical_device(
    surface: Arc<Surface>,
    device_extensions: &DeviceExtensions,
) -> (Arc<PhysicalDevice>, u32) {
    surface
        .instance()
        // find all available physical devices
        .enumerate_physical_devices()
        .expect("no physical devices found")
        // only include devices which support the given extensions
        .filter(|p| p.supported_extensions().contains(device_extensions))
        .filter_map(|p| {
            // access QueueFlags for each device
            // each device p, has a list of queue flags, q
            p.queue_family_properties()
                .iter()
                .enumerate()
                // return the first index for which the closure returns true
                .position(|(i, q)| {
                    // check that the queue supports graphics and that the device
                    //   can draw to the surface through this queue
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                // return devices as a tuple of (device, graphics_queue)
                .map(|q| (p, q as u32))
        })
        // check the type of the device, return the best device based on the following preferences
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            _ => 4,
        })
        .expect("no matching device found")
}

fn _create_device(
    surface: Arc<Surface>,
    device_extensions: &DeviceExtensions,
) -> (Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>) {
    // find the best device and it's relevant queue family
    let (physical_device, queue_family_index) = _select_physical_device(surface, device_extensions);
    // construct device
    Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            // deref
            enabled_extensions: *device_extensions,
            ..Default::default()
        },
    )
    .expect("couldn't build device")
}

fn _create_swapchain(
    surface: Arc<Surface>,
    device: Arc<Device>,
) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
    let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .expect("could not retrieve surface capabilities");

    // tell the swapchain how big the images should be
    // will either be current window size or a base resolution -> 640 x 480
    let image_extent = surface_capabilities.current_extent.unwrap_or([640, 480]);

    let composite_alpha = surface_capabilities
        .supported_composite_alpha
        .into_iter()
        .next()
        .unwrap();

    // set the number of images in the buffer
    let min_image_count = match surface_capabilities.max_image_count {
        // if there's no upper limit then limit it to 2
        None => max(2, surface_capabilities.min_image_count),
        Some(limit) => min(max(2, surface_capabilities.min_image_count), limit),
    };

    // basic configuration
    let pre_transform = surface_capabilities.current_transform;
    let (image_format, colour_space) = device
        .physical_device()
        .surface_formats(&surface, Default::default())
        .unwrap()[0];

    Swapchain::new(
        device,
        surface,
        SwapchainCreateInfo {
            min_image_count,
            image_format,
            image_extent,
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            pre_transform,
            composite_alpha,
            ..Default::default()
        },
    )
    .expect("could not construct swapchain")
}
