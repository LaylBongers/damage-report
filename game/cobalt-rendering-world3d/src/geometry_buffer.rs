use std::sync::{Arc};

use slog::{Logger};
use vulkano::image::attachment::{AttachmentImage};
use vulkano::image::{ImageUsage};
use vulkano::format::{self};

use cobalt_rendering::{Target};

pub struct GeometryBuffer {
    pub position_attachment: Arc<AttachmentImage<format::R16G16B16A16Sfloat>>,
    pub base_color_attachment: Arc<AttachmentImage<format::R8G8B8A8Srgb>>,
    pub normal_attachment: Arc<AttachmentImage<format::R16G16B16A16Sfloat>>,
    pub metallic_attachment: Arc<AttachmentImage<format::R8Unorm>>,
    pub roughness_attachment: Arc<AttachmentImage<format::R8Unorm>>,
    pub depth_attachment: Arc<AttachmentImage<format::D16Unorm>>,
}

impl GeometryBuffer {
    pub fn new(log: &Logger, target: &Target) -> Self {
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
        let depth_attachment = AttachmentImage::with_usage(
            target.device().clone(), target.size().into(), format::D16Unorm, attach_usage
        ).unwrap();

        GeometryBuffer {
            position_attachment,
            base_color_attachment,
            normal_attachment,
            metallic_attachment,
            roughness_attachment,
            depth_attachment,
        }
    }
}
