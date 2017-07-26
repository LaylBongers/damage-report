use palette::{Rgba};
use screenmath::{Lrtb};

use style::{Position, Size, SideH, SideV, FlowDirection};
use {FontId};

#[derive(Clone, Debug)]
pub struct Style {
    pub position: Position,
    /// The minimum spacing this element will have from the bordering elements and the parent's
    /// container edges. Bordering elements' margins overlap.
    pub margin: Lrtb,
    /// The minimum spacing this element's child elements will have from the container. Padding
    /// overlaps with child margins. Child scale sizing is done relative to the container after
    /// padding.
    pub padding: Lrtb,
    pub size: Size,
    pub flow_direction: FlowDirection,

    pub background_color: Option<Rgba>,
    pub hover_color: Option<Rgba>,
    pub active_color: Option<Rgba>,

    pub text_font: FontId,
    pub text_color: Rgba,
    pub text_size: f32,
    /// Not fully implemented, only supports horizontal align right now.
    pub text_align: (SideH, SideV),
}

impl Style {
    pub fn new() -> Self {
        Style {
            position: Position::Flow,
            margin: Lrtb::uniform(0.0),
            padding: Lrtb::uniform(0.0),
            size: Size::units(0.0, 0.0),
            flow_direction: FlowDirection::Right,

            background_color: None,
            hover_color: None,
            active_color: None,

            // First font should be assumed as default
            text_font: FontId(0),
            text_color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            // Reasonable default, if it's at 0 users will get confused about text not rendering
            text_size: 10.0,
            text_align: (SideH::Left, SideV::Top),
        }
    }
}
