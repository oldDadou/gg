extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

use piston::input::*;
use specs::prelude::*;

pub struct InputSystem {}

pub struct PressButtonResource {
    pub inputs: Vec<Button>,
}

pub struct ReleaseButtonResource {
    pub inputs: Vec<Button>,
}

#[derive(Debug)]
pub struct GameInputResources {
    pub key_down: Vec<Button>,
    pub active_keys: Vec<Button>,
    pub key_release: Vec<Button>,
}

impl GameInputResources {
    pub fn new() -> GameInputResources {
        GameInputResources {
            key_down: vec![],
            active_keys: vec![],
            key_release: vec![],
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (FetchMut<'a, PressButtonResource>,
     FetchMut<'a, ReleaseButtonResource>,
     FetchMut<'a, GameInputResources>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut press_input, mut release_input, mut game_inputs) = data;

        // println!("down {:?}", game_inputs.key_down);
        // println!("active {:?}", game_inputs.active_keys);
        // println!("release {:?}", game_inputs.key_release);

        game_inputs.key_down.clear();
        game_inputs.key_release.clear();

        while let &Some(button) = &press_input.inputs.pop() {
            game_inputs.key_down.push(button);
            game_inputs.active_keys.push(button);
        }

        while let &Some(button) = &release_input.inputs.pop() {
            game_inputs.key_release.push(button);
            game_inputs.active_keys.retain(|&input| input != button);
        }

    }
}
