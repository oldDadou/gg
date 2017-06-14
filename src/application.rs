extern crate opengl_graphics;
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate specs;
extern crate find_folder;

use assets_manager::*;
use resources::*;
use scene::*;
use piston::event_loop::*;
use piston::input::*;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;
use tiled_map::*;
use camera::*;
use renderable::*;
use specs::*;

pub struct Application<'win, 'a, 'b> {
    scene: Option<Scene<'a, 'b>>,
    world: World,
    assets: AssetsManager,
    window: &'win mut Sdl2Window
}

impl<'win, 'a, 'b> Application<'win, 'a, 'b> {

    pub fn build(win: &'win mut Sdl2Window) -> Application<'win, 'a, 'b> {

        let opengl = OpenGL::V3_2;
        let gl = GlGraphics::new(opengl);

        let mut assets = AssetsManager::new();

        let mut world = World::new();

        world.register::<Map>();
        world.register::<Camera>();
        world.register::<Renderable>();

        // Let's use some start value
        world.add_resource(RenderArgsResource { args: None });
        world.add_resource(ResizeArgsResource { args: None });
        world.add_resource(InputArgsResources { args: None });
        world.add_resource(DeltaTime { dt: 0f64 });

        // Init input resources
        world.add_resource(PressButtonResource { inputs: vec![] });
        world.add_resource(ReleaseButtonResource { inputs: vec![] });
        world.add_resource(GameInputResources::new());

        let scene = Some(SceneBuilder::new()
            .map(&String::from("map.tmx"))
            .manager(&mut assets)
            .graphics(gl)
            .build(&mut world));

        Application{
            scene: scene,
            assets: assets,
            world: world,
            window: win
        }

    }

    pub fn run(&mut self) {

        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(self.window) {

            *self.world.write_resource::<RenderArgsResource>() =
                RenderArgsResource { args: None };

            *self.world.write_resource::<ResizeArgsResource>() =
                ResizeArgsResource { args: None };

            if let Some(update) = e.update_args() {
                self.world.write_resource::<DeltaTime>().dt = update.dt;
            }

            if let Some(args) = e.resize_args() {
                *self.world.write_resource::<ResizeArgsResource>() =
                    ResizeArgsResource { args: Some((args[0], args[1])) };
            }

            if let Some(button) = e.press_args() {
                self.world
                    .write_resource::<PressButtonResource>()
                    .inputs
                    .push(button);
            }

            if let Some(button) = e.release_args() {
                self.world
                    .write_resource::<ReleaseButtonResource>()
                    .inputs
                    .push(button);
            }

            if let Some(args) = e.render_args() {
                *self.world.write_resource::<RenderArgsResource>() =
                    RenderArgsResource { args: Some(args.clone()) };
            }

            if let Some(_) = e.mouse_cursor_args() {
                *self.world.write_resource::<InputArgsResources>() =
                    InputArgsResources { args: Some(e.clone()) };
            }

            match self.scene {
                Some(ref mut scene) => {
                    scene.update(&mut self.world);
                },
                _ => {}
            }
        }
    }
}
