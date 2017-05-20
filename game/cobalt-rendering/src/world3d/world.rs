use cgmath::{Vector3};
use world3d::{Model, Material};

#[derive(Default)]
pub struct World {
    entities: Vec<Entity>,
}

impl World {
    pub fn add(&mut self, entity: Entity) -> EntityId {
        self.entities.push(entity);
        self.entities.len() - 1
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn entity_mut(&mut self, id: EntityId) -> &mut Entity {
        &mut self.entities[id]
    }
}

pub type EntityId = usize;

pub struct Entity {
    pub position: Vector3<f32>,
    pub model: Model,
    pub material: Material,
}
