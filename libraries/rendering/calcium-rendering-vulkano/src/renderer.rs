use std::sync::{Arc};

use slog::{Logger};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::instance::{Instance, PhysicalDevice, InstanceExtensions};
use vulkano::image::immutable::{ImmutableImage};
use vulkano::sync::{GpuFuture};
use vulkano::command_buffer::{AutoCommandBufferBuilder};

use calcium_rendering::{Error, CalciumErrorMappable, Renderer};

pub struct VulkanoRenderer {
    log: Logger,

    instance: Arc<Instance>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,

    // Queued up things we need to submit as part of command buffers
    queued_image_copies: Vec<(Arc<CpuAccessibleBuffer<[u8]>>, Arc<ImmutableImage<Format>>)>,
}

impl VulkanoRenderer {
    pub fn new(
        log: &Logger, required_extensions: InstanceExtensions,
    ) -> Result<Self, Error> {
        info!(log, "Creating vulkano renderer");

        // Start by setting up the vulkano instance, this is a silo of vulkan that all our vulkan
        //  types will belong to
        debug!(log, "Creating vulkan instance");
        let instance = {
            // Tell it we need at least the extensions vulkano-win needs
            Instance::new(None, &required_extensions, None)
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

            // Create the actual device
            Device::new(
                physical, physical.supported_features(), &device_ext,
                // Pass which queues we want, we want one single graphics queue, the priority
                //  doesn't really matter to us since there's only one
                [(graphics_queue_family, 0.5)].iter().cloned()
            ).unwrap()
        };

        // Get the graphics queue we requested
        let graphics_queue = queues.next().unwrap();

        Ok(VulkanoRenderer {
            log: log.clone(),

            instance: instance.clone(),
            device,
            graphics_queue,

            queued_image_copies: Vec::new(),
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

    pub fn queue_image_copy(
        &mut self,
        buffer: Arc<CpuAccessibleBuffer<[u8]>>,
        image: Arc<ImmutableImage<Format>>,
    ) {
        self.queued_image_copies.push((buffer, image));
    }

    pub fn submit_queued_commands(
        &mut self, mut future: Box<GpuFuture + Send + Sync>
    ) -> Box<GpuFuture + Send + Sync> {
        // If we don't have anything to upload, we don't need to alter the future at all
        if self.queued_image_copies.len() == 0 {
            return future;
        }

        // Create a command buffer to upload the textures with
        let mut image_copy_buffer_builder = AutoCommandBufferBuilder::new(
            self.device.clone(), self.graphics_queue.family()
        ).unwrap();

        // Add any textures we need to upload to the command buffer
        while let Some(val) = self.queued_image_copies.pop() {
            // Add the copy to the buffer
            image_copy_buffer_builder = image_copy_buffer_builder
                .copy_buffer_to_image(val.0, val.1)
                .unwrap();
        }

        // Add the command buffer to the future so it will be executed
        let image_copy_buffer = image_copy_buffer_builder.build().unwrap();
        future = Box::new(future
            .then_execute(self.graphics_queue.clone(), image_copy_buffer).unwrap()
        );

        future
    }
}

impl Renderer for VulkanoRenderer {
    fn log(&self) -> &Logger {
        &self.log
    }
}
