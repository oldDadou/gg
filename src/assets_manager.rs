extern crate specs;
extern crate find_folder;
extern crate opengl_graphics;
extern crate image;
extern crate tmx;

use std::collections::*;
use std::sync::Arc;
use graphics::color::gamma_srgb_to_linear;
use opengl_graphics::*;

use assets_manager::image::Pixel;

pub struct AssetsManager {
    textures: HashMap<String, Arc<opengl_graphics::Texture>>,
}

impl AssetsManager {
    pub fn new() -> AssetsManager {
        AssetsManager { textures: HashMap::new() }
    }

    pub fn load_texture(&mut self, path: &String) {
        let img = convert_image_from_srgb_to_linear(load_image(path));
        // let img = load_image(&self.name.unwrap());
        let texture = create_texture_from_image(img);
        self.textures.insert(path.clone(), Arc::new(texture));
    }

    pub fn get_texture(&mut self, path: &String) -> Arc<opengl_graphics::Texture> {
        if !self.textures.contains_key(path) {
            self.load_texture(path);
        }
        self.textures.get(path).unwrap().clone()
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
