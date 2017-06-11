extern crate specs;
use specs::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub position: [f64; 2],
    pub area: [f64; 2],
}

impl Component for Camera {
    type Storage = HashMapStorage<Camera>;
}

impl Camera {
    #[inline]
    pub fn collide(&self, rect1: &[f64; 4]) -> bool {
        let self_rect = [self.position[0] - self.area[0] / 2f64,
                         self.position[1] - self.area[1] / 2f64,
                         self.position[0] + self.area[0] / 2f64,
                         self.position[1] + self.area[1] / 2f64];

        ((rect1[0] < (self_rect[2])) & ((rect1[0] + rect1[2]) > self_rect[0]) &
         (rect1[1] < (self_rect[3])) & ((rect1[1] + rect1[3]) > self_rect[1]))

    }
}
