use cgmath::{Vector2};
use palette::{Rgba};

#[derive(Clone, Debug)]
pub struct Style {
    pub position: Position,
    /// The minimum spacing this element will have from the bordering elements and the parent's
    /// container edges. Bordering elements' margins overlap.
    pub margin: Lrtb,
    pub size: Size,

    pub background_color: Option<Rgba>,
    pub text_color: Rgba,
    pub text: String,
}

impl Style {
    pub fn new() -> Self {
        Style {
            position: Position::Flow,
            margin: Lrtb::new(0.0, 0.0, 0.0, 0.0),
            size: Size::units(0.0, 0.0),

            background_color: None,
            text_color: Rgba::new(0.0, 0.0, 0.0, 1.0),
            text: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lrtb {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Lrtb {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Lrtb {
            left, right, top, bottom
        }
    }

    pub fn uniform(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn left_top(&self) -> Vector2<f32> {
        Vector2::new(self.left, self.top)
    }
}

#[derive(Clone, Debug)]
pub enum Position {
    /// Positions the element using previous elements.
    Flow,
    /// Positions the element using a position relative to the parent.
    Relative(Vector2<f32>),
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: SizeValue,
    pub height: SizeValue,
}

impl Size {
    pub fn new(width: SizeValue, height: SizeValue) -> Self {
        Size {
            width, height,
        }
    }

    pub fn units(width: f32, height: f32) -> Self {
        Self::new(SizeValue::Units(width), SizeValue::Units(height))
    }

    pub fn to_units(&self, container_size: Vector2<f32>) -> Vector2<f32> {
        Vector2::new(
            self.width.to_units(container_size.x),
            self.height.to_units(container_size.y),
        )
    }
}

#[derive(Clone, Debug)]
pub enum SizeValue {
    /// Size is determined from units of HDPI factor. (This maps 1:1 to pixels at HDPI factor 1.0)
    Units(f32),
    /// Size is determined as a scale of the parent container.
    Scale(f32),
    // TODO: Size is determined automatically from the element's children.
    // Auto,
}

impl SizeValue {
    pub fn to_units(&self, container_size: f32) -> f32 {
        match *self {
            SizeValue::Units(v) => v,
            SizeValue::Scale(v) => v * container_size,
        }
    }
}
