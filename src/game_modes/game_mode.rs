use amethyst::{
    input::*,
    prelude::*,
    renderer::*,
    core::{Transform, GlobalTransform, cgmath::Vector3},
    ecs::*
};
use rand::prelude::*;

use basics::{
    block::Block,
    cursor::Cursor,
    spritesheet_loader::{
        load_blocks_sprite_sheet,
        load_sprite_sheet
    },
    rng_resource::RngResource,
};

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

    // creates all entities with block components attached, spritesheet data with sprite_number
    pub fn create_blocks(world: &mut World, kinds: Vec<Option<i32>>) {
        world.register::<Block>();

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            let (x, y): (f32, f32) = i2tuple(i);
            trans.scale = Vector3::new(2.0, 2.0, 2.0);

            let sprite_render_block = SpriteRender {
                sprite_sheet: load_blocks_sprite_sheet(world),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            world.create_entity()
                .with(sprite_render_block)
                .with(Block::new(i as i32, kinds[i], x, y))
                .with(GlobalTransform::default())
                .with(trans)
                .build();
        }
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for GameMode {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // create random generator for random seeded numbers
        let mut rng = SmallRng::from_seed(self.rng_seed);

        // create an array of numbers that all block kinds will have
        let mut block_kinds = Vec::new();
        for i in 0..BLOCKS {
            let num = rng.gen_range(0, 7);

            if num == 6 {
               block_kinds.push(None);
            }
            else {
                block_kinds.push(Some(num));
            }
        }

        GameMode::create_blocks(world, block_kinds);
        // add the random number generator as a global resource to be used
        world.add_resource::<RngResource>(RngResource { rng });

        // load the cursor sprite and attach its data component
        let sprite_sheet = SpriteRender {
            sprite_sheet: load_sprite_sheet(
                world,
                "cursor.png",
                (576.0, 40.0),
                (72.0, 40.0),
                8,
                [-32.0, -16.0]),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };
        world.register::<Cursor>();
        world.create_entity()
            .with(sprite_sheet)
            .with(Transparent::default())
            .with(Cursor::new((0.0, 0.0)))
            .with(GlobalTransform::default())
            .with(Transform::default())
            .build();

        initialise_camera(world);
    }
}

// create a camera that should have the same dimensions as the
// display_config.ron. TODO: use the dimensions
fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.z = 1.0;

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            192.0,
            384.0,
            0.0
        )))
        .with(transform)
        .build();
}
