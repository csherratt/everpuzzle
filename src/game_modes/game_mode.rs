use amethyst::{
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
        load_spritesheet
    },
    rng_resource::RngResource,
};

use data::block_data::{BLOCKS, COLS};
use data::helpers::i2tuple;
use std::boxed::Box;

pub struct GameMode {
    rng_seed: [u8; 16],
    config: DisplayConfig
}

impl GameMode {
    pub fn new(rng_seed: [u8; 16], config: DisplayConfig) -> GameMode {
        GameMode {
            rng_seed,
            config
        }
    }

    // creates all entities with block components attached, spritesheet data with sprite_number
    pub fn create_blocks(world: &mut World, kinds: Vec<i32>) {
        world.register::<Block>();
        let mut entities: Vec<Entity> = Vec::new();

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            trans.scale = Vector3::new(4.0, 4.0, 4.0);

            // set position instantly so no weird spawn flash happens
            let mut b = Block::new(i as u32, kinds[i], i2tuple(i));
            b.set_position(&mut trans);

            let sprite_render_block = SpriteRender {
                sprite_sheet: load_blocks_sprite_sheet(world),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            entities.push(world.create_entity()
                .with(sprite_render_block)
                .with(b)
                .with(GlobalTransform::default())
                .with(trans)
                .build());
        }
    }

    // create a camera that should have the same dimensions as the
    // display_config.ron. TODO: use the dimensions
    fn initialise_camera(&mut self, world: &mut World) {
        let mut transform = Transform::default();
        transform.translation.z = 1.0;

        world.create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,
                self.config.dimensions.unwrap().0 as f32,
                self.config.dimensions.unwrap().1 as f32,
                0.0
            )))
            .with(transform)
            .build();
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for GameMode {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // create random generator for random seeded numbers
        let mut rng = SmallRng::from_seed(self.rng_seed);

        // create an array of numbers that all block kinds will have
        let mut block_kinds = Vec::new();
        for _i in 0..BLOCKS {
            block_kinds.push(rng.gen_range(0, 7) - 1);
        }

        GameMode::create_blocks(world, block_kinds);
        // add the random number generator as a global resource to be used
        world.add_resource::<RngResource>(RngResource { rng });

        // load the cursor sprite and attach its data component
        let sprite_sheet = SpriteRender {
            sprite_sheet: load_spritesheet(
                world,
                "cursor.png",
                "cursor_spritesheet.ron"
            ),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        let mut trans = Transform::default();
        trans.scale = Vector3::new(2.0, 2.0, 2.0);

        let mut cursor = Cursor::new((2.0, 5.0));
        cursor.set_position(&mut trans);

        world.register::<Cursor>();
        world.create_entity()
            .with(sprite_sheet)
            .with(Transparent::default())
            .with(cursor)
            .with(GlobalTransform::default())
            .with(trans)
            .build();

        self.initialise_camera(world);
    }
}

