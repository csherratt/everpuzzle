use amethyst::input::*;
use amethyst::prelude::*;
use amethyst::renderer::*;
use amethyst::core::cgmath::{Matrix4, Vector3};
use rand::prelude::*;
use amethyst::core::{Transform, GlobalTransform};
use amethyst::ecs::*;

use basics::block::Block;
use basics::spritesheet_loader::load_sprite_sheet;
use basics::rng_resource::RngResource;
use data::block_data::BLOCKS;
use data::helpers::i2tuple;

pub struct GameMode {
    rng_seed: [u8; 16],
}

impl GameMode {
    pub fn new(rng_seed: [u8; 16]) -> GameMode {
        GameMode {
            rng_seed
        }
    }

    pub fn create_blocks(world: &mut World, kinds: Vec<i32>) {
        world.register::<Block>();

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            let (x, y): (f32, f32) = i2tuple(i);
            let scale_amount = Vector3::new(2.0, 2.0, 2.0);
            trans.translation = Vector3::new(
                (x * 16.0) * scale_amount.x,
                (y * 16.0) * scale_amount.y,
                0.0
            );
            trans.scale = scale_amount;

            let sprite_render_block = SpriteRender {
                sprite_sheet: load_sprite_sheet(world),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            world.create_entity()
                .with(sprite_render_block)
                .with(Block::new(0, kinds[i]))
                .with(GlobalTransform::default())
                .with(trans)
                .build();
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GameMode {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let mut rng = SmallRng::from_seed(self.rng_seed);

        let mut block_kinds = Vec::new();
        for i in 0..BLOCKS {
            block_kinds.push(rng.gen_range(0, 6));
        }

        GameMode::create_blocks(world, block_kinds);
        world.add_resource::<RngResource>(RngResource { rng });

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
