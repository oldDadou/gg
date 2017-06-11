extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate specs;
extern crate find_folder;

use specs::prelude::*;

mod system_render;
mod system_camera;
mod system_input;

mod tiled_map;
mod camera;

use camera::*;

use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: hello_world", [600, 600])
        .exit_on_esc(true)
        .vsync(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let sys: system_render::RenderSystem = system_render::RenderSystem::new(&mut gl);
    let camesys: system_camera::CameraSystem = system_camera::CameraSystem::new();

    let camera = Camera {
        position: [5f64, 5f64],
        area: [10f64, 10f64],
    };

    let mut world = World::new();

    world.register::<tiled_map::Map>();
    world.register::<Camera>();

    world.create_entity().with(tiled_map::Map::new()).build();
    world.create_entity().with(camera).build();

    // Let's use some start value
    world.add_resource(system_render::RenderArgsResource { args: None });

    // Init input resources
    world.add_resource(system_input::PressButtonResource { inputs: vec![] });
    world.add_resource(system_input::ReleaseButtonResource { inputs: vec![] });
    world.add_resource(system_input::GameInputResources::new());

    let mut dispatcher = DispatcherBuilder::new()
        .add_thread_local(system_input::InputSystem {})
        .add_thread_local(camesys)
        .add_thread_local(sys)
        .build();

    while let Some(e) = events.next(&mut window) {
        *world.write_resource::<system_render::RenderArgsResource>() =
            system_render::RenderArgsResource { args: None };

        if let Some(button) = e.press_args() {
            world
                .write_resource::<system_input::PressButtonResource>()
                .inputs
                .push(button);
        }

        if let Some(button) = e.release_args() {
            world
                .write_resource::<system_input::ReleaseButtonResource>()
                .inputs
                .push(button);
        }

        if let Some(args) = e.render_args() {
            *world.write_resource::<system_render::RenderArgsResource>() =
                system_render::RenderArgsResource { args: Some(args.clone()) };
        }
        dispatcher.dispatch(&mut world.res);

    }
}
