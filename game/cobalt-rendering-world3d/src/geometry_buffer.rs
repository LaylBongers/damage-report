use std::sync::{Arc};

use slog::{Logger};
use vulkano::image::attachment::{AttachmentImage};
use vulkano::image::{ImageUsage};
use vulkano::format::{self, Format};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};

use cobalt_rendering::{Target};

pub struct GeometryBuffer {
    pub position_attachment: Arc<AttachmentImage<format::R16G16B16A16Sfloat>>,
    pub base_color_attachment: Arc<AttachmentImage<format::R8G8B8A8Srgb>>,
    // TODO: This one can be changed to R8G8B8A8Unorm if the geometry shader converts the values
    //  back to a 0.0-1.0 range
    pub normal_attachment: Arc<AttachmentImage<format::R16G16B16A16Sfloat>>,
    pub metallic_attachment: Arc<AttachmentImage<format::R8Unorm>>,
    pub roughness_attachment: Arc<AttachmentImage<format::R8Unorm>>,
    pub depth_attachment: Arc<AttachmentImage<format::D16Unorm>>,

    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
}

impl GeometryBuffer {
    pub fn new(
        log: &Logger, target: &Target, depth_attachment: Arc<AttachmentImage<format::D16Unorm>>
    ) -> Self {
        // The gbuffer attachments we end up using in the final lighting pass need to have sampled
        //  set to true, or we can't sample them, resulting in a black color result.
        let attach_usage = ImageUsage {
            sampled: true,
            .. ImageUsage::none()
        };

        // Create the attachment images that make up the G-buffer
        debug!(log, "Creating g-buffer attachment images");
        let position_attachment = AttachmentImage::with_usage(
            target.device().clone(), target.size().into(), format::R16G16B16A16Sfloat, attach_usage
        ).unwrap();
        let base_color_attachment = AttachmentImage::with_usage(
            target.device().clone(), target.size().into(), format::R8G8B8A8Srgb, attach_usage
        ).unwrap();
        let normal_attachment = AttachmentImage::with_usage(
            target.device().clone(), target.size().into(), format::R16G16B16A16Sfloat, attach_usage
        ).unwrap();
        let metallic_attachment = AttachmentImage::with_usage(
            target.device().clone(), target.size().into(), format::R8Unorm, attach_usage
        ).unwrap();
        let roughness_attachment = AttachmentImage::with_usage(
            target.device().clone(), target.size().into(), format::R8Unorm, attach_usage
        ).unwrap();
        // Rather than create our own depth attachment, we re-use the one of the framebuffer the
        // lighting will eventually render to. A later forward rendering pass for transparent
        // objects will need the depth buffer, so we avoid a duplicate buffer and a copy this way.

        // Create the deferred render pass
        // TODO: Document better what a render pass does that a framebuffer doesn't
        debug!(log, "Creating g-buffer render pass");
        #[allow(dead_code)]
        let render_pass = Arc::new(single_pass_renderpass!(target.device().clone(),
            attachments: {
                position: {
                    load: Clear,
                    store: Store,
                    format: Format::R16G16B16A16Sfloat,
                    samples: 1,
                },
                base_color: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Srgb,
                    samples: 1,
                },
                normal: {
                    load: Clear,
                    store: Store,
                    format: Format::R16G16B16A16Sfloat,
                    samples: 1,
                },
                metallic: {
                    load: Clear,
                    store: Store,
                    format: Format::R8Unorm,
                    samples: 1,
                },
                roughness: {
                    load: Clear,
                    store: Store,
                    format: Format::R8Unorm,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [position, base_color, normal, metallic, roughness],
                depth_stencil: {depth}
            }
        ).unwrap());

        // Create the off-screen g-buffer framebuffer that we will use to actually tell vulkano
        //  what images we want to render to
        debug!(log, "Creating g-buffer framebuffer");
        let framebuffer = Arc::new(Framebuffer::start(render_pass.clone())
            .add(position_attachment.clone()).unwrap()
            .add(base_color_attachment.clone()).unwrap()
            .add(normal_attachment.clone()).unwrap()
            .add(metallic_attachment.clone()).unwrap()
            .add(roughness_attachment.clone()).unwrap()
            .add(depth_attachment.clone()).unwrap()
            .build().unwrap()
        ) as Arc<FramebufferAbstract + Send + Sync>;

        GeometryBuffer {
            position_attachment,
            base_color_attachment,
            normal_attachment,
            metallic_attachment,
            roughness_attachment,
            depth_attachment,

            render_pass,
            framebuffer,
        }
    }
}
