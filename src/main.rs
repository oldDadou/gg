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

mod scene;

mod tiled_map;
mod renderable;
mod camera;
mod resources;

use assets_manager::*;
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

    let mut assets = AssetsManager::new();

    let mut scene = SceneBuilder::new()
        .map(&String::from("map.tmx"))
        .manager(&mut assets)
        .graphics(gl)
        .build();

    while let Some(e) = events.next(&mut window) {

        *scene.mut_world().write_resource::<RenderArgsResource>() =
            RenderArgsResource { args: None };

        *scene.mut_world().write_resource::<ResizeArgsResource>() =
            ResizeArgsResource { args: None };


        if let Some(args) = e.resize_args() {
            *scene.mut_world().write_resource::<ResizeArgsResource>() =
                ResizeArgsResource { args: Some((args[0], args[1])) };
        }

        if let Some(button) = e.press_args() {
            scene.mut_world()
                .write_resource::<PressButtonResource>()
                .inputs
                .push(button);
        }

        if let Some(button) = e.release_args() {
            scene.mut_world().write_resource::<ReleaseButtonResource>()
                .inputs
                .push(button);
        }

        if let Some(args) = e.render_args() {
            *scene.mut_world().write_resource::<RenderArgsResource>() =
                RenderArgsResource { args: Some(args.clone()) };
        }

        if let Some(pos) = e.mouse_cursor_args() {
            let (x, y) = (pos[0] as f32, pos[1] as f32);
            *scene.mut_world().write_resource::<InputArgsResources>() =
                InputArgsResources { args: Some(e.clone()) };
        }

        scene.update();
        // dispatcher.dispatch(&mut world.res);
    }
}
