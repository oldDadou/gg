extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

use camera::*;
use renderable::*;
use specs::*;
use tiled_map;
use resources::*;


#[derive(Debug)]
pub struct TilesGridAccessor {
    pub tiles_size: (u32, u32),
    pub surface_size: (u32, u32),
}

impl TilesGridAccessor {

    /// Return the rect containing the sprite (ready to draw)
    pub fn access_sprite(&self, idx: u32) -> [f64; 4] {
        let x = (self.tiles_size.0) * ((idx - 1) % (self.surface_size.0 / self.tiles_size.0));
        let y = self.tiles_size.1 * (idx / (self.surface_size.0 / self.tiles_size.0));

        [x as f64,
         y as f64,
         self.tiles_size.0 as f64,
         self.tiles_size.1 as f64]
    }

}

#[inline]
fn position_to_screen_position(camera: &Camera, rect: &[f64; 2]) -> [f64; 2] {
    let came_pos = get_edge_position(camera);

    [rect[0] - came_pos[0], rect[1] - came_pos[1]]
}

pub struct RenderMapSystem {
}

impl RenderMapSystem {
    pub fn new() -> RenderMapSystem {
        RenderMapSystem {
        }
    }
}

impl<'a> System<'a> for RenderMapSystem {
    type SystemData = (Fetch<'a, RenderArgsResource>,
     FetchMut<'a, opengl_graphics::GlGraphics>,
     ReadStorage<'a, tiled_map::Map>,
     ReadStorage<'a, Renderable>,
     ReadStorage<'a, Camera>);


    fn run(&mut self, data: Self::SystemData) {

        let (resource, mut gl, map, renderable, camera) = data;

        for camera in (&camera).join() {
            for map_render in (&map, &renderable).join() {
                use graphics::*;
                use graphics::image::draw_many;

                let ref map = map_render.0;
                let ref texture = map_render.1.texture;

                let (sprite_w, sprite_h) = (map.tile_size.0 as u32, map.tile_size.1 as u32);

                let (tex_w, tex_h) = texture.get_size();

                let grid = TilesGridAccessor {
                    tiles_size: (sprite_w as u32, sprite_h as u32),
                    surface_size: (tex_w, tex_h),
                };

                match resource.args {
                    Some(args) => {

                        let (scale_w, scale_h) =
                            ((args.width as f64 / (sprite_w as f64 * camera_viewport_width(camera)) ),
                             (args.height as f64 / (sprite_h as f64 * camera_viewport_height(camera))) );

                        let hack = texture.clone();

                        gl.draw(args.viewport(), |c, g| {

                            clear([255.0, 255.0, 255.0, 0.0], g);
                            let mut glyph_rectangles: Vec<([f64; 4],
                                                           [f64; 4])> = vec![];


                            render_map(&mut glyph_rectangles, map, camera, &grid);

                            draw_many(&glyph_rectangles,
                                      [1.0, 1.0, 1.0, 1.0],
                                      hack.as_ref(),
                                      &DrawState::default(),
                                      c.transform.scale(scale_w as f64, scale_h as f64),
                                      g);
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}



pub fn render_map(glyph_rectangles: &mut Vec<([f64; 4], [f64; 4])>,
                  map: &tiled_map::Map,
                  camera: &Camera,
                  grid_accessor: &TilesGridAccessor) {
    // use tmx::*;

    // println!("{:?}", map.tmx.height());
    // println!("{:?}", map.tmx.width());

    for l in map.tmx.layers() {
        match l.data() {
            Some(tiles) => {

                for ti in tiles.tiles().enumerate() {

                    if ti.1.gid == 0 {
                        continue;
                    };

                    let vec = grid_accessor.access_sprite((ti.1.gid) as u32);

                    let position = [((ti.0 as u32) % map.dimension.0) as f64,
                                    ((ti.0 as u32) / map.dimension.1) as f64];

                    if camera_collide(camera, &[position[0], position[1], 1f64, 1f64]) {

                        let screen_position = position_to_screen_position(camera,
                                                                          &position as &[f64; 2]);

                        let pixel_position = [(map.tile_size.0 as f64 * screen_position[0]),
                                              map.tile_size.0 as f64 * screen_position[1]];

                        glyph_rectangles.push(([pixel_position[0],
                                                pixel_position[1],
                                                map.tile_size.0 as f64,
                                                map.tile_size.0 as f64],
                                               vec));
                    }
                }
                // If there is multiple layers
                // break;
            }
            _ => {}
        }
    }
}
