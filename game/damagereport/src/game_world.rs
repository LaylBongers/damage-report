use cgmath::{Vector3};
use slog::{Logger};
use cobalt_rendering::{Target, Texture};
use cobalt_rendering_world3d::{World, Model, Entity, Light, EntityId, LightId, Material};

use input::{InputState, FrameInput};
use player::{Player};

pub struct GameWorld {
    pub player: Player,
    devices: Vec<Device>,
    light: LightId,
    light_accum: f32,
}

impl GameWorld {
    pub fn init(log: &Logger, target: &mut Target, world: &mut World) -> Self {
        let player = Player::new();

        // Create the floor
        let floor_model = Model::load(log, target, "./assets/floor.obj", 0.1);
        let floor_material = Material {
            base_color: Texture::load(log, target, "./assets/texture.png"),
        };
        world.add(Entity {
            position: Vector3::new(0.0, 0.0, 0.0),
            mesh: floor_model.meshes[0].clone(),
            material: floor_material.clone(),
        });

        // Create the 3 test devices
        let device_model = Model::load(log, target, "./assets/device.obj", 0.1);
        let material_working = Material {
            base_color: Texture::load(log, target, "./assets/texture_broken.png"),
        };
        let material_broken = Material {
            base_color: Texture::load(log, target, "./assets/texture_working.png"),
        };
        let d1 = Device::new(
            world, Vector3::new(-2.0, 0.0, -4.0), &device_model,
            &floor_material, &material_working, &material_broken,
        );
        let d2 = Device::new(
            world, Vector3::new( 0.0, 0.0, -4.0), &device_model,
            &floor_material, &material_working, &material_broken,
        );
        let d3 = Device::new(
            world, Vector3::new( 2.0, 0.0, -4.0), &device_model,
            &floor_material, &material_working, &material_broken,
        );
        let devices = vec!(d1, d2, d3);

        // Create a centered test light
        let light = world.add_light(Light {
            position: Vector3::new(0.0, 1.5, 0.0),
            color: Vector3::new(1.0, 1.0, 1.0),
        });

        GameWorld {
            player,
            devices,

            // TODO: Remove these two
            light,
            light_accum: 0.0,
        }
    }

    pub fn update(
        &mut self, log: &Logger, time: f32, world: &mut World,
        input_state: &InputState, frame_input: &FrameInput
    ) {
        // Update the player based on the input we got so far
        self.player.update(&input_state, &frame_input, time);

        // Update the devices
        for device in &mut self.devices {
            device.update(log, time, world);
        }

        // Rotate the light around
        self.light_accum += time;
        let light = world.light_mut(self.light);
        light.position.x = f32::sin(self.light_accum) * 4.0;
        light.position.z = f32::cos(self.light_accum) * 4.0;
    }
}

struct Device {
    fixedness: f32,
    status: bool,
    light_entity: EntityId,
    material_working: Material,
    material_broken: Material,
}

impl Device {
    fn new(
        world: &mut World, position: Vector3<f32>, model: &Model,
        material_base: &Material, material_working: &Material, material_broken: &Material
    ) -> Self {
        world.add(Entity {
            position,
            mesh: model.meshes[0].clone(),
            material: material_base.clone(),
        });
        let light_entity = world.add(Entity {
            position,
            mesh: model.meshes[1].clone(),
            material: material_working.clone(),
        });

        Device {
            fixedness: 1.0,
            status: true,
            light_entity,
            material_working: material_working.clone(),
            material_broken: material_broken.clone(),
        }
    }

    fn set_status(&mut self, log: &Logger, value: bool) {
        self.status = value;
        info!(log, "Switched device status to {}", self.status);
    }

    fn update(&mut self, log: &Logger, time: f32, world: &mut World) {
        if self.status {
            self.fixedness -= time;
        } else {
            self.fixedness += time;
        }

        if self.fixedness < 0.0 && self.status {
            self.set_status(log, false);

            let entity = world.entity_mut(self.light_entity);
            entity.material = self.material_broken.clone();
        }
        if self.fixedness > 1.0 && !self.status {
            self.set_status(log, true);

            let entity = world.entity_mut(self.light_entity);
            entity.material = self.material_working.clone();
        }
    }
}
