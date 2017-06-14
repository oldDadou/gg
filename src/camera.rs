extern crate specs;
use specs::*;

#[derive(Debug)]
pub struct Camera {
    pub position: [f64; 2],
    pub area: [f64; 2],
    pub zoom: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new() -> Camera {
        let ratio = 16f64 / 9f64;
        Camera {
            position: [5f64, 5f64],
            area: [ratio * 10f64, 10f64],
            zoom: 1f64,
            aspect_ratio: ratio,
        }
    }
}

impl Component for Camera {
    type Storage = HashMapStorage<Camera>;
}

#[inline]
pub fn scale_to_resolution(camera: &mut Camera, width: u32, height: u32) {
    let aspect_ratio = width as f64 / height as f64;

    camera.aspect_ratio = aspect_ratio;
    camera.area = [aspect_ratio * camera.area[1], camera.area[1]]
}

#[inline]
pub fn camera_viewport_height(camera: &Camera) -> f64 {
    (camera.position[1] + ((camera.area[1] * camera.zoom) / 2f64)) -
    (camera.position[1] - ((camera.area[1] * camera.zoom) / 2f64))
}

#[inline]
pub fn camera_viewport_width(camera: &Camera) -> f64 {
    (camera.position[0] + ((camera.area[0] * camera.zoom) / 2f64)) -
    (camera.position[0] - ((camera.area[0] * camera.zoom) / 2f64))
}

#[inline]
pub fn zoom_to_fill_width(camera: &Camera, size: f64) -> f64 {
    size / camera.area[0]
}
#[inline]
pub fn zoom_to_fill_height(camera: &Camera, size: f64) -> f64 {
    size / camera.area[1]
}

#[inline]
pub fn get_edge_position(camera: &Camera) -> [f64; 4] {
    [camera.position[0] - ((camera.area[0] * camera.zoom) / 2f64),
     camera.position[1] - ((camera.area[1] * camera.zoom) / 2f64),
     camera.position[0] + ((camera.area[0] * camera.zoom) / 2f64),
     camera.position[1] + ((camera.area[1] * camera.zoom) / 2f64)]
}

#[inline]
pub fn camera_collide(camera: &Camera, rect1: &[f64; 4]) -> bool {
    let camera_view_port = get_edge_position(camera);
    ((rect1[0] < (camera_view_port[2])) & ((rect1[0] + rect1[2]) > camera_view_port[0]) &
     (rect1[1] < (camera_view_port[3])) & ((rect1[1] + rect1[3]) > camera_view_port[1]))
}

#[inline]
pub fn position_to_screen_position(camera: &Camera, rect: &[f64; 2]) -> [f64; 2] {
    let came_pos = get_edge_position(camera);
    [rect[0] - came_pos[0], rect[1] - came_pos[1]]
}

#[inline]
pub fn screen_position_to_position(camera: &Camera,
                                   screen_dimension: &[f64; 2],
                                   rect: &[f64; 2])
                                   -> [f64; 2] {
    let came_pos = get_edge_position(camera);

    let viewport_width = camera_viewport_width(camera);
    let viewport_height = camera_viewport_height(camera);

    [(rect[0] / (screen_dimension[0] / viewport_width)) + came_pos[0],
     (rect[1] / (screen_dimension[1] / viewport_height)) + came_pos[1]]
    // [(rect[0]/camera.aspect_ratio) + came_pos[0], (rect[1]/camera.aspect_ratio) + came_pos[1]]
}
