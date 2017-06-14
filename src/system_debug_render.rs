extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

extern crate piston;
extern crate graphics;
extern crate sdl2_window;

use opengl_graphics::glyph_cache::GlyphCache;

use tiled_map::*;
use camera::*;
use resources::*;
use specs::*;

pub struct RenderDebugSystem<'a> {
    cache: GlyphCache<'a>,
}

impl<'a> RenderDebugSystem<'a> {
    pub fn new() -> RenderDebugSystem<'a> {
        let glyph_cache = GlyphCache::new("assets/FiraSans-Regular.ttf").unwrap();

        RenderDebugSystem { cache: glyph_cache }
    }
}

impl<'a, 'b> System<'a> for RenderDebugSystem<'b> {
    type SystemData = (Fetch<'a, RenderArgsResource>,
     Fetch<'a, DeltaTime>,
     Fetch<'a, GameInputResources>,
     ReadStorage<'a, Camera>,
     ReadStorage<'a, Map>,
     FetchMut<'a, opengl_graphics::GlGraphics>);


    fn run(&mut self, data: Self::SystemData) {
        let (resource, dt, inputs, cameras, maps, mut gl) = data;

        for camera in cameras.join() {
            for _ in maps.join() {

                match resource.args {
                    Some(args) => {

                        gl.draw(args.viewport(), |c, g| {
                            use graphics::*;

                            let mut current_line = 1f64;
                            let y_offset = 5f64;
                            let text_size = 16f64;

                            let camera_info = format!("camera: {:?}", camera);
                            text::Text::new_color([255.0, 0.0, 0.0, 1.0], text_size as u32)
                                .draw(&camera_info,
                                      &mut self.cache,
                                      &DrawState::default(),
                                      c.transform.trans(y_offset, text_size * current_line),
                                      g);

                            current_line += 1f64;
                            let camera_info = format!("camera: {:?} width: {}, height: {}",
                                                      get_edge_position(camera),
                                                      camera_viewport_width(camera),
                                                      camera_viewport_height(camera));

                            text::Text::new_color([255.0, 0.0, 0.0, 1.0], text_size as u32)
                                .draw(&camera_info,
                                      &mut self.cache,
                                      &DrawState::default(),
                                      c.transform.trans(y_offset, text_size * current_line),
                                      g);


                            current_line += 1f64;

                            let pos = screen_position_to_position(camera,
                                                                  &[args.width as f64,
                                                                    args.height as f64],
                                                                  &[inputs.cursor_position.0,
                                                                    inputs.cursor_position.1]);
                            let camera_info =
                                format!("cursor pos: {}, {}", pos[0] as u32, pos[1] as u32);
                            text::Text::new_color([255.0, 0.0, 0.0, 1.0], text_size as u32)
                                .draw(&camera_info,
                                      &mut self.cache,
                                      &DrawState::default(),
                                      c.transform.trans(y_offset, text_size * current_line),
                                      g);

                        });
                    }
                    _ => {}
                }
            }
        }
    }
}
