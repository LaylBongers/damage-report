use std::sync::{Arc};
use std::cmp::{min, max};

use cgmath::{Vector2};
use vulkano::device::{Queue};
use vulkano::swapchain::{Swapchain, SurfaceTransform, Surface};
use vulkano::sync::{GpuFuture};
use vulkano::image::swapchain::{SwapchainImage};

use calcium_rendering::{Renderer};
use {VulkanoRenderer};

/// A representation of the buffer(s) renderers have to render to to show up on the target.
pub struct WindowSwapchain {
    pub swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,

    // Submissions from previous frames
    previous_frame: Option<Box<GpuFuture + Send + Sync>>,

    images_id: usize,
}

impl WindowSwapchain {
    pub fn new(
        renderer: &VulkanoRenderer, surface: &Arc<Surface>, size: Vector2<u32>,
    ) -> Self {
        // Now create the swapchain, we need this to actually swap between our back buffer and the
        //  window's front buffer, without it we can't show anything
        debug!(renderer.log(), "Creating swapchain");
        let (swapchain, images) = {
            // Get what the swap chain we want to create would be capable of, we can't request
            //  anything it can't do
            let caps = surface.capabilities(renderer.device().physical_device()).unwrap();

            // The swap chain's dimensions need to match the window size
            let dimensions = caps.current_extent.unwrap_or([size.x, size.y]);

            // The present mode is things like vsync and vsync-framerate, right now pick the first
            //  one, we're sure it will work but it's probably not optimal
            // TODO: Let the user decide
            let present = caps.present_modes.iter().next().unwrap();

            // This decides how alpha will be composited by the OS' window manager, we just pick
            //  the first available option
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();

            // And finally, chose the internal format that images will have
            // The swap chain needs to be in SRGB, and this format is guaranteed supported
            let format = ::vulkano::format::B8G8R8A8Srgb;

            // Finally, actually create the swapchain, with all its color images
            Swapchain::new(
                renderer.device().clone(), surface.clone(), caps.min_image_count, format,
                dimensions, 1,
                caps.supported_usage_flags, renderer.graphics_queue(),
                SurfaceTransform::Identity, alpha,
                present, true, None
            ).unwrap()
        };
        debug!(renderer.log(), "Created swapchain"; "images" => images.len());

        WindowSwapchain {
            swapchain,
            images,

            previous_frame: Some(Box::new(::vulkano::sync::now(renderer.device().clone()))),

            images_id: 0,
        }
    }

    pub fn images(&self) -> &Vec<Arc<SwapchainImage>> {
        &self.images
    }

    /// Increments every time the swapchain images vector gets updated, can be used to check if
    /// framebuffers should be updated.
    pub fn images_id(&self) -> usize {
        self.images_id
    }

    /// Resizes the swapchain, returns the actual size it was resized to which may be different
    /// from the requested size.
    pub fn resize(
        &mut self, mut size: Vector2<u32>, renderer: &VulkanoRenderer, surface: &Arc<Surface>,
    ) -> Vector2<u32> {
        // Limit to the size the surface's capabilities allow
        let caps = surface.capabilities(renderer.device().physical_device()).unwrap();
        size.x = max(size.x, caps.min_image_extent[0]);
        size.y = max(size.y, caps.min_image_extent[1]);
        size.x = min(size.x, caps.max_image_extent[0]);
        size.y = min(size.y, caps.max_image_extent[1]);

        // Perform the actual resize
        let (swapchain, images) = self.swapchain.recreate_with_dimension(size.into()).unwrap();
        self.swapchain = swapchain;
        self.images = images;

        self.images_id += 1;

        size
    }

    pub fn cleanup_finished_frames(&mut self) {
        self.previous_frame.as_mut().unwrap().cleanup_finished();
    }

    pub fn start_frame(
        &mut self
    ) -> (usize, Box<GpuFuture + Send + Sync>) {
        let (image_num, aquire_future) = ::vulkano::swapchain::acquire_next_image(
            self.swapchain.clone(), None
        ).unwrap();

        let future = self.previous_frame.take().unwrap().join(aquire_future);
        let future: Box<GpuFuture + Send + Sync> = Box::new(future);

        (image_num, future)
    }

    pub fn finish_frame(
        &mut self, future: Box<GpuFuture + Send + Sync>, graphics_queue: Arc<Queue>, image_num: usize
    ) {
        let future = future
            // Present the image resulting from all the submitted command buffers on the screen
            .then_swapchain_present(
                graphics_queue, self.swapchain.clone(), image_num
            )
            // Finally, submit it all to the driver
            .then_signal_fence_and_flush().unwrap();

        // Keep track of these submissions so we can later wait on them
        self.previous_frame = Some(Box::new(future));
    }
}
