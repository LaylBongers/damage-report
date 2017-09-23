use std::any::{Any};
use calcium_rendering::{Renderer};
use {RenderBatch, Simple2DRenderTarget, Simple2DRenderTargetRaw, Projection};

/// A 2D renderer capable of rendering render batches.
pub trait Simple2DRenderer<R: Renderer>: Any + Sized {
    type RenderTargetRaw: Simple2DRenderTargetRaw<R, Self> + Any;
    type RenderPassRaw: Simple2DRenderPassRaw<R> + Any;

    /// Starts a rendering pass, using this you can render batches to a frame.
    fn start_pass<'a>(
        &self,
        frame: &'a mut R::Frame,
        render_target: &'a mut Simple2DRenderTarget<R, Self>,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer,
    ) -> Simple2DRenderPass<'a, R, Self>;

    /// Finishes the rendering pass.
    fn finish_pass<'a>(&self, pass: Simple2DRenderPass<'a, R, Self>, renderer: &mut R);
}

pub struct Simple2DRenderPass<'a, R: Renderer, SR: Simple2DRenderer<R>> {
    raw: SR::RenderPassRaw,
    frame: &'a mut R::Frame,
    finished: bool,
}

impl<'a, R: Renderer, SR: Simple2DRenderer<R>> Simple2DRenderPass<'a, R, SR> {
    pub fn raw_new(raw: SR::RenderPassRaw, frame: &'a mut R::Frame) -> Self {
        Simple2DRenderPass {
            raw,
            frame,
            finished: false,
        }
    }

    pub fn raw_mut(&mut self) -> &mut SR::RenderPassRaw {
        &mut self.raw
    }

    pub fn frame_mut(&mut self) -> &mut R::Frame {
        &mut self.frame
    }

    /// Renders the given render batches to the frame this pass was started for.
    pub fn render_batches(
        &mut self,
        batches: &[RenderBatch<R>], projection: Projection,
        renderer: &mut R, window_renderer: &mut R::WindowRenderer,
    ) {
        self.raw.render_batches(batches, projection, &mut self.frame, renderer, window_renderer);
    }

    /// Marks this render pass as being finished by the renderer, do not call this yourself!
    /// TODO: Move this to a separate trait so it's clear and needs to be use'd.
    pub fn mark_finished(&mut self) {
        self.finished = true;
    }
}

impl<'a, R: Renderer, SR: Simple2DRenderer<R>> Drop for Simple2DRenderPass<'a, R, SR> {
    fn drop(&mut self) {
        if !self.finished {
            panic!("You need to finish a started render pass")
        }
    }
}

pub trait Simple2DRenderPassRaw<R: Renderer> {
    fn render_batches(
        &mut self,
        batches: &[RenderBatch<R>], projection: Projection,
        frame: &mut R::Frame, renderer: &mut R, window_renderer: &mut R::WindowRenderer,
    );
}
