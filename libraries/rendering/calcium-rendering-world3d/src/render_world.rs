use std::sync::{Arc};
use cgmath::{Vector3, InnerSpace};

use calcium_rendering::{Types};

use {Material, World3DTypes};

pub struct RenderWorld<T: Types, WT: World3DTypes<T>> {
    entities: Vec<Option<Entity<T, WT>>>,
    lights: Vec<Light>,

    pub ambient_light: Vector3<f32>,
    pub directional_light: Vector3<f32>,
    pub directional_direction: Vector3<f32>,
}

impl<T: Types, WT: World3DTypes<T>> RenderWorld<T, WT> {
    pub fn new() -> Self {
        RenderWorld {
            entities: Vec::new(),
            lights: Vec::new(),

            ambient_light: Vector3::new(0.0, 0.0, 0.0),
            directional_light: Vector3::new(0.0, 0.0, 0.0),
            directional_direction: Vector3::new(-1.0, 1.5, -0.5).normalize(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity<T, WT>) -> EntityId {
        // TODO: Find an empty entity

        self.entities.push(Some(entity));
        EntityId(self.entities.len() - 1)
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        // TODO: IMPORTANT, implement backend mesh unloading
        self.entities[id.0] = None;
    }

    pub fn entities(&self) -> &Vec<Option<Entity<T, WT>>> {
        &self.entities
    }

    pub fn entity_mut(&mut self, id: EntityId) -> &mut Entity<T, WT> {
        self.entities[id.0].as_mut().unwrap()
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

pub struct Entity<T: Types, WT: World3DTypes<T>> {
    pub position: Vector3<f32>,
    pub mesh: Arc<WT::Mesh>,
    pub material: Material<T>,
}

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub radius: f32,
}
