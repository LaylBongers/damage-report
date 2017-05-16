use cgmath::{Vector3};

#[derive(Default)]
pub struct World {
    pub positions: Vec<Vector3<f32>>
}

impl World {
    pub fn add(&mut self, position: Vector3<f32>) {
        self.positions.push(position);
    }
}
