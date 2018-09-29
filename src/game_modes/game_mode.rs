use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Event, VirtualKeyCode, Camera, Projection,
};
use amethyst::core::cgmath::{Matrix4, Vector3};
use rand::prelude::*;
use amethyst::core::{GlobalTransform};
use amethyst::ecs::*;

use basics::playfield::Playfield;
use basics::block::Block;

pub struct GameMode {
    playfield: Option<Entity>,
    rng_seed: [u8; 16],
}

impl GameMode {
    pub fn new(some_seed: [u8; 16]) -> GameMode {
        GameMode {
            playfield: None,
            rng_seed: some_seed,
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GameMode {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        world.register::<Playfield>();

        let mut p = Playfield {
            stack: Vec::new(),
            rng: SmallRng::from_seed(self.rng_seed)
        };
        p.create(world);

        self.playfield = Some(world.create_entity()
            .with(p)
            .with(GlobalTransform::default())
            .build());

        initialise_camera(world);
    }    

    fn handle_event(&mut self, _data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            500.0,
            500.0,
            0.0
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}
