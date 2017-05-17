use std::sync::{Arc};
use std::time::{Duration};

use cgmath::{Vector2};
use vulkano::command_buffer::{self, AutoCommandBufferBuilder, CommandBufferBuilder, DynamicState};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::framebuffer::{Framebuffer, RenderPass, Subpass, RenderPassDesc, RenderPassAbstract, FramebufferAbstract};
use vulkano::format::{ClearValue};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::{Swapchain, SurfaceTransform};
use vulkano::image::{SwapchainImage};
use vulkano::sync::{GpuFuture};
use vulkano_win::{self, VkSurfaceBuild, Window};
use winit::{EventsLoop, WindowBuilder, Event as WinitEvent, WindowEvent, ElementState, ScanCode, VirtualKeyCode, ModifiersState};

pub struct Target {
    // Winit window
    events_loop: EventsLoop,
    window: Window,

    // Persistent values needed for vulkan rendering
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    images: Vec<Arc<SwapchainImage>>,
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,

    // Submissions from previous frames
    submissions: Vec<Box<GpuFuture>>,

    // Generic data
    size: Vector2<u32>,
    focused: bool,
}

impl Target {
    pub fn init() -> Self {
        let size = Vector2::new(1280, 720);

        // Start by setting up the vulkano instance, this is a silo of vulkan that all our vulkan
        //  types will belong to
        let instance = {
            // Tell it we need at least the extensions vulkano-win needs
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).unwrap()
        };

        // Pick a GPU to use for rendering. We assume first device as the one to render with
        // TODO: Allow user to select in some way, perhaps through config
        let physical = PhysicalDevice::enumerate(&instance)
            .next().unwrap();
        println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

        // Set up the window we want to render to, along with an EventsLoop we can use to listen
        //  for input and other events happening to the window coming from the OS
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_dimensions(size.x, size.y)
            .with_title(format!("Cobalt"))
            .build_vk_surface(&events_loop, &instance)
            .unwrap();

        // Find a GPU graphics queue family, we later create a queue from this family to talk to
        //  the GPU
        let graphics_queue_family = physical.queue_families().find(|q| {
            // The queue needs to support graphics (of course) and needs to support drawing to
            //  the previously created window's surface
            q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
        }).unwrap();

        // Finally, we create our actual connection with the GPU. We need a "device", which
        //  represents the connection between our program and the device, and queues, which we use
        //  to issue rendering commands to the GPU
        let (device, mut queues) = {
            // We need to request features explicitly, we need at least the swap chain
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                .. DeviceExtensions::none()
            };

