extern crate sdl2;
extern crate specs;

extern crate tmx;

use scene::*;

use sdl2::pixels::*;
use sdl2::event::*;
use sdl2::keyboard::*;
use std::time::*;

pub trait Drawable {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
}


pub struct Engine {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

pub struct EngineBuilder;
impl EngineBuilder {

    pub fn new() -> EngineBuilder {
        EngineBuilder{}
    }

    pub fn build(self) -> Engine {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 160*2, 160*2)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();

        Engine{
            sdl_context: sdl_context,
            canvas: canvas,
        }
    }
}


impl<'a> Engine {

    #[inline]
    pub fn draw_scene(&mut self, scene: &Scene) {
        self.canvas.clear();
        scene.map.draw(&mut self.canvas);
        self.canvas.present();
    }

    pub fn update_scene(&self, _: &Scene) {

    }

    pub fn run(&mut self) {

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let scene = SceneBuilder::new()
        .load();

        let mut running = true;
        'running: while running {

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        running = false;
                    },
                    _ => {}
                }
            }

            self.update_scene(&scene);
            self.draw_scene(&scene);

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
