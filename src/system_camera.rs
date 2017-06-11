extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

use piston::input::*;
use specs::prelude::*;
use camera::*;
use system_input::*;
use tiled_map;

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> CameraSystem {
        CameraSystem {}
    }
}

#[inline]
fn get_edge_position(camera: &Camera) -> [f64; 4] {
    [camera.position[0] - camera.area[0] / 2f64,
     camera.position[1] - camera.area[1] / 2f64,
     camera.position[0] + camera.area[0] / 2f64,
     camera.position[1] + camera.area[1] / 2f64]
}

#[inline]
fn smooth_camera_position(camera: &mut Camera) {

    let edge = get_edge_position(camera);
    if edge[2] > 30f64 {
        camera.position[0] = 30f64 - (camera.area[0] / 2f64);
    }
    if edge[0] < 0f64 {
        camera.position[0] = camera.area[0] / 2f64;
    }

    if edge[1] < 0f64 {
        camera.position[1] = camera.area[1] / 2f64;
    }
    if edge[3] > 30f64 {
        camera.position[1] = 30f64 - (camera.area[1] / 2f64);
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (Fetch<'a, GameInputResources>,
     ReadStorage<'a, tiled_map::Map>,
     WriteStorage<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (game_inputs, game_map, mut camera_comps) = data;

        let (zoom_max, zoom_min) = (6.0, 30.0);

        for _ in (&game_map).join() {
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

                    if *button == Button::Keyboard(Key::NumPadPlus) {
                        if (camera.area[0] - 0.5f64) > zoom_max {
                            camera.area[0] -= 0.5f64;
                            camera.area[1] -= 0.5f64;
                        } else {
                            camera.area[0] = zoom_max;
                            camera.area[1] = zoom_max;
                        }
                    }
                    if *button == Button::Keyboard(Key::NumPadMinus) {
                        if (camera.area[0] + 0.5f64) < zoom_min {
                            camera.area[0] += 0.5f64;
                            camera.area[1] += 0.5f64;
                        } else {
                            camera.area[0] = zoom_min;
                            camera.area[1] = zoom_min;
                        }
                    }

                }
                smooth_camera_position(camera)
            }
        }
    }
}
