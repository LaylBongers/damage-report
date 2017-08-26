//! Types related to the visual look of elements.

use cgmath::{Vector2, Point2};

mod color;
mod position;
mod size;
mod style;

pub use self::color::{color_highlight, color_active};
pub use self::position::{Position, SideH, SideV};
pub use self::size::{Size, SizeValue};
pub use self::style::{Style};

// Re-export screenmath types for convenience
pub use screenmath::{Rectangle, Lrtb};

#[derive(Debug, Clone, Copy)]
pub enum FlowDirection {
    Right,
    Left,
    Down,
    Up,
}

impl FlowDirection {
    pub fn flow_start(&self, container: &Rectangle<f32>) -> Point2<f32> {
        match *self {
            FlowDirection::Right => container.min,
            FlowDirection::Left => Point2::new(container.max.x, container.min.y),
            FlowDirection::Down => container.min,
            FlowDirection::Up => Point2::new(container.min.x, container.max.y),
        }
    }

    pub fn position(&self, flow_cursor: Point2<f32>, size: Vector2<f32>) -> Point2<f32> {
        match *self {
            FlowDirection::Right => flow_cursor,
            FlowDirection::Left => flow_cursor + -Vector2::new(size.x, 0.0),
            FlowDirection::Down => flow_cursor,
            FlowDirection::Up => flow_cursor + -Vector2::new(0.0, size.y),
        }
    }

    pub fn advance_cursor(
        &self, position: Point2<f32>, size: Vector2<f32>, mut cursor: Point2<f32>
    ) -> Point2<f32> {
        match *self {
            FlowDirection::Right => cursor.x = position.x + size.x,
            FlowDirection::Left => cursor.x = position.x,
            FlowDirection::Down => cursor.y = position.y + size.y,
            FlowDirection::Up => cursor.y = position.y,
        }

        cursor
    }
}
