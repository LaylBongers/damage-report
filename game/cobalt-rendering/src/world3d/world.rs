use cgmath::{Vector3};
use world3d::{Model};

#[derive(Default)]
pub struct World {
    pub entities: Vec<Entity>,
}

impl World {
    pub fn add(&mut self, position: Vector3<f32>, model: Model) {
        self.entities.push(Entity {
            position: position,
            model: model,
        });
    }
}

pub struct Entity {
    pub position: Vector3<f32>,
    pub model: Model,
}
