use std::sync::{Arc};

use cgmath::{Vector2};
use slog::{Logger};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::framebuffer::{FramebufferAbstract};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sync::{GpuFuture};

use error::{CobaltErrorMap};
use target_swapchain::{TargetSwapchain};
use {Error, Window, WindowCreator};

/// A representation of a render target, manages the initial connection with the drivers, and
/// presenting images on the target window.
pub struct Target {
    // Persistent values needed for vulkan rendering
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    target_swapchain: TargetSwapchain,

    // Queued up things we need to submit as part of command buffers
    queued_texture_copies: Vec<(
        Arc<CpuAccessibleBuffer<[u8]>>,
        Arc<ImmutableImage<Format>>
    )>,

    // Generic data
    size: Vector2<u32>,
}

impl Target {
    pub fn new<W: WindowCreator>(log: &Logger, window_creator: W) -> Result<(Self, W::W), Error> {
        info!(log, "Initializing target"; "backend" => "vulkan");
        let size = Vector2::new(1280, 720);

        // Start by setting up the vulkano instance, this is a silo of vulkan that all our vulkan
        //  types will belong to
        debug!(log, "Creating Vulkan instance");
        let instance = {
            // Tell it we need at least the extensions vulkano-win needs
            Instance::new(None, &window_creator.required_extensions(), None)
                .map_platform_err()?
        };

        // Pick a GPU to use for rendering. We assume first device as the one to render with
        // TODO: Allow user to select in some way, perhaps through config
        debug!(log, "Finding target physical device");
        let physical = PhysicalDevice::enumerate(&instance).next()
            .ok_or_else(|| Error::Platform("No physical devices found".into()))?;
        debug!(log, "Found physical device";
            "device" => physical.name(), "type" => format!("{:?}", physical.ty())
        );

        // Set up the window we want to render to, along with an EventsLoop we can use to listen
        //  for input and other events happening to the window coming from the OS
        debug!(log, "Creating window");
        let window = window_creator.create_window(instance.clone(), size);

        // Find a GPU graphics queue family, we later create a queue from this family to talk to
        //  the GPU
        debug!(log, "Finding graphics queue family with required features");
        let graphics_queue_family = physical.queue_families().find(|q| {
            // The queue needs to support graphics (of course) and needs to support drawing to
            //  the previously created window's surface
            q.supports_graphics() && window.surface().is_supported(*q).unwrap_or(false)
        }).unwrap();

        // Finally, we create our actual connection with the GPU. We need a "device", which
        //  represents the connection between our program and the device, and queues, which we use
        //  to issue rendering commands to the GPU
        debug!(log, "Creating logical device and queues");
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

        // Create the swapchain we'll have to render to to make things actually show up on screen
        let target_swapchain = TargetSwapchain::new(
            log, &window, size, physical, device.clone(), &graphics_queue
        );

        Ok((Target {
            device,
            graphics_queue,
            target_swapchain,

            queued_texture_copies: Vec::new(),

            size,
        }, window))
    }

    pub fn start_frame(&mut self) -> Frame {
        self.target_swapchain.clean_old_submissions();

        // Get the image for this frame, along with a future that will let us queue up the order of
        //  command buffer submissions.
        let (framebuffer, image_num, mut future) = self.target_swapchain.start_frame();

        // If we have any images to load, we need to submit another buffer before anything else
        if self.queued_texture_copies.len() != 0 {
            // Create a command buffer to upload the textures with
            let mut image_copy_buffer_builder = AutoCommandBufferBuilder::new(
                self.device.clone(), self.graphics_queue.family()
            ).unwrap();

            // Add any textures we need to upload to the command buffer
            while let Some(val) = self.queued_texture_copies.pop() {
                image_copy_buffer_builder = image_copy_buffer_builder
                    .copy_buffer_to_image(val.0, val.1)
                    .unwrap();
            }

            // Add the command buffer to the future so it will be executed
            let image_copy_buffer = image_copy_buffer_builder.build().unwrap();
            future = Box::new(future
                .then_execute(self.graphics_queue.clone(), image_copy_buffer).unwrap()
            );
        }

        Frame {
            framebuffer,
            image_num,
            future: Some(future),
        }
    }

    pub fn finish_frame(&mut self, frame: Frame) {
        self.target_swapchain.finish_frame(
            frame.future.unwrap(), self.graphics_queue.clone(), frame.image_num
        );
    }

    pub fn queue_texture_copy(
        &mut self,
        buffer: Arc<CpuAccessibleBuffer<[u8]>>,
        texture: Arc<ImmutableImage<Format>>,
    ) {
        self.queued_texture_copies.push((buffer, texture));
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    pub fn swapchain(&self) -> &TargetSwapchain {
        &self.target_swapchain
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }
}

pub struct Frame {
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    image_num: usize,
    pub future: Option<Box<GpuFuture>>,
}
