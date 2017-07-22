use cgmath::{Vector2};

#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    /// Positions the element using previous elements.
    Flow,
    /// Positions the element using a position relative to the parent.
    Relative(Vector2<f32>, DockH, DockV),
}

impl Position {
    pub fn is_flow(&self) -> bool {
        self == &Position::Flow
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DockH {
    Left, Middle, Right,
}

impl DockH {
    pub fn relative_position(&self, docked_position: f32, size: f32, container: f32) -> f32 {
        match *self {
            DockH::Left => docked_position,
            DockH::Middle => container*0.5 - size*0.5 + docked_position,
            DockH::Right => container - size + docked_position,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DockV {
    Top, Middle, Bottom,
}


impl DockV {
    pub fn relative_position(&self, docked_position: f32, size: f32, container: f32) -> f32 {
        match *self {
            DockV::Top => docked_position,
            DockV::Middle => container*0.5 - size*0.5 + docked_position,
            DockV::Bottom => container - size + docked_position,
        }
    }
}
