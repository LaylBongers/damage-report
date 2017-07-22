use palette::{Rgba};

use style::{Position, Lrtb, Size, CursorBehavior};

#[derive(Clone, Debug)]
pub struct Style {
    pub position: Position,
    /// The minimum spacing this element will have from the bordering elements and the parent's
    /// container edges. Bordering elements' margins overlap.
    pub margin: Lrtb,
    pub size: Size,

    pub background_color: Option<Rgba>,
    pub text_color: Rgba,
    pub text_size: f32,

    pub cursor_behavior: CursorBehavior,
}

impl Style {
    pub fn new() -> Self {
        Style {
            position: Position::Flow,
            margin: Lrtb::new(0.0, 0.0, 0.0, 0.0),
            size: Size::units(0.0, 0.0),

            background_color: None,
            text_color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            // Reasonable default, if it's at 0 users will get confused about text not rendering
            text_size: 10.0,

            cursor_behavior: CursorBehavior::PassThrough,
        }
    }
}
