use std::sync::{Arc};
use cgmath::{Vector3, InnerSpace};

use calcium_rendering::{Renderer};

use {Material, World3DRenderer, Mesh};

pub struct RenderWorld<R: RendererRaw, WR: World3DRenderer<R>> {
    entities: Vec<Option<Entity<R, WR>>>,
    lights: Vec<Light>,

    pub ambient_light: Vector3<f32>,
    pub directional_light: Vector3<f32>,
    pub directional_direction: Vector3<f32>,
}

impl<R: RendererRaw, WR: World3DRenderer<R>> RenderWorld<R, WR> {
    pub fn new() -> Self {
        RenderWorld {
            entities: Vec::new(),
            lights: Vec::new(),

            ambient_light: Vector3::new(0.0, 0.0, 0.0),
            directional_light: Vector3::new(0.0, 0.0, 0.0),
            directional_direction: Vector3::new(-1.0, 1.5, -0.5).normalize(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity<R, WR>) -> EntityId {
        // TODO: Find empty entity slots

        self.entities.push(Some(entity));
        EntityId(self.entities.len() - 1)
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        self.entities[id.0] = None;
    }

    pub fn entities(&self) -> &Vec<Option<Entity<R, WR>>> {
        &self.entities
    }

    pub fn entity_mut(&mut self, id: EntityId) -> &mut Entity<R, WR> {
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

pub struct Entity<R: RendererRaw, WR: World3DRenderer<R>> {
    pub position: Vector3<f32>,
    pub mesh: Arc<Mesh<R, WR>>,
    pub material: Material<R>,
}

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub radius: f32,
}
