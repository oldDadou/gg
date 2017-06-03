extern crate tmx;
extern crate sdl2;

use engine::Drawable;

use std::path::Path;

use sdl2::rect::*;
use sdl2::surface::*;
use sdl2::image::*;

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

pub struct Map {
    pub tmx: tmx::Map,
    grid_helper: TilesGridAccessor
}

impl Map {
    pub fn new() -> Map {

        let tmx_map = match tmx::Map::open("assets/sans-titre2.tmx") {
            Ok(map) => map,
            Err(e) => panic!("Got an error: {}", e)
        };

        let grid = TilesGridAccessor {
            tiles_size: (16, 16),
            surface_size: (640, 576)
        };

        Map{
            tmx: tmx_map,
            grid_helper: grid
        }
    }
}

/// Im a not sure if I want this class to be drawable
impl Drawable for Map {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {

       let map_loader = sdl2::rwops::RWops::from_file(Path::new("assets/Overworld.png"), "r").unwrap();
       let texture_creator = canvas.texture_creator();
       let surface: Surface = map_loader.load_png().unwrap();
       let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

       for l in self.tmx.layers() {
           for ti in l.data().unwrap().tiles().enumerate() {
               let dest_rect  = Rect::new(32 * (ti.0 % 10) as i32, 32 * (ti.0 / 10) as i32, 32, 32);
               canvas.copy_ex(&texture, self.grid_helper.access_sprite((ti.1.gid) as u32), dest_rect, 0.0, None, false, false).unwrap();
           }
       }
    }
}
