extern crate opengl_graphics;

use tiled_map::*;
use specs::prelude::*;
use camera::*;

use system_render::*;
use system_camera::*;
use system_input::*;

pub struct Scene<'a> {
    world: World,
    dispatcher: Dispatcher<'a>,
}

impl<'a> Scene<'a> {

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

    pub fn build<'b>(self) -> Scene<'b> {
        let mut world = World::new();

        let camera = Camera {
            position: [5f64, 5f64],
            area: [10f64, 10f64],
        };


        let sys: RenderSystem = RenderSystem::new();
        let camesys: CameraSystem = CameraSystem::new();

        world.register::<Map>();
        world.register::<Camera>();

        world
            .create_entity()
            .with(MapBuilder::new().name(&String::from("32.tmx")).build())
            .build();
        world.create_entity().with(camera).build();

        world.add_resource(self.gl.unwrap());

        // Let's use some start value
        world.add_resource(RenderArgsResource { args: None });

        // Init input resources
        world.add_resource(PressButtonResource { inputs: vec![] });
        world.add_resource(ReleaseButtonResource { inputs: vec![] });
        world.add_resource(GameInputResources::new());
        // world.add_resource(self.gl);

        let mut dispatcher = DispatcherBuilder::new()
            .add_thread_local(InputSystem {})
            .add_thread_local(camesys)
            .add_thread_local(sys)
            .build();

        Scene {
            dispatcher: dispatcher,
            world: world,
        }
    }

    pub fn map(mut self, map_name: &String) -> SceneBuilder {
        let mut builder = MapBuilder::new();
        builder = builder.name(map_name);
        self.map = Some(builder.build());
        self
    }
}
