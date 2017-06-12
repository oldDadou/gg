extern crate specs;
extern crate find_folder;
extern crate opengl_graphics;
extern crate image;
extern crate tmx;

use specs::*;
use opengl_graphics::*;

use std::sync::Arc;

pub struct Renderable {
    pub texture: Arc<Texture>
}

impl Component for Renderable {
    type Storage = HashMapStorage<Renderable>;
}

pub struct RenderableBuilder {
    texture: Option<Arc<Texture>>,
}

impl RenderableBuilder {

    pub fn new() -> RenderableBuilder {
        RenderableBuilder {
            texture: None
        }
    }

    pub fn texture(mut self, texture: Arc<Texture>) -> RenderableBuilder {
        self.texture = Some(texture);
        self
    }

    pub fn build(self) -> Renderable {

        Renderable{
            texture: self.texture.unwrap()
        }

    }
}
