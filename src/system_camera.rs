extern crate find_folder;
extern crate opengl_graphics;
extern crate specs;
extern crate image;

use piston::input::*;
use specs::*;
use camera::*;
use system_input::*;
use tiled_map;
use resources::*;

pub struct CameraSystem {}

impl CameraSystem {
    pub fn new() -> CameraSystem {
        CameraSystem {}
    }
}

#[inline]
fn smooth_camera_position(camera: &mut Camera) {
    let edge = get_edge_position(camera);

    if edge[2] > 30f64 {
        camera.position[0] = 30f64 - camera_viewport_width(camera) / 2f64;
    }
    if edge[0] < 0f64 {
        camera.position[0] = camera_viewport_width(camera) / 2f64;
    }

    if edge[1] < 0f64 {
        camera.position[1] = camera_viewport_height(camera) / 2f64;
    }
    if edge[3] > 30f64 {
        camera.position[1] = 30f64 - (camera_viewport_height(camera) / 2f64);
    }

}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Fetch<'a, ResizeArgsResource>,
        Fetch<'a, GameInputResources>,
        ReadStorage<'a, tiled_map::Map>,
        WriteStorage<'a, Camera>);

        fn run(&mut self, data: Self::SystemData) {

            let (resizeargs, game_inputs, game_map, mut camera_comps) = data;

            for map in (&game_map).join() {

                for camera in (&mut camera_comps).join() {

                    // If the windows has been resized, we scale the camera to keep
                    // the aspect ratio
                    match resizeargs.args {
                        Some(args) => {
                            scale_to_resolution(camera, args.0, args.1);
                        },
                        _ => {}
                    }

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

                    for button in &game_inputs.key_down {

                        if *button == Button::Keyboard(Key::NumPadPlus) {
                            camera.zoom += 0.1;
                            if camera_viewport_width(camera) <= 5.0f64 || camera_viewport_height(camera) <= 5.0f64 {
                                camera.zoom -= 0.1;
                            }
                        }

                        if *button == Button::Keyboard(Key::NumPadMinus) {
                            camera.zoom -= 0.1;

                            if (camera_viewport_width(camera) > map.dimension.0 as f64) || ( camera_viewport_height(camera) > map.dimension.1 as f64) {
                                camera.zoom = zoom_to_fill(camera, 30f64);
                            }

                        }
                    }
                    smooth_camera_position(camera)
                }
            }
        }
    }
