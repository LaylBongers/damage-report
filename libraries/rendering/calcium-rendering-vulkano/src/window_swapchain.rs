use std::sync::{Arc};

use cgmath::{Vector2};
use vulkano::device::{Queue};
use vulkano::framebuffer::{Framebuffer, RenderPassAbstract, FramebufferAbstract};
use vulkano::format::{self};
use vulkano::swapchain::{Swapchain, SurfaceTransform, Surface};
use vulkano::sync::{GpuFuture};
use vulkano::image::attachment::{AttachmentImage};
use vulkano::image::swapchain::{SwapchainImage};

use calcium_rendering::{Renderer};
use {VulkanoRenderer};

/// A representation of the buffer(s) renderers have to render to to show up on the target.
pub struct WindowSwapchain {
    pub swapchain: Arc<Swapchain>,
    pub depth_attachment: Arc<AttachmentImage<format::D32Sfloat_S8Uint>>,
    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,

    // Submissions from previous frames
    previous_frame: Option<Box<GpuFuture + Send + Sync>>,
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

        // To render in 3D, we need an extra buffer to keep track of the depth. Since this won't be
        //  displayed, we don't need multiple of it like we do with the color swapchain. This isn't
        //  marked as transient as we'll have to use its values across multiple framebuffers and
        //  render passes.
        // A format more precise than D16Unorm had to be used. That precision ended up giving
        //  noticeable rendering artifacts at relatively nearby depths. A floating point format is
        //  used to take advantage of the increased precision given by the reversed-z technique.
        debug!(renderer.log(), "Creating depth buffer");
        let depth_attachment = create_depth_attachment(renderer, &images);

        // Set up a render pass TODO: Comment better
        let color_buffer_format = swapchain.format();
        let depth_buffer_format = ::vulkano::format::Format::D32Sfloat_S8Uint;
        #[allow(dead_code)]
        let render_pass = Arc::new(single_pass_renderpass!(renderer.device().clone(),
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
        ).unwrap()) as Arc<RenderPassAbstract + Send + Sync>;

        // Set up the frame buffers matching the render pass
        // For each image in the swap chain, we create a frame buffer that renders to that image
        //  and to the depth buffer attachment. These attachments are defined by the render pass.
        debug!(renderer.log(), "Creating framebuffers for swapchain");
        let framebuffers = create_framebuffers(&images, &render_pass, &depth_attachment);

        WindowSwapchain {
            swapchain,
            depth_attachment,
            render_pass,
            framebuffers,
            previous_frame: Some(Box::new(::vulkano::sync::now(renderer.device().clone()))),
        }
    }

    pub fn resize(&mut self, renderer: &VulkanoRenderer, size: Vector2<u32>) {
        let (swapchain, images) = self.swapchain.recreate_with_dimension(size.into()).unwrap();
        self.swapchain = swapchain;
        self.depth_attachment = create_depth_attachment(renderer, &images);
        self.framebuffers = create_framebuffers(
            &images, &self.render_pass, &self.depth_attachment
        );
    }

    pub fn cleanup_finished_frames(&mut self) {
        self.previous_frame.as_mut().unwrap().cleanup_finished();
    }

    pub fn start_frame(&mut self) -> (Arc<FramebufferAbstract + Send + Sync>, usize, Box<GpuFuture + Send + Sync>) {
        let (image_num, aquire_future) = ::vulkano::swapchain::acquire_next_image(
            self.swapchain.clone(), None
        ).unwrap();

        let future = self.previous_frame.take().unwrap().join(aquire_future);
        let future: Box<GpuFuture + Send + Sync> = Box::new(future);

        (self.framebuffers[image_num].clone(), image_num, future)
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

fn create_depth_attachment(
    renderer: &VulkanoRenderer, images: &Vec<Arc<SwapchainImage>>,
) -> Arc<AttachmentImage<format::D32Sfloat_S8Uint>> {
    AttachmentImage::new(
        renderer.device().clone(), images[0].dimensions(), format::D32Sfloat_S8Uint
    ).unwrap()
}

fn create_framebuffers(
    images: &Vec<Arc<SwapchainImage>>,
    render_pass: &Arc<RenderPassAbstract + Send + Sync>,
    depth_attachment: &Arc<AttachmentImage<format::D32Sfloat_S8Uint>>,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    images.iter().map(|image| {
        Arc::new(Framebuffer::start(render_pass.clone())
            .add(image.clone()).unwrap()
            .add(depth_attachment.clone()).unwrap()
            .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>
    }).collect()
}
