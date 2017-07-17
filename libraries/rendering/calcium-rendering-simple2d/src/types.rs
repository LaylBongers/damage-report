use std::any::{Any};

use calcium_rendering::{Types};

use {Simple2DRenderer, Simple2DRenderTargetRaw};

/// An associated types container with all types for a backend.
pub trait Simple2DTypes<T: Types>: Sized {
    type Renderer: Simple2DRenderer<T, Self> + Any;
    type RenderTargetRaw: Simple2DRenderTargetRaw<T, Self> + Any;
}
