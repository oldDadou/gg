extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

use piston::input::*;
use resources::*;
use specs::*;

pub struct InputSystem {}

impl GameInputResources {
    pub fn new() -> GameInputResources {
        GameInputResources {
            key_down: vec![],
            active_keys: vec![],
            key_release: vec![],
            cursor_position: (-1f64, -1f64),
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (FetchMut<'a, PressButtonResource>,
     FetchMut<'a, InputArgsResources>,
     FetchMut<'a, ReleaseButtonResource>,
     FetchMut<'a, GameInputResources>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut press_input, input, mut release_input, mut game_inputs) = data;

        game_inputs.key_down.clear();
        game_inputs.key_release.clear();

        match &input.args {
            &Some(Input::Move(motion)) => {
                match motion {
                    Motion::MouseCursor(x, y) => {
                        game_inputs.cursor_position = (x, y);
                    }
                    _ => panic!("Unsupported mouse Move"),
                }
            }
            &Some(_) => {},
            &None => {}
        }
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
