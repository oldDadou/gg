extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

extern crate piston;
extern crate graphics;
extern crate sdl2_window;


use opengl_graphics::glyph_cache::GlyphCache;

use resources::*;
use specs::*;
use resources::*;

pub struct RenderDebugSystem<'a> {
    cache : GlyphCache<'a>,
}

impl<'a> RenderDebugSystem<'a> {
    pub fn new() -> RenderDebugSystem<'a> {
        let glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();

        RenderDebugSystem {
            cache: glyph_cache
        }
    }
}

impl<'a, 'b> System<'a> for RenderDebugSystem<'b> {
    type SystemData = (Fetch<'a, RenderArgsResource>,
    Fetch<'a, GameInputResources>,
    FetchMut<'a, opengl_graphics::GlGraphics>);


    fn run(&mut self, data: Self::SystemData) {

        let (resource, inputs, mut gl) = data;

        match resource.args {
            Some(args) => {
                gl.draw(args.viewport(), |c, g| {
                    use graphics::*;

                    text::Text::new_color([255.0, 0.0, 0.0, 1.0], 32).draw("Debug mod",
                    &mut self.cache,
                    &DrawState::default(),
                    c.transform
                    .trans(10.0, 20.0),
                    g);

                    let mouse_info = format!("x_screen: {}, y_screen: {}", inputs.cursor_position.0, inputs.cursor_position.1);
                    text::Text::new_color([255.0, 0.0, 0.0, 1.0], 32).draw(&mouse_info,
                    &mut self.cache,
                    &DrawState::default(),
                    c.transform
                    .trans(10.0, 50.0),
                    g);

                });
            }
            _ => {}
        }
    }

}
