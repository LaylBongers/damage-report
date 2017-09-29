//#![allow(dead_code)]

use std::sync::{Arc};
use std::path::{PathBuf};

use cgmath::{Vector2, Point2};
use input::{Input, Button, Key, ButtonState, ButtonArgs};
use window::{Window, WindowSettings};
use slog::{Logger};
use calcium_flowy::FlowyRenderer;
use flowy::{Ui, Element};
use flowy::style::{Style, Position, Size, SideH, SideV};
use palette::pixel::{Srgb};
use rusttype::{FontCollection};
use tiled;

use calcium_game::{LoopTimer};
use calcium_rendering::{Error};
use calcium_rendering::texture::{Texture};
use calcium_rendering_2d::render_data::{RenderBatch, ShaderMode, DrawRectangle, Rectangle, Projection, RenderData, RenderSet, UvMode};
use calcium_rendering_2d::{Renderer2DTarget};
use calcium_rendering_context::{Runtime, Context};
use calcium_rendering::raw::{RendererRaw};

use model::{Map};
use view::{MapRenderer};

struct FriendlyUnit<R: RendererRaw> {
    name: String,
    tex: Arc<Texture<R>>,
    selecttex: Arc<Texture<R>>,
    position: Point2<f32>,
    size: Vector2<f32>,
    speed: f32,
    selected: bool,

    tabrel: f32,
}

impl <R: RendererRaw> FriendlyUnit<R> {
    pub fn new(name: String, tex: Arc<Texture<R>>, selecttex: Arc<Texture<R>>, position: Point2<f32>, size: Vector2<f32>, speed: f32) -> FriendlyUnit<R> {
        FriendlyUnit {name: name, tex: tex, selecttex: selecttex, position: position, size: size, speed: speed, selected: false, tabrel: 0.0}
    }

    pub fn update(&mut self, delta: f32, selected: bool, pinput: &PlayerInput) {
        /* do update-y things */
        self.tabrel -= delta;
        if self.tabrel <= 0.0 && pinput.tab {
            //println!("I am {}, Selection Status: {}.", self.name, selected);
            self.tabrel = 0.1;

            self.selected = selected;
        }
        if self.selected {
            if pinput.w {self.position.y -= self.speed * delta;}
            if pinput.a {self.position.x -= self.speed * delta;}
            if pinput.s {self.position.y += self.speed * delta;}
            if pinput.d {self.position.x += self.speed * delta;}
        }
    }
    pub fn render(&mut self, batches: &mut Vec<RenderBatch<R>>) {
        //let mut batches = Vec::new();
        let mut normaltexture = RenderBatch::new(
            ShaderMode::Texture(self.tex.clone()), UvMode::YDown
        );
        normaltexture.push_rectangle(DrawRectangle::full_texture(
            // position is centered in the texture
            Rectangle::new(self.position + -self.size/2.0, self.position + self.size/2.0)
        ));
        batches.push(normaltexture);

        if self.selected {
            let mut selectiontexture = RenderBatch::new(
                ShaderMode::Texture(self.selecttex.clone()), UvMode::YDown
            );
            selectiontexture.push_rectangle(DrawRectangle::full_texture(
                Rectangle::new(self.position + -self.size, self.position + self.size)
            ));
            batches.push(selectiontexture);
        }
    }
    pub fn get_position(&mut self) -> Point2<f32> {
        self.position
    }
    pub fn get_name(&mut self) -> &String {
        &self.name
    }
}

struct PlayerInput {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub tab: bool,
}

pub struct StaticRuntime {
    pub log: Logger,
}

