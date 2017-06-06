extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

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

fn create_texture_from_image(img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> opengl_graphics::Texture {
    let mut texture_settings = opengl_graphics::TextureSettings::new();
    texture_settings.set_convert_gamma(true);
    texture_settings.set_compress(true);
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

struct TilesGridAccessor{
    tiles_size: (u32, u32),
    surface_size: (u32, u32)
}

impl TilesGridAccessor {
    /// Return the rect containing the sprite (ready to draw)
    pub fn access_sprite(&self, idx: u32) -> [f64; 4]  {
        let x = (self.tiles_size.0 ) * ((idx - 1) % (self.surface_size.0 / self.tiles_size.0));
        let y = self.tiles_size.1 * (idx / (self.surface_size.0 / self.tiles_size.0));

        [x as f64 , y as f64, self.tiles_size.0 as f64, self.tiles_size.1  as f64]
    }
}


pub struct RenderSystem<'a> {
    pub gl: &'a mut opengl_graphics::GlGraphics,
    pub texture: Rc<opengl_graphics::Texture>
}

impl<'a> RenderSystem<'a> {

    pub fn new<'b>(gl: &'b mut GlGraphics) -> RenderSystem<'b> {
        let img = convert_image_from_srgb_to_linear(load_image("Overworld.png"));
        let texture = Rc::new(create_texture_from_image(img));

        RenderSystem{
            gl: gl,
            texture: texture
        }
    }

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

        let (sprite_w, sprite_h) = (16 as u32, 16 as u32);
        let (tex_w, tex_h) = self.texture.get_size();

        let grid = TilesGridAccessor {
            tiles_size: (sprite_w as u32, sprite_h as u32),
            surface_size: (tex_w, tex_h)
        };

        // let t = Texture::from_path(&Path::new("./assets/Overworld.png")).unwrap();
        match resource.args {
            Some(args) => {

                let (screen_w, screen_h) = (args.width as f64, args.height as f64);

                let scale_w = screen_w/160f64;
                let scale_h = screen_h/160f64;

                let scale_w = 1.0;
                let scale_h = 1.0;

                println!("{:?} {:?}", args, (scale_w, scale_h));

                let viewport = Viewport{
                    rect: [0, 0, 160, 160],
                    draw_size: [args.width, args.height],
                    window_size: [args.width, args.height]
                };

                let hack = self.texture.clone();

                self.gl.draw(args.viewport(), |c, g| {

                    let _c = Context::new_viewport(viewport);
                    // image(&*hack, c.transform, g);

                    clear([0.0, 0.0, 0.0, 0.0], g);
                    for b in (&map).join() {
                        let map: &tiled_map::Map = b;

                        for l in map.tmx.layers() {
                            for ti in l.data().unwrap().tiles().enumerate() {

                                let position = [ (sprite_w * ((ti.0 as u32)  % 10u32)) as f64,  (sprite_h * ((ti.0 as u32) / 10u32)) as f64 ];
                                let transformed = _c.transform.trans(position[0], position[1]);

                                let ref draw_state: DrawState = Default::default();

                                let vec = grid.access_sprite((ti.1.gid) as u32);

                                Image::new()
                                .rect([0f64, 0f64, 16f64, 16f64])
                                .maybe_src_rect(Some(vec))
                                .draw(&*hack, draw_state, transformed, g);
                            }
                        }
                    }
                });
            },
            _ => {
            }
        }

    }
}
