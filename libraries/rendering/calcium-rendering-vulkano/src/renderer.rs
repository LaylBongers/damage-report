use std::sync::{Arc};

use slog::{Logger};
use vulkano::format::{Format};
use vulkano::buffer::{CpuAccessibleBuffer};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::instance::{PhysicalDevice};
use vulkano::image::immutable::{ImmutableImage};

use calcium_rendering::{Error};
use {VulkanoWindowRenderer, VulkanoSystemContext};

pub struct VulkanoRenderer {
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,

    // Queued up things we need to submit as part of command buffers
    queued_image_copies: Vec<(Arc<CpuAccessibleBuffer<[u8]>>, Arc<ImmutableImage<Format>>)>,
}

impl VulkanoRenderer {
    pub fn new(
        log: &Logger, system_context: &VulkanoSystemContext,
        windows: &mut [&mut VulkanoWindowRenderer]
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

        // Find a GPU graphics queue family, we later create a queue from this family to talk to
        //  the GPU
        debug!(log, "Finding graphics queue family with required features");
        let graphics_queue_family = physical.queue_families().find(|q| {
            // The queue needs to support graphics (of course) and needs to support drawing to
            //  the previously created windows' surfaces
            q.supports_graphics() && {
                let mut supported = true;
                for win in windows.iter() {
                    supported = supported && win.surface.is_supported(*q).unwrap_or(false)
                }
                supported
            }
        }).expect("Unable to find fitting graphics queue");

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

        // Tell the windows to finish initializing so we can use them after this is done
        for win in windows.iter_mut() {
            win.finish_initialization(log, physical.clone(), device.clone(), &graphics_queue);
        }

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
