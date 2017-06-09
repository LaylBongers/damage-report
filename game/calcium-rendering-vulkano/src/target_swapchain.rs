use std::sync::{Arc};
use std::time::{Duration};

use cgmath::{Vector2};
use slog::{Logger};
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{Framebuffer, RenderPassAbstract, FramebufferAbstract};
use vulkano::format::{self, D16Unorm};
use vulkano::instance::{PhysicalDevice};
use vulkano::swapchain::{Swapchain, SurfaceTransform};
use vulkano::sync::{GpuFuture};
use vulkano::image::{Image};
use vulkano::image::attachment::{AttachmentImage};

use {Window};

/// A representation of the buffer(s) renderers have to render to to show up on the target.
pub struct TargetSwapchain {
    swapchain: Arc<Swapchain>,
    pub depth_attachment: Arc<AttachmentImage<format::D16Unorm>>,
    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,

    // Submissions from previous frames
    submissions: Vec<Box<GpuFuture>>,
}

impl TargetSwapchain {
    pub fn new<W: Window>(
        log: &Logger, window: &W, size: Vector2<u32>,
        physical: PhysicalDevice, device: Arc<Device>, graphics_queue: &Arc<Queue>,
    ) -> Self {
        // Now create the swapchain, we need this to actually swap between our back buffer and the
        //  window's front buffer, without it we can't show anything
        debug!(log, "Creating swapchain");
        let (swapchain, images) = {
            // Get what the swap chain we want to create would be capable of, we can't request
            //  anything it can't do
            let caps = window.surface().capabilities(physical).unwrap();

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
                device.clone(), window.surface().clone(), caps.min_image_count, format,
                dimensions, 1,
                caps.supported_usage_flags, graphics_queue, SurfaceTransform::Identity, alpha,
                present, true, None
            ).unwrap()
        };
        debug!(log, "Created swapchain"; "images" => images.len());

        // To render in 3D, we need an extra buffer to keep track of the depth. Since this won't be
        //  displayed, we don't need multiple of it like we do with the color swapchain. This isn't
        //  marked as transient as we'll have to use its values across multiple framebuffers and
        //  render passes.
        debug!(log, "Creating depth buffer");
        let depth_attachment = AttachmentImage::new(
            device.clone(), images[0].dimensions().width_height(), D16Unorm
        ).unwrap();

        // Set up a render pass TODO: Comment better
        let color_buffer_format = swapchain.format();
        let depth_buffer_format = ::vulkano::format::Format::D16Unorm;
        #[allow(dead_code)]
        let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_buffer_format,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: depth_buffer_format,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        ).unwrap());

        // Set up the frame buffers matching the render pass
        // For each image in the swap chain, we create a frame buffer that renders to that image
        //  and to the depth buffer attachment. These attachments are defined by the render pass.
        debug!(log, "Creating framebuffers for swapchain");
        let framebuffers = images.iter().map(|image| {
            Arc::new(Framebuffer::start(render_pass.clone())
                .add(image.clone()).unwrap()
                .add(depth_attachment.clone()).unwrap()
                .build().unwrap()
            ) as Arc<FramebufferAbstract + Send + Sync>
        }).collect::<Vec<_>>();

        TargetSwapchain {
            swapchain,
            depth_attachment,
            render_pass,
            framebuffers,
            submissions: Vec::new(),
        }
    }

    pub fn clean_old_submissions(&mut self) {
        // Clearing the old submissions by keeping alive only the ones which probably aren't
        //  finished
        while self.submissions.len() >= 4 {
            self.submissions.remove(0);
        }
    }

    pub fn start_frame(&self) -> (Arc<FramebufferAbstract + Send + Sync>, usize, Box<GpuFuture>) {
        let (image_num, future) = ::vulkano::swapchain::acquire_next_image(
            self.swapchain.clone(), Duration::new(1, 0)
        ).unwrap();
        let future: Box<GpuFuture> = Box::new(future);

        (self.framebuffers[image_num].clone(), image_num, future)
    }

    pub fn finish_frame(
        &mut self, future: Box<GpuFuture>, graphics_queue: Arc<Queue>, image_num: usize
    ) {
        let future = future
            // Present the image resulting from all the submitted command buffers on the screen
            .then_swapchain_present(
                graphics_queue, self.swapchain.clone(), image_num
            )
            // Finally, submit it all to the driver
            .then_signal_fence_and_flush().unwrap();

        // Keep track of these submissions so we can later wait on them
        self.submissions.push(Box::new(future));
    }
}