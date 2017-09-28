use std::sync::{Arc};

use cgmath::{Vector2};
use slog::{Logger};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::sync::{NowFuture, GpuFuture};
use vulkano::command_buffer::{CommandBufferExecFuture, AutoCommandBuffer};
use vulkano::swapchain::{Surface};

use calcium_rendering::raw::{RawAccess, RendererRaw};
use calcium_rendering::{Error, Frame};

use {VulkanoTextureRaw, WindowSwapchain};

pub struct VulkanoRendererRaw {
    instance: Arc<Instance>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,

    size: Vector2<u32>,
    pub surface: Arc<Surface>,
    pub swapchain: WindowSwapchain,
    queued_resize: bool,
    next_frame_id: u64,

    queued_cb_futures: Vec<CommandBufferExecFuture<NowFuture, AutoCommandBuffer>>,
}

impl VulkanoRendererRaw {
    pub fn new(
        log: &Logger, instance: Arc<Instance>,
        surface: Arc<Surface>, size: Vector2<u32>,
    ) -> Result<Self, Error> {
        info!(log, "Creating vulkano renderer");

        // Pick a GPU to use for rendering. We assume first device as the one to render with
        // TODO: Allow user to select in some way, perhaps through config
        debug!(log, "Finding target physical device");
        let physical = PhysicalDevice::enumerate(&instance).next()
            .ok_or_else(|| Error::Platform("No physical devices found".into()))?;
        debug!(log, "Found physical device";
            "device" => physical.name(), "type" => format!("{:?}", physical.ty())
        );

        // Find a GPU graphics queue family that we want a queue of.
        // TODO: No checks are being made if the queue can render to the window surfaces, so far on
        //  my test machines this hasn't been a problem yet, but if this becomes a problem perahps
        //  create at least one queue of every graphics supported queue family and select the one
        //  appropriate for the window. (surface.is_supported(*q).unwrap_or(false))
        debug!(log, "Finding graphics queue family with required features");
        let graphics_queue_family = physical.queue_families().find(|q| {
            q.supports_graphics()
        }).expect("Unable to find graphics queue family");

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

            // Check the features we need are supported
            // TODO: Create a system for optional extensions so we can detect features
            let features = physical.supported_features();
            if !features.sampler_anisotropy {
                return Err(Error::Unsupported(
                    "Anisotropic filtering not supported by platform".into()
                ))
            }

            // Create the actual device
            Device::new(
                physical, features, &device_ext,
                // Pass which queues we want, we want one single graphics queue, the priority
                //  doesn't really matter to us since there's only one
                [(graphics_queue_family, 0.5)].iter().cloned()
            ).unwrap()
        };

        // Get the graphics queue we requested
        let graphics_queue = queues.next().unwrap();

        // Create the swapchain we'll have to render to to make things actually show up on screen
        let swapchain = WindowSwapchain::new(log, &device, &graphics_queue, &surface, size);

        Ok(VulkanoRendererRaw {
            instance: instance.clone(),
            device,
            graphics_queue,

            size,
            surface,
            swapchain,
            queued_resize: false,
            next_frame_id: 0,

            queued_cb_futures: Vec::new(),
        })
    }

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    pub fn queue_command_buffer_future(
        &mut self,
        future: CommandBufferExecFuture<NowFuture, AutoCommandBuffer>,
    ) {
        self.queued_cb_futures.push(future);
    }

    pub fn submit_queued_commands(
        &mut self, mut future: Box<GpuFuture + Send + Sync>
    ) -> Box<GpuFuture + Send + Sync> {
        // If we don't have anything to upload, we don't need to alter the future at all
        if self.queued_cb_futures.len() == 0 {
            return future;
        }

        // Join together any queued futures futures
        // TODO: Add functionality for concurrent or non-blocking uploading of textures
        while let Some(val) = self.queued_cb_futures.pop() {
            future = Box::new(future.join(val));
        }

        future
    }

    pub fn queue_resize(&mut self, size: Vector2<u32>) {
        // Limit to at least 1x1 in size, we crash otherwise.
        if size.x <= 0 || size.y <= 0 {
            return
        }

        // We can be spammed with resize events many times in the same frame, so defer changing the
        //  swapchain.
        self.queued_resize = true;

        // We do however want to immediately set the size value as it may be used for 2D geometry
        // location calculations, which would lag behind at least one frame like this if the
        // calculations are done before start_frame.
        self.size = size;
    }
}

impl RendererRaw for VulkanoRendererRaw {
    type FrameRaw = VulkanoFrameRaw;
    type TextureRaw = VulkanoTextureRaw;

    fn size(&self) -> Vector2<u32> {
        self.size
    }

    fn start_frame(&mut self) -> Frame<Self> {
        self.swapchain.cleanup_finished_frames();

        // Before we render, see if we need to execute a queued resize
        if self.queued_resize {
            // Overwrite the size with the actual size we were changed to
            self.size = self.swapchain.resize(self.size, &self.device, &self.surface);
            self.queued_resize = false;
        }

        // Get the image for this frame, along with a future that will let us queue up the order of
        //  command buffer submissions.
        let (image_num, future) = self.swapchain.start_frame();

        self.next_frame_id += 1;
        Frame::raw_new(VulkanoFrameRaw {
            image_num,
            future: Some(future),
            frame_id: self.next_frame_id - 1,
            size: self.size,
        })
    }

    fn finish_frame(&mut self, mut frame: Frame<Self>) {
        self.swapchain.finish_frame(
            frame.raw_mut().future.take().unwrap(),
            self.graphics_queue.clone(),
            frame.raw().image_num
        );
    }
}

pub struct VulkanoFrameRaw {
    pub image_num: usize,
    pub future: Option<Box<GpuFuture + Send + Sync>>,
    pub frame_id: u64,
    pub size: Vector2<u32>,
}
