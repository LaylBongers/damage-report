use cgmath::{Vector2};
use palette::{Rgba, Luma};

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

#[derive(Clone, Debug, PartialEq)]
pub enum CursorBehavior {
    /// Doesn't listen to cursor events, but clicks and hovering get blocked by it.
    Block,
    /// Doesn't listen to cursor events, clicks and hovering go right through.
    PassThrough,
    /// Can be clicked, can change colors if hovered over or clicked.
    Clickable {
        hover: Option<Rgba>,
        hold: Option<Rgba>,
    },
}

impl CursorBehavior {
    pub fn clickable(hover: Option<Rgba>, hold: Option<Rgba>) -> Self {
        CursorBehavior::Clickable {
            hover,
            hold,
        }
    }

    pub fn clickable_autocolor(base: Rgba) -> Self {
        let luma: Luma = base.into();

        let highlighted = highlighted_for(luma, base);
        let hold = hold_for(luma, base);

        CursorBehavior::clickable(Some(highlighted), Some(hold))
    }
}

fn highlighted_for(luma: Luma, color: Rgba) -> Rgba {
    if luma.luma > 0.8 {
        Rgba::new(color.red - 0.2, color.green - 0.2, color.blue - 0.2, color.alpha)
    } else if luma.luma < 0.2 {
        Rgba::new(color.red + 0.2, color.green + 0.2, color.blue + 0.2, color.alpha)
    } else {
        Rgba::new(
            clampf32((1.0 - color.red) * 0.5 * color.red + color.red),
            clampf32((1.0 - color.green) * 0.1 * color.green + color.green),
            clampf32((1.0 - color.blue) * 0.1 * color.blue + color.blue),
            color.alpha,
        )
    }
}

fn hold_for(luma: Luma, color: Rgba) -> Rgba {
    if luma.luma > 0.8 {
        Rgba::new(color.red, color.green - 0.2, color.blue - 0.2, color.alpha)
    } else if luma.luma < 0.2 {
        Rgba::new(color.red + 0.4, color.green + 0.2, color.blue + 0.2, color.alpha)
    } else {
        Rgba::new(
            clampf32((1.0 - color.red) * 0.75 * color.red + color.red),
            clampf32((1.0 - color.green) * 0.25 * color.green + color.green),
            clampf32((1.0 - color.blue) * 0.25 * color.blue + color.blue),
            color.alpha,
        )
    }
}

fn clampf32(f: f32) -> f32 {
    if f < 0.0 { 0.0 } else if f > 1.0 { 1.0 } else { f }
}
