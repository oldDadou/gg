extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

use piston::input::*;
use specs::prelude::*;
use camera::*;
use system_input::*;

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> CameraSystem {
        CameraSystem {}
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (Fetch<'a, GameInputResources>, WriteStorage<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (game_inputs, mut camera_comps) = data;

        for camera in (&mut camera_comps).join() {
            for button in &game_inputs.active_keys {
                if *button == Button::Keyboard(Key::Left) {
                    camera.position[0] -= 0.05;
                }
                if *button == Button::Keyboard(Key::Right) {
                    camera.position[0] += 0.05;
                }
                if *button == Button::Keyboard(Key::Up) {
                    camera.position[1] -= 0.05;
                }
                if *button == Button::Keyboard(Key::Down) {
                    camera.position[1] += 0.05;
                }
            }
        }
    }
}
