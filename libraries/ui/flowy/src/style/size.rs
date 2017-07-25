use cgmath::{Vector2};
use screenmath::{Lrtb};

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

    pub fn scale(width: f32, height: f32) -> Self {
        Self::new(SizeValue::Scale(width), SizeValue::Scale(height))
    }

    pub fn to_units(
        &self, container_size: Vector2<f32>, container_padding: &Lrtb
    ) -> Vector2<f32> {
        Vector2::new(
            self.width.to_units(container_size.x - container_padding.left - container_padding.right),
            self.height.to_units(container_size.y - container_padding.top - container_padding.bottom),
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
