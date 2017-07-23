use cgmath::{Vector2};

use calcium_rendering_simple2d::{Rectangle};

mod cursor;
mod position;
mod size;
mod style;

pub use self::cursor::{CursorBehavior};
pub use self::position::{Position, SideH, SideV};
pub use self::size::{Size, SizeValue};
pub use self::style::{Style};

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

    /// Takes the value and its own left value, then returns a new Lrtb with the maximum of the two
    /// left values.
    pub fn max_left(&self, value: f32) -> Self {
        let mut lrtb = self.clone();
        lrtb.left = f32::max(value, lrtb.left);
        lrtb
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FlowDirection {
    Right,
    Left,
    Down,
    Up,
}

impl FlowDirection {
    pub fn flow_start(&self, container: &Rectangle<f32>) -> Vector2<f32> {
        match *self {
            FlowDirection::Right => container.start,
            FlowDirection::Left => Vector2::new(container.end.x, container.start.y),
            FlowDirection::Down => container.start,
            FlowDirection::Up => Vector2::new(container.start.x, container.end.y),
        }
    }

    pub fn position(&self, flow_cursor: Vector2<f32>, size: Vector2<f32>) -> Vector2<f32> {
        match *self {
            FlowDirection::Right => flow_cursor,
            FlowDirection::Left => flow_cursor - Vector2::new(size.x, 0.0),
            FlowDirection::Down => flow_cursor,
            FlowDirection::Up => flow_cursor - Vector2::new(0.0, size.y),
        }
    }

    pub fn advance_cursor(
        &self, position: Vector2<f32>, size: Vector2<f32>, mut cursor: Vector2<f32>
    ) -> Vector2<f32> {
        match *self {
            FlowDirection::Right => cursor.x = position.x + size.x,
            FlowDirection::Left => cursor.x = position.x,
            FlowDirection::Down => cursor.y = position.y + size.y,
            FlowDirection::Up => cursor.y = position.y,
        }

        cursor
    }
}
