use std::sync::{Arc};
use std::rc::{Rc};

use cgmath::{self, Vector2};
use gfx::{self, Device, Factory, VertexBuffer, ConstantBuffer};
use gfx::handle::{Sampler, Buffer};
use gfx::pso::{PipelineState};
use gfx::pso::resource::{RawShaderResource};
use gfx::traits::{FactoryExt};
use gfx::texture::{SamplerInfo, FilterMethod, WrapMode};

use calcium_rendering::{Error};
use calcium_rendering::texture::{Texture, SampleMode};
use calcium_rendering_gfx::{GfxRenderer, GfxFrame, ColorFormat, GfxWindowRenderer};
use calcium_rendering_simple2d::{Simple2DRenderer, RenderBatch, ShaderMode, Simple2DRenderTarget, Simple2DRenderPassRaw, Simple2DRenderPass, Projection};

use {GfxSimple2DRenderTargetRaw};

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "v_position",
        uv: [f32; 2] = "v_uv",
        color: [f32; 4] = "v_color",
    }

    constant Transform {
        transform: [[f32; 4]; 4] = "u_transform",
    }

    constant Mode {
        mode: u32 = "u_mode",
    }

    pipeline pipe {
        vbuf: VertexBuffer<Vertex> = (),
        transform: ConstantBuffer<Transform> = "Transform",
        mode: ConstantBuffer<Mode> = "Mode",
        texture: RawShaderResource = "u_texture",
        texture_sampler: ::gfx::pso::resource::Sampler = "u_texture",
        out: gfx::BlendTarget<ColorFormat> = (
            "Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA
        ),
    }
}

struct GfxRenderData<D: Device + 'static, F: Factory<D::Resources> + 'static> {
    pso: PipelineState<D::Resources, pipe::Meta>,
    dummy_texture: Arc<Texture<GfxRenderer<D, F>>>,
    mode_buffers: Vec<Buffer<D::Resources, Mode>>,

    linear_sampler: Sampler<D::Resources>,
    nearest_sampler: Sampler<D::Resources>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static> GfxRenderData<D, F> {
    fn sampler_for_mode(&self, sample_mode: SampleMode) -> &Sampler<D::Resources> {
        match sample_mode {
            SampleMode::Linear => &self.linear_sampler,
            SampleMode::Nearest => &self.nearest_sampler,
        }
    }
}

pub struct GfxSimple2DRenderer<D: Device + 'static, F: Factory<D::Resources> + 'static> {
    render_data: Rc<GfxRenderData<D, F>>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static> GfxSimple2DRenderer<D, F> {
    pub fn new(
        renderer: &mut GfxRenderer<D, F>
    ) -> Result<Self, Error> {
        let pso = renderer.factory.create_pipeline_simple(
            include_bytes!("../shaders/simple2d_150_vert.glsl"),
            include_bytes!("../shaders/simple2d_150_frag.glsl"),
            pipe::new()
        ).unwrap();

        let dummy_texture = Texture::new()
            .from_bytes(vec![255u8; 8*8], Vector2::new(8, 8), false)
            .as_single_channel()
            .build(renderer)?;

        // Create pre-made buffers for the shader modes
        let mut mode_buffers = Vec::new();
        for i in 0..3 {
            let mode = Mode {
                mode: i
            };
            let mode_buffer = renderer.factory.create_constant_buffer(1);
            renderer.encoder.update_buffer(&mode_buffer, &[mode], 0).unwrap();
            mode_buffers.push(mode_buffer);
        }

        // Create the samplers for the two sample modes
        let linear_sampler = renderer.factory.create_sampler(SamplerInfo::new(
            FilterMethod::Trilinear,
            WrapMode::Clamp,
        ));
        let nearest_sampler = renderer.factory.create_sampler(SamplerInfo::new(
            FilterMethod::Scale,
            WrapMode::Clamp,
        ));

        Ok(GfxSimple2DRenderer {
            render_data: Rc::new(GfxRenderData {
                pso,
                dummy_texture,
                mode_buffers,

                linear_sampler,
                nearest_sampler,
            })
        })
    }
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderer<GfxRenderer<D, F>> for GfxSimple2DRenderer<D, F>
{
    type RenderTargetRaw = GfxSimple2DRenderTargetRaw;
    type RenderPassRaw = GfxSimple2DRenderPassRaw<D, F>;

    fn start_pass<'a>(
        &self,
        frame: &'a mut GfxFrame,
        render_target: &mut Simple2DRenderTarget<GfxRenderer<D, F>, Self>,
        renderer: &mut GfxRenderer<D, F>, _window_renderer: &mut GfxWindowRenderer,
    ) -> Simple2DRenderPass<'a, GfxRenderer<D, F>, Self> {
        // Clear if we were told to clear
        if render_target.raw.is_clear() {
            renderer.encoder.clear(&renderer.color_view, [0.0, 0.0, 0.0, 1.0]);
        }

        Simple2DRenderPass::raw_new(GfxSimple2DRenderPassRaw {
            render_data: self.render_data.clone()
        }, frame)
    }

