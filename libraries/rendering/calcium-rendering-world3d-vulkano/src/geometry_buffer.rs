use std::sync::{Arc};

use vulkano::image::attachment::{AttachmentImage};
use vulkano::image::{ImageUsage};
use vulkano::format::{self, Format};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};

use calcium_rendering::{Renderer, Viewport};
use calcium_rendering_vulkano::{VulkanoRenderer};

pub struct GeometryBuffer {
    // TODO: This can be changed to R16G16B16A16Sfloat if lighting its positions are relative to
    //  the camera rather than relative to the world origin, precision quickly causes bugs. This is
    //  a high priority as R32G32B32A32Sfloat isn't gurenteed to be supported but
    //  R16G16B16A16Sfloat is.
    pub position_attachment: Arc<AttachmentImage<format::R32G32B32A32Sfloat>>,
    pub base_color_attachment: Arc<AttachmentImage<format::R8G8B8A8Srgb>>,
    // TODO: This one can be changed to R8G8B8A8Unorm if the geometry shader converts the values
    //  back to a 0.0-1.0 range.
    pub normal_attachment: Arc<AttachmentImage<format::R16G16B16A16Sfloat>>,
    pub metallic_attachment: Arc<AttachmentImage<format::R8Unorm>>,
    pub roughness_attachment: Arc<AttachmentImage<format::R8Unorm>>,
    pub depth_attachment: Arc<AttachmentImage<format::D32Sfloat_S8Uint>>,

    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    pub framebuffer: Arc<FramebufferAbstract + Send + Sync>,
}

impl GeometryBuffer {
    pub fn new(
        renderer: &VulkanoRenderer,
        viewport: &Viewport,
    ) -> Self {
        info!(renderer.log(), "Creating g-buffer");

        // The gbuffer attachments we end up using in the final lighting pass need to have sampled
        //  set to true, or we can't sample them, resulting in a black color result.
        let attach_usage = ImageUsage {
            sampled: true,
            .. ImageUsage::none()
        };

        // Create the attachment images that make up the G-buffer
        let position_attachment = AttachmentImage::with_usage(
            renderer.device().clone(), viewport.size.cast().into(),
            format::R32G32B32A32Sfloat, attach_usage
        ).unwrap();
        let base_color_attachment = AttachmentImage::with_usage(
            renderer.device().clone(), viewport.size.cast().into(),
            format::R8G8B8A8Srgb, attach_usage
        ).unwrap();
        let normal_attachment = AttachmentImage::with_usage(
            renderer.device().clone(), viewport.size.cast().into(),
            format::R16G16B16A16Sfloat, attach_usage
        ).unwrap();
        let metallic_attachment = AttachmentImage::with_usage(
            renderer.device().clone(), viewport.size.cast().into(),
            format::R8Unorm, attach_usage
        ).unwrap();
        let roughness_attachment = AttachmentImage::with_usage(
            renderer.device().clone(), viewport.size.cast().into(),
            format::R8Unorm, attach_usage
        ).unwrap();
        let depth_attachment = AttachmentImage::new(
            renderer.device().clone(), viewport.size.cast().into(),
            format::D32Sfloat_S8Uint
        ).unwrap();
        // Rather than create our own depth attachment, we re-use the one of the framebuffer the
        // lighting will eventually render to. A later forward rendering pass for transparent
        // objects will need the depth buffer, so we avoid a duplicate buffer and a copy this way.

        // Create the deferred render pass
        // TODO: Document better what a render pass does that a framebuffer doesn't
        #[allow(dead_code)]
        let render_pass = Arc::new(single_pass_renderpass!(renderer.device().clone(),
            attachments: {
                position: {
                    load: Clear,
                    store: Store,
                    format: Format::R32G32B32A32Sfloat,
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
                    format: Format::D32Sfloat_S8Uint,
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
