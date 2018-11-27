use amethyst::{
    core::{cgmath::Vector3, GlobalTransform, Transform},
    ecs::prelude::Entity,
    prelude::*,
    renderer::*,
    utils::fps_counter::FPSCounter,
};
use rand::prelude::*;

use components::{
    block::Block,
    cursor::Cursor,
    kind_generator::KindGenerator,
    playfield::stack::Stack,
    playfield::{playfield_clear::PlayfieldClear, playfield_push::PlayfieldPush},
    spritesheet_loader::{load_sprite_sheet, SpriteSheetLoader},
};

use data::block_data::BLOCKS;

pub struct GameMode {
    rng_seed: [u8; 16],
    config: DisplayConfig,
}

impl GameMode {
    pub fn new(rng_seed: [u8; 16], config: DisplayConfig) -> GameMode {
        GameMode { rng_seed, config }
    }

    // creates all entities with block components attached, spritesheet data with sprite_number
    pub fn create_blocks(world: &mut World, kinds: Vec<i32>) -> Vec<Entity> {
        world.register::<Block>();
        let mut block_entities: Vec<Entity> = Vec::new();

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            trans.scale = Vector3::new(4.0, 4.0, 4.0);

            // set position instantly so no weird spawn flash happens
            let (x, y) = Stack::i2xy(i);
            let mut b = Block::new(i as u32, kinds[i], x as i32, y as i32);

            let sprite_render_block = SpriteRender {
                sprite_sheet: SpriteSheetLoader::load_blocks_sprite_sheet(world),
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };

            block_entities.push(
                world
                    .create_entity()
                    .with(sprite_render_block)
                    .with(b)
                    .with(GlobalTransform::default())
                    .with(trans)
                    .build(),
            );
        }

        block_entities
    }

    // create a camera that should have the same dimensions as the
    // display_config.ron. TODO: use the dimensions
    fn initialise_camera(&mut self, world: &mut World) {
        let mut transform = Transform::default();
        transform.translation.z = 1.0;

        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,
                self.config.dimensions.unwrap().0 as f32,
                self.config.dimensions.unwrap().1 as f32,
                0.0,
            ))).with(transform)
            .build();
    }
}

impl<'a, 'b> SimpleState<'a, 'b> for GameMode {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // create random generator for random seeded numbers
        let mut kind_gen: KindGenerator = KindGenerator {
            rng: SmallRng::from_seed(self.rng_seed),
        };
        let kinds = kind_gen.create_stack(5, 8);

        let block_entities = GameMode::create_blocks(world, kinds);
        // add the random number generator as a global resource to be used
        world.add_resource::<KindGenerator>(kind_gen);

        // load the cursor sprite and attach its data component
        let sprite_sheet = SpriteRender {
            sprite_sheet: load_sprite_sheet(world, "cursor.png", "cursor_spritesheet.ron"),
            sprite_number: 0,
            flip_horizontal: false,
            flip_vertical: false,
        };

        // cursor transform
        let mut trans = Transform::default();
        trans.scale = Vector3::new(2.0, 2.0, 2.0);

        let cursor = Cursor::new(2.0, 5.0);
        cursor.set_position(&mut trans);

        // generate a cursor entity
        world.register::<Cursor>();
        let cursor_entity = world
            .create_entity()
            .with(sprite_sheet)
            .with(Transparent::default())
            .with(cursor)
            .with(GlobalTransform::default())
            .with(trans)
            .build();

        world.add_resource::<FPSCounter>(Default::default());

        // Create a Playfield with a stack, clear, push component,
        // STACK gives anccess to blocks and cursor dependant on the general storages
        world.register::<Stack>();
        world.register::<PlayfieldClear>();
        world.register::<PlayfieldPush>();
        world
            .create_entity()
            .with(PlayfieldClear::default())
            .with(PlayfieldPush::default())
            .with(Stack::new(block_entities, cursor_entity))
            .build();

        self.initialise_camera(world);
    }
}
