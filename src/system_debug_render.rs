extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

extern crate piston;
extern crate graphics;
extern crate sdl2_window;

use piston::window::{WindowSettings, Size};
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::*;
use opengl_graphics::glyph_cache::GlyphCache;
use sdl2_window::Sdl2Window;
use camera::*;
use renderable::*;
use specs::*;
use tiled_map;
use resources::*;

pub struct RenderDebugSystem<'a> {
    cache : GlyphCache<'a>,
}

impl<'a> RenderDebugSystem<'a> {
    pub fn new() -> RenderDebugSystem<'a> {
        let mut glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();

        RenderDebugSystem {
            cache: glyph_cache
        }
    }
}

impl<'a, 'b> System<'a> for RenderDebugSystem<'b> {
    type SystemData = (Fetch<'a, RenderArgsResource>,
    FetchMut<'a, opengl_graphics::GlGraphics>);


    fn run(&mut self, data: Self::SystemData) {

        let (resource, mut gl) = data;

        match resource.args {
            Some(args) => {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::*;

                    text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32).draw("Hello opengl_graphics!",
                    &mut self.cache,
                    &DrawState::default(),
                    c.transform
                    .trans(10.0, 100.0),
                    g);
                });
            }
            _ => {}
        }
    }

}
