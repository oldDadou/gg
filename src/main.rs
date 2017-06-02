extern crate sdl2;
extern crate specs;

#[macro_use]
#[allow(unused_imports)]
extern crate tmx;

mod engine;
mod scene;

use engine::*;

#[cfg(not(feature="parallel"))]
fn main() {
}

#[cfg(feature="parallel")]
#[link(name = "SDL2")]
pub fn main() {
    let mut engine = EngineBuilder::new().build();
    engine.run();
}
