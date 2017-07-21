use cgmath::{Vector2};
use palette::{Rgba};

mod size;
mod style;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    /// Positions the element using previous elements.
    Flow,
    /// Positions the element using a position relative to the parent.
    Relative(Vector2<f32>),
}

impl Position {
    pub fn is_flow(&self) -> bool {
        self == &Position::Flow
    }
}

#[derive(Clone, Debug)]
pub enum MouseBehavior {
    PassThrough,
    Clickable {
        hover: Option<Rgba>,
        hold: Option<Rgba>,
    },
}
