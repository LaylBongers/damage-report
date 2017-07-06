use std::sync::{Arc};

use slog::{Logger};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::instance::{PhysicalDevice};
use vulkano::image::immutable::{ImmutableImage};

use calcium_rendering::{Error};
use {VulkanoSystemContext};

pub struct VulkanoRenderer {
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,

    // Queued up things we need to submit as part of command buffers
    // TODO: This stopped being handled because of a refactor, make sure they're submitted again
    queued_image_copies: Vec<(Arc<CpuAccessibleBuffer<[u8]>>, Arc<ImmutableImage<Format>>)>,
}

impl VulkanoRenderer {
    pub fn new(
        log: &Logger, system_context: &VulkanoSystemContext,
    ) -> Result<Self, Error> {
        info!(log, "Initializing vulkano renderer");

        // Pick a GPU to use for rendering. We assume first device as the one to render with
        // TODO: Allow user to select in some way, perhaps through config
        debug!(log, "Finding target physical device");
        let physical = PhysicalDevice::enumerate(&system_context.instance).next()
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
            device,
            graphics_queue,

            queued_image_copies: Vec::new(),
        })
    }

    pub fn queue_image_copy(
        &mut self,
        buffer: Arc<CpuAccessibleBuffer<[u8]>>,
        image: Arc<ImmutableImage<Format>>,
    ) {
        self.queued_image_copies.push((buffer, image));
    }
}
