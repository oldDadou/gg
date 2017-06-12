extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate specs;
extern crate find_folder;

mod system_render;
mod system_camera;
mod system_input;

mod scene;

mod tiled_map;
mod renderable;
mod camera;
mod resources;

use resources::*;
use scene::*;
use piston::event_loop::*;
use piston::input::*;
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

    let mut scene = SceneBuilder::new()
        .map(&String::from("map.tmx"))
        .graphics(gl)
        .build();

    while let Some(e) = events.next(&mut window) {

        *scene.mut_world().write_resource::<RenderArgsResource>() =
            RenderArgsResource { args: None };

        *scene.mut_world().write_resource::<ResizeArgsResource>() =
            ResizeArgsResource { args: None };

        if let Some(button) = e.press_args() {
            scene.mut_world()
                .write_resource::<system_input::PressButtonResource>()
                .inputs
                .push(button);
        }

        if let Some(button) = e.release_args() {
            scene.mut_world().write_resource::<system_input::ReleaseButtonResource>()
                .inputs
                .push(button);
        }

        if let Some(args) = e.resize_args() {
            *scene.mut_world().write_resource::<ResizeArgsResource>() =
                ResizeArgsResource { args: Some((args[0], args[1])) };
        }

        if let Some(args) = e.render_args() {
            *scene.mut_world().write_resource::<RenderArgsResource>() =
                RenderArgsResource { args: Some(args.clone()) };
        }

        scene.update();
        // dispatcher.dispatch(&mut world.res);
    }
}
