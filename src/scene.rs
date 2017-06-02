extern crate sdl2;
extern crate specs;

extern crate tmx;

pub struct Scene {
    pub map: tmx::Map
}

pub struct SceneBuilder {
}

impl SceneBuilder {

    pub fn new() -> SceneBuilder {
        SceneBuilder{
        }
    }

    pub fn load(self) ->  Scene {

        let map = match tmx::Map::open("assets/sans-titre2.tmx") {
            Ok(map) => map,
            Err(e) => panic!("Got an error: {}", e)
        };

        Scene{
            map: map
        }
    }

}
