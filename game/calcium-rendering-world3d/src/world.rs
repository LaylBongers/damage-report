use cgmath::{Vector3};
use {Mesh, Material};

pub struct World {
    entities: Vec<Entity>,
    ambient_light: Vector3<f32>,
    lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
            ambient_light: Vector3::new(0.0, 0.0, 0.0),
            lights: Vec::new(),
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

    pub fn set_ambient_light(&mut self, value: Vector3<f32>) {
        self.ambient_light = value;
    }

    pub fn ambient_light(&self) -> Vector3<f32> {
        self.ambient_light
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
    pub mesh: Mesh,
    pub material: Material,
}

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub radius: f32,
}
