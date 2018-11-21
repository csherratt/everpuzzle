use amethyst::{
    prelude::*,
    renderer::*,
    core::{Transform, GlobalTransform, cgmath::Vector3},
    utils::fps_counter::FPSCounter,
    ecs::prelude::Entity,
};
use rand::prelude::*;

use basics::{
    block::Block,
    cursor::Cursor,
    spritesheet_loader::{
        SpriteSheetLoader,
        load_sprite_sheet
    },
    kind_generator::KindGenerator,
};

use data::block_data::{COLS, BLOCKS};
use data::helpers::i2tuple;

pub struct BlockStack {
    pub entities: Vec<Entity>,
}

impl Default for BlockStack {
    fn default() -> BlockStack {
        BlockStack {
            entities: Vec::new(),
        }
    }
}

impl BlockStack {
    fn new(entities: Vec<Entity>) -> BlockStack {
        BlockStack {
            entities,
        }
    }
}

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
    pub fn create_blocks(world: &mut World, kinds: Vec<i32>) -> BlockStack {
        world.register::<Block>();
        let mut entities = Vec::new();

        for i in 0..BLOCKS {
            let mut trans = Transform::default();
            trans.scale = Vector3::new(4.0, 4.0, 4.0);

            // set position instantly so no weird spawn flash happens
            let (x, y) = i2tuple(i);
            let mut b = Block::new(kinds[i], x as i32, y as i32);

            let sprite_render_block = SpriteRender {
                sprite_sheet: SpriteSheetLoader::load_blocks_sprite_sheet(world),
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

        // add all neighbors as entities
        let mut storage = world.write_storage::<Block>();
        for i in COLS..BLOCKS {
            let mut b = storage.get_mut(entities[i]).unwrap();            
            b.down = Some(entities[i - COLS]);
        }

        BlockStack::new(entities)
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

        let block_stack = GameMode::create_blocks(world, kinds);
        world.add_resource::<BlockStack>(block_stack);
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

        world.add_resource::<FPSCounter>(Default::default());

        self.initialise_camera(world);
    }
}

