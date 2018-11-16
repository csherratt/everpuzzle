use amethyst::{
    prelude::*,
    renderer::*,
    core::{Transform, GlobalTransform, cgmath::Vector3},
    ecs::*,
    assets::*
};
use rand::prelude::*;

use basics::{
    block::Block,
    cursor::Cursor,
    spritesheet_loader::{
        SpriteSheetLoader,
        load_sprite_sheet
    },
    rng_resource::RngResource,
    kind_generator::KindGenerator
};

use data::block_data::BLOCKS;
use data::helpers::i2tuple;

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

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            trans.scale = Vector3::new(4.0, 4.0, 4.0);

            // set position instantly so no weird spawn flash happens
            let mut b = Block::new(i as u32, kinds[i], i2tuple(i));
            b.set_position(&mut trans);

            let sprite_render_block = SpriteRender {
                sprite_sheet: SpriteSheetLoader::load_blocks_sprite_sheet(world),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            world.create_entity()
                .with(sprite_render_block)
                .with(b)
                .with(GlobalTransform::default())
                .with(trans)
                .build();
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
        let mut kind_gen: KindGenerator = KindGenerator { 
            rng: SmallRng::from_seed(self.rng_seed) 
        };
        let kinds = kind_gen.create_stack(5, 8);

        GameMode::create_blocks(world, kinds);
        // add the random number generator as a global resource to be used
        world.add_resource::<KindGenerator>(kind_gen);

        // load the cursor sprite and attach its data component
        let sprite_sheet = SpriteRender {
            sprite_sheet: load_sprite_sheet(
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

