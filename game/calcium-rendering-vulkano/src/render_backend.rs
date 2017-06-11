use std::sync::{Arc};
use std::collections::{HashMap};

use cgmath::{Vector2};
use slog::{Logger};
use vulkano::buffer::{CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder};
use vulkano::device::{DeviceExtensions, Device, Queue};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::sync::{GpuFuture};
use vulkano::framebuffer::{FramebufferAbstract};

use calcium_rendering::{Error, RenderBackend, CalciumErrorMap, Frame, Texture};
use target_swapchain::{TargetSwapchain};
use texture::{VulkanoTextureBackend};
use {VulkanoTargetSystem};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct TextureId(usize);

pub struct VulkanoRenderBackend {
    // Persistent values needed for vulkan rendering
    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
    pub target_swapchain: TargetSwapchain,

    // Queued up things we need to submit as part of command buffers
    queued_texture_copies: Vec<(Arc<CpuAccessibleBuffer<[u8]>>, TextureId)>,

    pub size: Vector2<u32>,
    textures: HashMap<TextureId, VulkanoTextureBackend>,
}

impl VulkanoRenderBackend {
    pub fn new(log: &Logger, target: &mut VulkanoTargetSystem) -> Result<VulkanoRenderBackend, Error> {
        info!(log, "Initializing vulkano backend");
        let size = Vector2::new(1280, 720);

        // Start by setting up the vulkano instance, this is a silo of vulkan that all our vulkan
        //  types will belong to
        debug!(log, "Creating vulkan instance");
        let instance = {
            // Tell it we need at least the extensions vulkano-win needs
            Instance::new(None, &target.required_extensions(), None)
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
        let target_surface = target.create_surface(instance.clone(), size);

        // Find a GPU graphics queue family, we later create a queue from this family to talk to
        //  the GPU
        debug!(log, "Finding graphics queue family with required features");
        let graphics_queue_family = physical.queue_families().find(|q| {
            // The queue needs to support graphics (of course) and needs to support drawing to
            //  the previously created window's surface
            q.supports_graphics() && target_surface.is_supported(*q).unwrap_or(false)
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
            log, &target_surface, size, physical, device.clone(), &graphics_queue
        );

        Ok(VulkanoRenderBackend {
            device,
            graphics_queue,
            target_swapchain,

            queued_texture_copies: Vec::new(),

            size,
            textures: HashMap::new(),
        })
    }

    /// Requests a backend texture for a frontend texture. Submits the texture for loading if not
    /// yet submitted.
    pub fn request_texture(
        &mut self, log: &Logger, texture: &Arc<Texture>
    ) -> Option<&VulkanoTextureBackend> {
        // Look up the texture from the texture backend storage, or add it if it isn't there yet
        let texture_backend = self.lookup_or_submit_texture(log, texture);

        // Check if it's ready for rendering
        return if texture_backend.is_ready() {
            Some(texture_backend)
        } else {
            None
        };
    }

    fn lookup_or_submit_texture(
        &mut self, log: &Logger, texture: &Arc<Texture>
    ) -> &VulkanoTextureBackend {
        let key = TextureId(arc_key(&texture));

        // If we don't have this texture yet, submit it first
        if !self.textures.contains_key(&key) {
            self.submit_texture(log, texture);
        }

        self.textures.get(&key).unwrap()
    }

    fn submit_texture(&mut self, log: &Logger, texture: &Arc<Texture>) {
        // TODO: Offload loading to a separate thread

        // Start by loading in the actual image
        let (texture_backend, buffer) = VulkanoTextureBackend::load(
            log, self, &texture.source, texture.format
        );

        // Store the texture backend, maintaining its ID so we can look it back up
        let texture_id = self.store_texture(&texture, texture_backend);

        // Then submit the buffer and texture for copying, it will be picked up later at the start
        //  of a frame to actually be copied over
        self.queue_texture_copy(buffer, texture_id);
    }

    fn store_texture(
        &mut self, texture: &Arc<Texture>, texture_backend: VulkanoTextureBackend
    ) -> TextureId {
        let key = TextureId(arc_key(texture));

        // First make sure this texture doesn't already exist, this shouldn't ever happen, but it's
        // not that expensive to make sure
        if self.textures.contains_key(&key) {
            panic!("Texture backend already exists for texture")
        }

        // Now that we're sure, we can submit the texture
        self.textures.insert(key, texture_backend);

        key
    }

    fn queue_texture_copy(
        &mut self,
        buffer: Arc<CpuAccessibleBuffer<[u8]>>,
        texture: TextureId,
    ) {
        self.queued_texture_copies.push((buffer, texture));
    }
}

/// Creates a value to use as key in a hashmap for referring to the abstract existence of a value
/// in an arc. This is equivalent to using a reference as key in a hashmap/dictionary in other
/// languages.
fn arc_key<T>(value: &Arc<T>) -> usize {
    value.as_ref() as *const T as usize
}

impl RenderBackend for VulkanoRenderBackend {
    fn start_frame(&mut self) -> Box<Frame> {
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
                // Look up the actual texture
                let texture = self.textures.get_mut(&val.1).unwrap();

                // Add the copy to the buffer
                image_copy_buffer_builder = image_copy_buffer_builder
                    .copy_buffer_to_image(val.0, texture.image.clone())
                    .unwrap();

                // Now that the texture's copied, we can mark it ready
                texture.mark_ready();
            }

            // Add the command buffer to the future so it will be executed
            let image_copy_buffer = image_copy_buffer_builder.build().unwrap();
            future = Box::new(future
                .then_execute(self.graphics_queue.clone(), image_copy_buffer).unwrap()
            );
        }

        Box::new(VulkanoFrame {
            framebuffer,
            image_num,
            future: Some(future),
        })
    }

    fn finish_frame(&mut self, mut frame: Box<Frame>) {
        let frame = frame.downcast_mut::<VulkanoFrame>().unwrap();
        self.target_swapchain.finish_frame(
            frame.future.take().unwrap(), self.graphics_queue.clone(), frame.image_num
        );
    }
}

pub struct VulkanoFrame {
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
    pub image_num: usize,
    pub future: Option<Box<GpuFuture>>,
}

impl Frame for VulkanoFrame {
}