impl Runtime for StaticRuntime {
    fn run<C: Context>(self, context: C) -> Result<(), Error> {
        info!(self.log, "Loading program");

        // Set up everything we need to render
        let window_settings = WindowSettings::new("RPG Game", [1280, 720]);
        let (mut renderer, mut window) =
            context.renderer(Some(self.log.clone()), &window_settings)?;
        let mut simple2d_renderer = context.simple2d_renderer(&mut renderer)?;
        let mut simple2d_render_target = Renderer2DTarget::new(
            true, &renderer, &simple2d_renderer
        );

        let mut ui_renderer = FlowyRenderer::new(&mut renderer)?;
        let mut ui = Ui::new();
        let root_id = ui.elements.root_id();

        let font = FontCollection::from_bytes(
            ::ttf_noto_sans::REGULAR
        ).into_font().unwrap();
        ui.fonts.push(font);

        let fps = Element::new(Style {
            position: Position::Relative(Point2::new(0.0, 0.0), SideH::Right, SideV::Top),
            size: Size::units(120.0, 14.0),
            text_color: Srgb::new(1.0, 1.0, 1.0).into(),
            text_size: 14.0,
            .. Style::new()
        });
        let fps_id = ui.elements.add_child(fps, root_id);

        {
            let fpso = &mut ui.elements[fps_id];
            fpso.set_text(format!("test text"));
        }

        // Units data
        let friendly_texture = Texture::new()
            .from_file("./assets/friendly.png")
            .with_nearest_sampling()
            .build(&mut renderer)?;
        let selection_texture = Texture::new()
            .from_file("./assets/selection.png")
            .with_nearest_sampling()
            .build(&mut renderer)?;

        // Set up the game map's tiles
        let map_path = PathBuf::from("./assets/test_map.tmx");
        let tmap = tiled::parse_file(&map_path).unwrap();
        let map = Map::new(&tmap, &self.log);
        let map_renderer = MapRenderer::new(&tmap, &map_path, &mut renderer)?;

        let mut players_units = Vec::new();

        let alfred = FriendlyUnit::new(String::from("Alfred"), friendly_texture.clone(), selection_texture.clone(), Point2::new(200.0,200.0), Vector2::new(32.0,32.0), 256.0 );
        let bertil = FriendlyUnit::new(String::from("Bertil"), friendly_texture.clone(), selection_texture.clone(), Point2::new(300.0,300.0), Vector2::new(32.0,32.0), 256.0 );
        let carl = FriendlyUnit::new(String::from("Carl"), friendly_texture.clone(), selection_texture.clone(), Point2::new(400.0,400.0), Vector2::new(32.0,32.0), 256.0 );
        let dagobert = FriendlyUnit::new(String::from("Dagobert"), friendly_texture.clone(), selection_texture.clone(), Point2::new(300.0,500.0), Vector2::new(32.0,32.0), 256.0 );

        players_units.push(alfred);
        players_units.push(bertil);
        players_units.push(carl);
        players_units.push(dagobert);

        let (mut selected_unit, mut tabrelease) = (3,0.1);

        let (mut left_pressed, mut right_pressed, mut up_pressed, mut down_pressed, mut tab_pressed) =
            (false, false, false, false, false);

        // Run the actual game loop
        let mut timer = LoopTimer::start();
        info!(self.log, "Finished loading, starting main loop");
        while !window.should_close() {
            let delta = timer.tick();

            // Handle input
            while let Some(event) = window.poll_event() {
                // Let the context handle anything needed
                context.handle_event(&event, &mut renderer, &mut window);

                match event {
                    Input::Button(ButtonArgs {state, button, scancode: _scancode}) => {
                        let press = state == ButtonState::Press;
                        match button {
                            Button::Keyboard(Key::A) =>
                                left_pressed = press,
                            Button::Keyboard(Key::D) =>
                                right_pressed = press,
                            Button::Keyboard(Key::W) =>
                                up_pressed = press,
                            Button::Keyboard(Key::S) =>
                                down_pressed = press,

                            Button::Keyboard(Key::Tab) =>
                                tab_pressed = press,

                            _ => {},
                        }
                    },
                    _ => {},
                }
            }

            let pinput = PlayerInput {w: up_pressed, a: left_pressed, s: down_pressed, d: right_pressed, tab: tab_pressed};

            {
                let fpso = &mut ui.elements[fps_id];
                fpso.style_mut().position = Position::Relative(players_units[selected_unit].get_position(), SideH::Left, SideV::Top);
                fpso.set_text(players_units[selected_unit].get_name().clone());
            }

            // TODO: kill this
            tabrelease -= delta;
            if tabrelease <= 0.0 && tab_pressed {
                if selected_unit == 3 {
                    selected_unit = 0;
                }
                else {
                    selected_unit+=1;
                }
                tabrelease = 0.1;
                println!("selected unit is now {}", selected_unit);
            }

            // Update the player units
            for (i, unit) in players_units.iter_mut().enumerate() {
                unit.update(delta, i == selected_unit, &pinput);
            }

            // Set up the rendering data we'll need
            let mut render_data = RenderData::new();
            let mut world_batches = Vec::new();
            let camera_size = renderer.size().cast();

            // Render the tiles
            map_renderer.render(&map, &mut world_batches, camera_size);

            // Render the player units
            for unit in &mut players_units {
                unit.render(&mut world_batches);
            }

            // Submit the world render data
            //let camera = Camera::new(32.0, Point2::new(0.0, 0.0));
            //Projection::Camera(camera)
            render_data.render_sets.push(RenderSet::new(Projection::Pixels, world_batches));

            // Render the UI
            let mut ui_batches = Vec::new();
            ui_renderer.render(&mut ui, &mut ui_batches, camera_size, &mut renderer)?;
            render_data.render_sets.push(RenderSet::new(Projection::Pixels, ui_batches));

            // Finally do the 2D rendering itself
            let mut frame = renderer.start_frame();
            simple2d_renderer.render(
                &render_data, &mut frame, &mut simple2d_render_target, &mut renderer
            );
            renderer.finish_frame(frame);
            window.swap_buffers();
        }

        Ok(())
    }
}