    fn finish_pass<'a>(
        &self, mut pass: Simple2DRenderPass<'a, GfxRenderer<D, F>, Self>, _renderer: &mut GfxRenderer<D, F>,
    ) {
        // Make sure the pass doesn't panic
        pass.mark_finished();
    }
}

pub struct GfxSimple2DRenderPassRaw<D: Device + 'static, F: Factory<D::Resources> + 'static> {
    render_data: Rc<GfxRenderData<D, F>>,
}

impl<D: Device + 'static, F: Factory<D::Resources> + 'static>
    Simple2DRenderPassRaw<GfxRenderer<D, F>> for GfxSimple2DRenderPassRaw<D, F>
{
    fn render_batches(
        &mut self,
        batches: &[RenderBatch<GfxRenderer<D, F>>], projection: Projection,
        frame: &mut GfxFrame, renderer: &mut GfxRenderer<D, F>, _window_renderer: &mut GfxWindowRenderer,
    ) {
        // Create a projection matrix that just matches coordinates to pixels
        let proj = cgmath::ortho(
            0.0, frame.size().x as f32,
            frame.size().y as f32, 0.0,
            1.0, -1.0
        );
        let transform = Transform {
            transform: proj.into()
        };
        let transform_buffer = renderer.factory.create_constant_buffer(1);
        renderer.encoder.update_buffer(&transform_buffer, &[transform], 0).unwrap();

        // Go over all batches
        for batch in batches {
            // Create a big mesh of all the rectangles we got told to draw this batch
            let mut vertices = Vec::new();
            for vertex in &batch.vertices {
                vertices.push(Vertex {
                    position: vertex.position.into(),
                    uv: vertex.uv.into(),
                    color: vertex.color.into(),
                });
            }

            // Create an actual VBO from it
            let (vertex_buffer, slice) = renderer.factory.create_vertex_buffer_with_slice(
                &vertices, ()
            );

            // Get the mode ID this batch has and a texture to render
            // TODO: Figure out a way to avoid having to have a dummy texture
            let (mode_id, texture, sampler) = match &batch.mode {
                &ShaderMode::Color =>
                    (0, &self.render_data.dummy_texture, &self.render_data.linear_sampler),
                &ShaderMode::Texture(ref texture) =>
                    (1, texture, self.render_data.sampler_for_mode(texture.raw.sample_mode)),
                &ShaderMode::Mask(ref texture) =>
                    (2, texture, self.render_data.sampler_for_mode(texture.raw.sample_mode)),
            };

            // Get the matching buffer for this shader mode
            let mode_buffer = &self.render_data.mode_buffers[mode_id];

            // Gather together all the data we need to render
            let data = pipe::Data {
                vbuf: vertex_buffer,
                transform: transform_buffer.clone(),
                mode: mode_buffer.clone(),
                texture: texture.raw.view.raw().clone(),
                texture_sampler: sampler.clone(),
                out: renderer.color_view.clone(),
            };

            // Finally, add the draw to the encoder
            renderer.encoder.draw(&slice, &self.render_data.pso, &data);
        }
    }
}
