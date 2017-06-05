extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;

use specs::prelude::*;
use piston::input::*;
use tiled_map;
use std::rc::Rc;


pub struct RenderSystem<'a> {
    pub gl: &'a mut opengl_graphics::GlGraphics,
    pub texture: Rc<opengl_graphics::Texture>
}

pub struct RenderArgsResource {
    pub args: Option<RenderArgs>
}

impl Component for tiled_map::Map {
    type Storage = HashMapStorage<tiled_map::Map>;
}

impl<'a, 'b> System<'a> for RenderSystem<'b> {
    type SystemData = (Fetch<'a, RenderArgsResource>, ReadStorage<'a, tiled_map::Map>);

    fn run(&mut self, data: Self::SystemData) {
        use graphics::*;

        let (resource, map) = data;

        match resource.args {
            Some(args) => {

                let hack = self.texture.clone();
                self.gl.draw(args.viewport(), |c, g| {
                    let transform = c.transform.scale( (args.width as f64)/640f64, (args.height as f64)/576f64);

                    clear([255.0, 255.0, 255.0, 0.0], g);

                    let ref draw_state: DrawState = Default::default();

                    Image::new()
                        .rect([0.0, 0.0, 16.0, 16.0])
                        .src_rect([0.0, 0.0, 16.0, 16.0])
                        .draw(&*hack, draw_state, transform, g);

                    // image(&*hack, transform, g);

                });

                for b in (&map).join() {
                    let map: &tiled_map::Map = b;
                }

            },
            _ => {
            }
        }

    }
}
