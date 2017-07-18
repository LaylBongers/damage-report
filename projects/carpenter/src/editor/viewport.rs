use cgmath::{Vector3, Quaternion, One};

use calcium_rendering::{Error, Types, Texture, TextureFormat};
use calcium_rendering_world3d::{RenderWorld, Camera, World3DRenderer, Entity, World3DTypes, Model, Material, World3DRenderTarget};

pub struct EditorViewport<T: Types, WT: World3DTypes<T>> {
    camera: Camera,
    render_world: RenderWorld<T, WT>,
}

impl<T: Types, WT: World3DTypes<T>> EditorViewport<T, WT> {
    pub fn new(renderer: &mut T::Renderer) -> Result<Self, Error> {
        let camera = Camera::new(
            Vector3::new(0.0, 2.0, 5.0),
            Quaternion::one(),
        );
        let mut render_world = RenderWorld::new();
        seed_world(&mut render_world, renderer)?;

        Ok(EditorViewport {
            camera,
            render_world,
        })
    }

    pub fn render(
        &self,
        frame: &mut T::Frame,
        renderer: &mut T::Renderer,
        window_renderer: &mut T::WindowRenderer,
        world3d_renderer: &mut WT::Renderer,
        world3d_rendertarget: &mut World3DRenderTarget<T, WT>,
    ) {
        world3d_renderer.render(
            &self.render_world, &self.camera, world3d_rendertarget,
            renderer, window_renderer, frame
        );
    }
}

fn seed_world<T: Types, WT: World3DTypes<T>>(
    world: &mut RenderWorld<T, WT>, renderer: &mut T::Renderer
) -> Result<(), Error> {
    world.ambient_light = Vector3::new(0.05, 0.05, 0.05);
    world.directional_light = Vector3::new(1.0, 1.0, 1.0);

    let model = Model::<T, WT>::load(renderer, "./assets/cube.obj", 1.0);
    let material = Material {
        base_color: Texture::from_file(
            renderer, "./assets/texture.png", TextureFormat::Srgb
        )?,
        normal_map: Texture::from_file(
            renderer, "./assets/texture_normal.png", TextureFormat::Linear
        )?,
        metallic_map: Texture::from_file(
            renderer, "./assets/texture_metallic.png", TextureFormat::LinearRed
        )?,
        roughness_map: Texture::from_file(
            renderer, "./assets/texture_roughness.png", TextureFormat::LinearRed
        )?,
    };

    world.add_entity(Entity {
        position: Vector3::new(0.0, 0.0, 0.0),
        mesh: model.meshes[0].clone(),
        material: material,
    });

    Ok(())
}
