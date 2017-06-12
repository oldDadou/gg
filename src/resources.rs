extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;
extern crate tmx;

use piston::input::*;
pub struct RenderArgsResource {
    pub args: Option<RenderArgs>,
}

pub struct ResizeArgsResource {
    pub args: Option<(u32, u32)>,
}
