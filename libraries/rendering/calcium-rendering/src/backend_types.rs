use std::any::{Any};
use texture::{Texture};
use {WindowRenderer};

pub trait BackendTypes: Any + Clone {
    type WindowRenderer: Any + WindowRenderer<Self>;
    type Renderer: Any;
    type Frame: Any;
    
    type Texture: Texture<Self> + Any + Send + Sync;
}
