extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate specs;
extern crate find_folder;

mod system_map_render;
mod system_debug_render;
mod system_camera;
mod system_input;

mod assets_manager;

mod application;
mod scene;

mod tiled_map;
mod renderable;
mod camera;
mod resources;

use application::*;
use assets_manager::*;
use piston::event_loop::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Sdl2Window = WindowSettings::new("opengl_graphics: hello_world", [1280, 720])
        .exit_on_esc(true)
        .vsync(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());

    let gl = GlGraphics::new(opengl);

    let mut app = Application::build(&mut window);
    app.run();

}
