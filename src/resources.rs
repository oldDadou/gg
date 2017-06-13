extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

use piston::input::*;

pub struct InputArgsResources {
    pub args: Option<Input>
}

pub struct RenderArgsResource {
    pub args: Option<RenderArgs>,
}

pub struct ResizeArgsResource {
    pub args: Option<(u32, u32)>,
}


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
    pub cursor_position: (f64, f64)
}
