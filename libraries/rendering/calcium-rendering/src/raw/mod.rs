use {Error, Renderer};
use texture::{TextureBuilder};

/// This trait is meant for internal usage, it allows backends to access the raw data behind high
/// level types.
pub trait RawAccess<T> {
    fn raw(&self) -> &T;
    fn raw_mut(&mut self) -> &mut T;
}

pub trait TextureRaw<R: Renderer>: Sized {
    fn new(builder: TextureBuilder<R>, renderer: &mut R) -> Result<Self, Error>;
}
