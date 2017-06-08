extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate specs;
extern crate find_folder;

use specs::prelude::*;

mod system_render;
mod tiled_map;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;

use opengl_graphics::*;


fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Sdl2Window =
        WindowSettings::new("opengl_graphics: hello_world", [600, 600])
        .exit_on_esc(true)
        .vsync(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let sys: system_render::RenderSystem = system_render::RenderSystem::new(&mut gl);

    let mut world = World::new();

    world.register::<tiled_map::Map>();

    world.create_entity().with(tiled_map::Map::new()).build();
    world.add_resource(system_render::RenderArgsResource {args: None}); // Let's use some start value

    let mut dispatcher = DispatcherBuilder::new().add_thread_local(sys).build();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            *world.write_resource::<system_render::RenderArgsResource>() = system_render::RenderArgsResource {args: Some(args.clone())};
            dispatcher.dispatch(&mut world.res);
        }
    }
}
