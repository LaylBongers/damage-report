use cgmath::{Vector2};

#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    /// Positions the element using previous elements.
    Flow,
    /// Positions the element using a position relative to the parent.
    Relative(Vector2<f32>, SideH, SideV),
}

impl Position {
    pub fn is_flow(&self) -> bool {
        self == &Position::Flow
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SideH {
    Left, Middle, Right,
}

impl SideH {
    pub fn relative_position(&self, docked_position: f32, size: f32, container: f32) -> f32 {
        match *self {
            SideH::Left => docked_position,
            SideH::Middle => container*0.5 - size*0.5 + docked_position,
            SideH::Right => container - size + docked_position,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SideV {
    Top, Middle, Bottom,
}


impl SideV {
    pub fn relative_position(&self, docked_position: f32, size: f32, container: f32) -> f32 {
        match *self {
            SideV::Top => docked_position,
            SideV::Middle => container*0.5 - size*0.5 + docked_position,
            SideV::Bottom => container - size + docked_position,
        }
    }
}
