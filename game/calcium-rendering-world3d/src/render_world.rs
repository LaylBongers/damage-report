use std::sync::{Arc};
use cgmath::{Vector3, InnerSpace};
use {Mesh, Material};

pub struct RenderWorld {
    entities: Vec<Entity>,
    lights: Vec<Light>,

    pub ambient_light: Vector3<f32>,
    pub directional_light: Vector3<f32>,
    pub directional_direction: Vector3<f32>,
}

impl RenderWorld {
    pub fn new() -> Self {
        RenderWorld {
            entities: Vec::new(),
            lights: Vec::new(),

            ambient_light: Vector3::new(0.0, 0.0, 0.0),
            directional_light: Vector3::new(0.0, 0.0, 0.0),
            directional_direction: Vector3::new(-1.0, 1.5, -0.5).normalize(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        self.entities.push(entity);
        EntityId(self.entities.len() - 1)
    }

    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub fn entity_mut(&mut self, id: EntityId) -> &mut Entity {
        &mut self.entities[id.0]
    }

    pub fn add_light(&mut self, light: Light) -> LightId {
        self.lights.push(light);
        LightId(self.lights.len() - 1)
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn light_mut(&mut self, id: LightId) -> &mut Light {
        &mut self.lights[id.0]
    }
}

#[derive(Copy, Clone)]
pub struct EntityId(usize);

#[derive(Copy, Clone)]
pub struct LightId(usize);

pub struct Entity {
    pub position: Vector3<f32>,
    pub mesh: Arc<Mesh>,
    pub material: Material,
}

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub radius: f32,
}
