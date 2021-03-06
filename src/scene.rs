extern crate opengl_graphics;

use tiled_map::*;
use specs::*;
use camera::*;
use renderable::*;

use assets_manager::*;

use system_map_render::*;
use system_debug_render::*;
use system_camera::*;
use system_input::*;

pub struct Scene<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Scene<'a, 'b> {

    pub fn update(&mut self, world: &mut World) {
        self.dispatcher.dispatch(&mut world.res);
    }
}

pub struct SceneBuilder<'a> {
    map: Option<Map>,
    gl: Option<opengl_graphics::GlGraphics>,
    assets: Option<&'a mut AssetsManager>,
}

impl<'a> SceneBuilder<'a> {
    pub fn new() -> SceneBuilder<'a> {
        SceneBuilder {
            map: None,
            gl: None,
            assets: None
        }
    }

    pub fn manager(mut self, manager: &'a mut AssetsManager) -> SceneBuilder<'a> {
        self.assets = Some(manager);
        self
    }

    pub fn graphics(mut self, gl: opengl_graphics::GlGraphics) -> SceneBuilder<'a> {
        self.gl = Some(gl);
        self
    }

    pub fn map(mut self, map_name: &String) -> SceneBuilder<'a> {
        let mut builder = MapBuilder::new();
        builder = builder.name(map_name);
        self.map = Some(builder.build());
        self
    }

    pub fn build<'b, 'd>(self, world: &mut World) -> Scene<'b, 'd> {

        let camera = Camera::new();

        let sys: RenderMapSystem = RenderMapSystem::new();
        let debug: RenderDebugSystem = RenderDebugSystem::new();
        let camesys: CameraSystem = CameraSystem::new();

        let map = self.map.unwrap();

        world
            .create_entity()
            .with(RenderableBuilder::new()
                      .texture(self.assets.unwrap().get_texture(&map.tileset_file))
                      .build())
            .with(map)
            .build();


        world.create_entity().with(camera).build();
        world.add_resource(self.gl.unwrap());

        let dispatcher = DispatcherBuilder::new()
            .add_thread_local(InputSystem {})
            .add_thread_local(camesys)
            .add_thread_local(sys)
            .add_thread_local(debug)
            .build();

        Scene {
            dispatcher: dispatcher,
        }
    }
}
