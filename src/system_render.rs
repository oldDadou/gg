extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

use camera::*;
use system_render::image::Pixel;
use specs::prelude::*;
use piston::input::*;
use tiled_map;
use std::rc::Rc;
use graphics::color::gamma_srgb_to_linear;
use opengl_graphics::*;


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
    texture_settings.set_mag(Filter::Nearest);
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

struct TilesGridAccessor {
    tiles_size: (u32, u32),
    surface_size: (u32, u32),
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


pub struct RenderSystem<'a> {
    pub gl: &'a mut opengl_graphics::GlGraphics,
    pub texture: Rc<opengl_graphics::Texture>,
}

impl<'a> RenderSystem<'a> {
    pub fn new<'b>(gl: &'b mut GlGraphics) -> RenderSystem<'b> {
        let img = convert_image_from_srgb_to_linear(load_image("Overworld.png"));
        let texture = Rc::new(create_texture_from_image(img));

        RenderSystem {
            gl: gl,
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

impl<'a, 'b> System<'a> for RenderSystem<'b> {
    type SystemData = (Fetch<'a, RenderArgsResource>,
     ReadStorage<'a, tiled_map::Map>,
     ReadStorage<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {

        let (resource, map, camera) = data;
        for camera in (&camera).join() {
            use graphics::*;
            use graphics::image::draw_many;


            let (map_w, map_h) = (30, 30);

            let (sprite_w, sprite_h) = (16 as u32, 16 as u32);
            let (map_pixel_w, map_pixel_h) = (map_w * sprite_w, map_h * sprite_h);

            let (tex_w, tex_h) = self.texture.get_size();

            let grid = TilesGridAccessor {
                tiles_size: (sprite_w as u32, sprite_h as u32),
                surface_size: (tex_w, tex_h),
            };

            match resource.args {
                Some(args) => {

                    let (scale_w, scale_h) = ((args.width as f64 / (map_pixel_w as f64)),
                                              (args.height as f64 / map_pixel_h as f64));

                    let hack = self.texture.clone();

                    self.gl
                        .draw(args.viewport(), |c, g| {

                            clear([255.0, 0.0, 0.0, 0.0], g);
                            let mut glyph_rectangles: Vec<([f64; 4],
                                                           [f64; 4])> = vec![];

                            for b in (&map).join() {

                                let map: &tiled_map::Map = b;
                                for l in map.tmx.layers().enumerate() {
                                    for ti in l.1.data().unwrap().tiles().enumerate() {

                                        let vec = grid.access_sprite((ti.1.gid) as u32);

                                        let position = [((ti.0 as u32) % map_w) as f64,
                                                        ((ti.0 as u32) / map_h) as f64];

                                        if camera.collide(&[position[0], position[1], 1f64, 1f64]) {

                                            let pixel_position = [sprite_w as f64 * position[0],
                                                                  sprite_h as f64 * position[1]];

                                            glyph_rectangles.push(([pixel_position[0],
                                                                    pixel_position[1],
                                                                    sprite_w as f64,
                                                                    sprite_h as f64],
                                                                   vec));
                                        }

                                    }
                                }
                            }

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