            // Create the actual device
            Device::new(
                &physical, physical.supported_features(), &device_ext,
                // Pass which queues we want, we want one single graphics queue, the priority
                //  doesn't really matter to us since there's only one
                [(graphics_queue_family, 0.5)].iter().cloned()
            ).unwrap()
        };

        // Get the graphics queue we requested
        let graphics_queue = queues.next().unwrap();

        // Now create the swap chain, we need this to actually swap between our back buffer and the
        //  window's front buffer, without it we can't show anything
        let (swapchain, images) = {
            // Get what the swap chain we want to create would be capable of, we can't request
            //  anything it can't do
            let caps = window.surface().get_capabilities(&physical).unwrap();

            // The swap chain's dimensions need to match the window size
            let dimensions = caps.current_extent.unwrap_or([size.x, size.y]);

            // The present mode is things like vsync and vsync-framerate, right now pick the first
            //  one, we're sure it will work but it's probably not optimal
            // TODO: Let the user decide
            let present = caps.present_modes.iter().next().unwrap();

            // This decides how alpha will be composited by the OS' window manager, we just pick
            //  the first available option
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();

            // And finally, chose the internal format that images will have, we're picking the
            //  first available format again
            let format = caps.supported_formats[0].0;

            // Finally, actually create the swap chain
            Swapchain::new(
                &device, &window.surface(), caps.min_image_count, format, dimensions, 1,
                &caps.supported_usage_flags, &graphics_queue, SurfaceTransform::Identity, alpha,
                present, true, None
            ).unwrap()
        };

        // Set up a render pass TODO: Comment better
        #[allow(dead_code)]
        let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: images[0].format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap());

        // Set up the frame buffers matching the render pass TODO: Comment better
        let framebuffers = images.iter().map(|image| {
            let attachments = render_pass.desc().start_attachments().color(image.clone());
            let dimensions = [image.dimensions()[0], image.dimensions()[1], 1];
            Framebuffer::new(render_pass.clone(), dimensions, attachments).unwrap() as Arc<FramebufferAbstract + Send + Sync>
        }).collect::<Vec<_>>();

        Target {
            events_loop,
            window,

            device,
            graphics_queue,
            images,
            swapchain,
            render_pass,
            framebuffers,

            submissions: Vec::new(),

            size,
            focused: true,
        }
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        let mut event = Vec::new();
        let focused = &mut self.focused;

        self.events_loop.poll_events(|ev| {
            match ev {
                WinitEvent::WindowEvent { event: ev, .. } => {
                    match ev {
                        WindowEvent::Closed => event.push(Event::Closed),
                        WindowEvent::Focused(efocused) => *focused = efocused,
                        WindowEvent::KeyboardInput(state, scan_code, key_code, modifiers) =>
                            event.push(
                                Event::KeyboardInput(state, scan_code, key_code, modifiers)
                            ),
                        WindowEvent::MouseMoved(x, y) =>
                            if *focused {
                                event.push(Event::MouseMoved(Vector2::new(x as u32, y as u32)))
                            },
                        _ => (),
                    }
                },
            }
        });

        event
    }

    pub fn start_frame(&mut self) -> Frame {
        // Clearing the old submissions by keeping alive only the ones which probably aren't
        //  finished
        while self.submissions.len() >= 4 {
            self.submissions.remove(0);
        }

        // Get the image for this frame
        let (image_num, future) = self.swapchain.acquire_next_image(Duration::new(1, 0)).unwrap();

        let clear_values = vec!(ClearValue::Float([0.0, 0.0, 1.0, 1.0]));

        // Create the command buffer for this frame, this will hold all the draw calls and we'll
        //  submit them all at once
        let command_buffer_builder = AutoCommandBufferBuilder::new(
                self.device.clone(), self.graphics_queue.family()
            ).unwrap()
            // We immediately start with a render pass
            .begin_render_pass(
                self.framebuffers[image_num].clone(), false,
                clear_values
            ).unwrap();

        //.draw(pipeline.clone(), DynamicState::none(), vertex_buffer.clone(), (), ())
        //.unwrap()

        Frame {
            command_buffer_builder,
            image_num,
            future: Box::new(future),
        }
    }

    pub fn finish_frame(&mut self, frame: Frame) {
        // End the render pass and finish the command buffer
        let command_buffer = frame.command_buffer_builder
            .end_render_pass().unwrap()
            .build().unwrap();

        // TODO: ???
        let future = frame.future
            .then_execute(self.graphics_queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(self.graphics_queue.clone(), self.swapchain.clone(), frame.image_num)
            .then_signal_fence_and_flush().unwrap();
        self.submissions.push(Box::new(future));
    }

    pub fn set_cursor_position(&self, position: Vector2<u32>) {
        self.window.window()
            .set_cursor_position(position.x as i32, position.y as i32)
            .unwrap();
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    pub fn images(&self) -> &Vec<Arc<SwapchainImage>> {
        &self.images
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }
}

#[derive(Debug)]
pub enum Event {
    Closed,
    KeyboardInput(ElementState, ScanCode, Option<VirtualKeyCode>, ModifiersState),
    MouseMoved(Vector2<u32>),
}

pub struct Frame {
    command_buffer_builder: AutoCommandBufferBuilder,
    image_num: usize,
    future: Box<GpuFuture>,
}
