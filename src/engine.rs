extern crate sdl2;
extern crate specs;

extern crate tmx;

use std::path::Path;

use scene::*;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::surface::*;
use sdl2::image::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


struct TilesGridAccessor{
    tiles_size: (u32, u32),
    surface_size: (u32, u32)
}

impl TilesGridAccessor {
    pub fn access_sprite(&self, idx: u32) -> Rect  {
        let x = (self.tiles_size.0 ) * ((idx - 1) % (self.surface_size.0 / self.tiles_size.0));
        let y = self.tiles_size.1 * (idx / (self.surface_size.0 / self.tiles_size.0));
        Rect::new(x as i32 , y as i32, self.tiles_size.0 as u32, self.tiles_size.1  as u32)
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
}

impl Drawable for tmx::Map {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {

       let map_loader = sdl2::rwops::RWops::from_file(Path::new("assets/Overworld.png"), "r").unwrap();
       let texture_creator = canvas.texture_creator();
       let surface: Surface = map_loader.load_png().unwrap();
       let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

       let grid = TilesGridAccessor {
           tiles_size: (16, 16),
           surface_size: (surface.width(), surface.height())
       };

       for l in self.layers() {
           for ti in l.data().unwrap().tiles().enumerate() {
               let dest_rect  = Rect::new(32 * (ti.0 % 10) as i32, 32 * (ti.0 / 10) as i32, 32, 32);
               canvas.copy_ex(&texture, grid.access_sprite((ti.1.gid) as u32), dest_rect, 0.0, None, false, false).unwrap();
           }
       }
    }
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

        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
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

    pub fn draw_scene(&mut self, scene: &Scene) {
        scene.map.draw(&mut self.canvas);
    }

    pub fn update_scene(&self, scene: &Scene) {

    }

    pub fn run(&mut self) {

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut scene = SceneBuilder::new()
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

            self.canvas.clear();
            self.draw_scene(&scene);
            self.canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
