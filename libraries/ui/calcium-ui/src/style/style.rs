use palette::{Rgba};

use style::{Position, Lrtb, Size, CursorBehavior, SideH, SideV, FlowDirection};

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
    pub text_color: Rgba,
    pub text_size: f32,
    /// Not fully implemented, only supports horizontal align right now.
    pub text_align: (SideH, SideV),

    pub cursor_behavior: CursorBehavior,
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
            text_color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            // Reasonable default, if it's at 0 users will get confused about text not rendering
            text_size: 10.0,
            text_align: (SideH::Left, SideV::Top),

            cursor_behavior: CursorBehavior::PassThrough,
        }
    }
}
