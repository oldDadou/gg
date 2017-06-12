extern crate specs;
use specs::*;

#[derive(Debug)]
pub struct Camera {
    pub position: [f64; 2],
    pub area: [f64; 2],
    pub zoom: f64,
    pub aspect_ratio: f64
}

impl Camera {
    pub fn new() -> Camera {
        Camera{
            position: [5f64, 5f64],
            area: [(19f64/9f64) * 10f64, 10f64],
            zoom: 1f64,
            aspect_ratio: 1f64
        }
    }
}

impl Component for Camera {
    type Storage = HashMapStorage<Camera>;
}

#[inline]
pub fn scale_to_resolution(camera: &mut Camera, width: u32, height: u32) {
    let aspect_ration = width as f64 / height as f64;
    camera.area = [ aspect_ration * camera.area[1], camera.area[1]]
}

#[inline]
pub fn camera_viewport_width(camera: &Camera) -> f64 {
    (camera.position[0] + (camera.area[0] / (camera.zoom * 2f64))) - (camera.position[0] - (camera.area[0] / (camera.zoom * 2f64)))
}

#[inline]
pub fn zoom_to_fill(camera: &Camera, size: f64) -> f64 {
    camera.area[0] / size
}

#[inline]
pub fn camera_viewport_height(camera: &Camera) -> f64 {
    (camera.position[1] + (camera.area[1] / (camera.zoom * 2f64))) - (camera.position[1] - (camera.area[1] / (camera.zoom * 2f64)))
}

#[inline]
pub fn get_edge_position(camera: &Camera) -> [f64; 4] {
    [
     camera.position[0] - (camera.area[0] / camera.zoom / 2f64),
     camera.position[1] - (camera.area[1] / camera.zoom / 2f64),
     camera.position[0] + (camera.area[0] / camera.zoom / 2f64),
     camera.position[1] + (camera.area[1] / camera.zoom / 2f64)
    ]
}

#[inline]
pub fn camera_collide(camera: &Camera,  rect1: &[f64; 4]) -> bool {
    let camera_view_port = get_edge_position(camera);

    ((rect1[0] < (camera_view_port[2])) & ((rect1[0] + rect1[2]) > camera_view_port[0]) &
    (rect1[1] < (camera_view_port[3])) & ((rect1[1] + rect1[3]) > camera_view_port[1]))
}
