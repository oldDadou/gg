extern crate tmx;
extern crate find_folder;

#[derive(Debug)]
pub struct Map {
    pub tmx: tmx::Map,
}

impl Map {
    pub fn new() -> Map {

        let mut assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        assets.push("large.tmx");

        let tmx_map = match tmx::Map::open(assets.as_path()) {
            Ok(map) => map,
            Err(e) => panic!("Got an error: {}", e),
        };

        Map { tmx: tmx_map }
    }
}
