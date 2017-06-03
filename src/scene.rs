extern crate sdl2;
extern crate specs;

extern crate tmx;

use engine::Drawable;
use map::*;

pub struct Scene {
    pub map: Map
}

pub struct SceneBuilder {
}

impl Drawable for Scene {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.map.draw(canvas);
    }
}

impl SceneBuilder {

    pub fn new() -> SceneBuilder {
        SceneBuilder{
        }
    }

    pub fn load(self) ->  Scene {

        let map = Map::new();

        Scene{
            map: map
        }
    }

}
