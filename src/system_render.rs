extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

use camera::*;
use system_render::image::Pixel;
use specs::prelude::*;
use piston::input::*;
use tiled_map;
use std::rc::Rc;
use graphics::color::gamma_srgb_to_linear;
use opengl_graphics::*;

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

fn load_image(path: &str) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let resources = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let sprite = resources.join(path);
    let img = image::open(&sprite).unwrap();
    let img = match img {
        image::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba(),
    };
    img
}

fn create_texture_from_image(img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>)
                             -> opengl_graphics::Texture {
    let mut texture_settings = opengl_graphics::TextureSettings::new();
    texture_settings.set_convert_gamma(true);
    texture_settings.set_compress(true);
    texture_settings.set_filter(Filter::Nearest);
    texture_settings.set_mag(Filter::Nearest);
    // texture_settings.set_mipmap(Filter::Linear);
    opengl_graphics::Texture::from_image(&img, &texture_settings)
}

fn convert_image_from_srgb_to_linear(img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>)
                                     -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let mut new_img = img.clone();

    for (x, y, pixel) in img.enumerate_pixels() {
        let (r, g, b, a) = pixel.channels4();
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let a = a as f32 / 255.0;
        let new_color = gamma_srgb_to_linear([r, g, b, a]);
        let r = (new_color[0] * 255.0) as u8;
        let g = (new_color[1] * 255.0) as u8;
        let b = (new_color[2] * 255.0) as u8;
        let a = (new_color[3] * 255.0) as u8;
        let new_pixel = image::Pixel::from_channels(r, g, b, a);
        new_img.put_pixel(x, y, new_pixel);
    }

    new_img
}


#[inline]
fn position_to_screen_position(camera: &Camera, rect: &[f64; 2]) -> [f64; 2] {
    let cam_rect = [camera.position[0] - camera.area[0] / 2f64,
                    camera.position[1] - camera.area[1] / 2f64];

    [rect[0] - cam_rect[0], rect[1] - cam_rect[1]]
}

pub struct RenderSystem {
    // pub gl: &'a mut opengl_graphics::GlGraphics,
    pub texture: Rc<opengl_graphics::Texture>,
}

impl RenderSystem {
    pub fn new() -> RenderSystem {
        let img = convert_image_from_srgb_to_linear(load_image("lel.png"));
        let texture = Rc::new(create_texture_from_image(img));

        RenderSystem {
            // gl: gl,
            texture: texture,
        }
    }
}

pub struct RenderArgsResource {
    pub args: Option<RenderArgs>,
}

impl Component for tiled_map::Map {
    type Storage = HashMapStorage<tiled_map::Map>;
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Fetch<'a, RenderArgsResource>,
     FetchMut<'a, opengl_graphics::GlGraphics>,
     ReadStorage<'a, tiled_map::Map>,
     ReadStorage<'a, Camera>);


    fn run(&mut self, data: Self::SystemData) {

        let (resource, mut gl, map, camera) = data;

        for camera in (&camera).join() {
            for map in (&map).join() {
                use graphics::*;
                use graphics::image::draw_many;

                let (sprite_w, sprite_h) = (map.tile_size.0 as u32, map.tile_size.1 as u32);

                let (tex_w, tex_h) = self.texture.get_size();

                let grid = TilesGridAccessor {
                    tiles_size: (sprite_w as u32, sprite_h as u32),
                    surface_size: (tex_w, tex_h),
                };

                match resource.args {
                    Some(args) => {

                        let (scale_w, scale_h) =
                            ((args.width as f64 / (sprite_w as f64 * camera.area[0])),
                             (args.height as f64 / (sprite_h as f64 * camera.area[1])));

                        let hack = self.texture.clone();

                        gl.draw(args.viewport(), |c, g| {

                            clear([255.0, 255.0, 255.0, 0.0], g);
                            let mut glyph_rectangles: Vec<([f64; 4],
                                                           [f64; 4])> = vec![];


                            render_map(&mut glyph_rectangles, map, camera, &grid);

                            draw_many(&glyph_rectangles,
                                      [1.0, 1.0, 1.0, 1.0],
                                      &*hack,
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

                    if camera.collide(&[position[0], position[1], 1f64, 1f64]) {

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
