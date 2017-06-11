extern crate tmx;
extern crate find_folder;


#[derive(Debug)]
pub struct Map {
    pub tmx: tmx::Map,
    pub dimension: (u32, u32),
    pub tile_size: (u32, u32),
}

impl Map {
    pub fn new() -> Map {

        let mut assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        assets.push("32.tmx");

        let tmx_map = match tmx::Map::open(assets.as_path()) {
            Ok(map) => map,
            Err(e) => panic!("Got an error: {}", e),
        };

        for layer in tmx_map.layers() {
            println!("{:?}", layer.name());
        }

        let dimension = (tmx_map.width(), tmx_map.height());
        let tile_size = (tmx_map.tile_width(), tmx_map.tile_height());

        Map {
            tmx: tmx_map,
            dimension: dimension,
            tile_size: tile_size,
        }
    }
}
