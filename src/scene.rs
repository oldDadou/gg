extern crate opengl_graphics;

use tiled_map::*;
use specs::*;
use camera::*;
use renderable::*;

use system_render::*;
use system_camera::*;
use system_input::*;


pub struct Scene<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Scene<'a, 'b> {

    pub fn mut_world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn update(&mut self) {
        self.dispatcher.dispatch(&mut self.world.res);
    }
}

pub struct SceneBuilder {
    map: Option<Map>,
    gl: Option<opengl_graphics::GlGraphics>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            map: None,
            gl: None,
        }
    }

    pub fn graphics(mut self, gl: opengl_graphics::GlGraphics) -> SceneBuilder {
        self.gl = Some(gl);
        self
    }

    pub fn map(mut self, map_name: &String) -> SceneBuilder {
        let mut builder = MapBuilder::new();
        builder = builder.name(map_name);
        self.map = Some(builder.build());
        self
    }

    pub fn build<'b, 'a>(self) -> Scene<'b, 'a> {

        let mut world = World::new();

        let camera = Camera {
            position: [5f64, 5f64],
            area: [8f64, 8f64],
        };


        let sys: RenderSystem = RenderSystem::new();
        let camesys: CameraSystem = CameraSystem::new();

        let map = self.map.unwrap();

        world.register::<Map>();
        world.register::<Camera>();
        world.register::<Renderable>();

        world
            .create_entity()
            .with(RenderableBuilder::new().name(&map.tileset_file).build())
            .with(map)
            .build();


        world.create_entity().with(camera).build();

        world.add_resource(self.gl.unwrap());

        // Let's use some start value
        world.add_resource(RenderArgsResource { args: None });

        // Init input resources
        world.add_resource(PressButtonResource { inputs: vec![] });
        world.add_resource(ReleaseButtonResource { inputs: vec![] });
        world.add_resource(GameInputResources::new());

        let dispatcher = DispatcherBuilder::new()
            .add_thread_local(InputSystem {})
            .add_thread_local(camesys)
            .add_thread_local(sys)
            .build();

        Scene {
            dispatcher: dispatcher,
            world: world,
        }
    }
}
