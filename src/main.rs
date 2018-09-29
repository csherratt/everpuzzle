extern crate amethyst;
extern crate rand;

use amethyst::prelude::*;
use amethyst::renderer::*;
use amethyst::core::TransformBundle;
use amethyst::input::InputBundle;
use rand::prelude::*;

mod data;
mod basics;
mod game_modes;
mod systems;
use systems::block_system::BlockSystem;
use systems::playfield_system::PlayfieldSystem;
use game_modes::game_mode::GameMode;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/src/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
            .with_pass(DrawSprite::new()),
    );

    const SOME_SEED: [u8; 16] = [0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    // create some randomized seed to be shared
    let mut rand_seed: [u8; 16] = [0; 16];
    for x in &mut rand_seed {
        *x = rand::random::<u8>();
    }

    let binding_path = format!(
        "{}/src/resources/controller_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(PlayfieldSystem {is_down: false}, "playfield_system", &["input_system"])
        .with(BlockSystem, "block_system", &[]);
        
    let assets_dir = format!("{}/src/sprites/", env!("CARGO_MANIFEST_DIR"));
    let mut game = Application::<GameData>::new(assets_dir, GameMode::new(SOME_SEED), game_data)?;

    game.run();

    Ok(())
}

