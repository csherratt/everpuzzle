use amethyst::prelude::*;
use amethyst::renderer::*;
use amethyst::assets::*;

pub struct SpriteSheetLoader {
    pub block_handle: SpriteSheetHandle
}

impl SpriteSheetLoader {
    pub fn new(world: &mut World) -> SpriteSheetLoader {
        SpriteSheetLoader {
            block_handle: SpriteSheetLoader::load_blocks_sprite_sheet(world)
        }
    }

    pub fn load_blocks_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
        let loader = world.read_resource::<Loader>();

        let texture_handle = {
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "blocks_orig.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        // `texture_id` is a application defined ID given to the texture to store in the `World`.
        // This is needed to link the texture to the sprite_sheet.
        let texture_id = 0;
        world.write_resource::<MaterialTextureSet>()
            .insert(texture_id, texture_handle);

        const SPRITESHEET_SIZE: (f32, f32) = (144.0, 144.0);

        // Create the sprite for the paddles.
        //
        // Texture coordinates are expressed as a proportion of the sprite sheet's dimensions between
        // 0.0 and 1.0, so they must be divided by the width or height.
        //
        // In addition, on the Y axis, texture coordinates are 0.0 at the bottom of the sprite sheet and
        // 1.0 at the top, which is the opposite direction of pixel coordinates, so we have to invert
        // the value by subtracting the pixel proportion from 1.0.
        let mut all_sprites: Vec<Sprite> = Vec::new();
        for y in 0..9 {
            for x in 0..9 {
                all_sprites.push(Sprite {
                    width: 16.0,
                    height: 16.0,
                    offsets: [-8.0, -8.0],
                    tex_coords: TextureCoordinates {
                        left: x as f32 * 16.0 / SPRITESHEET_SIZE.0,
                        right: (x as f32 + 1.0) * 16.0 / SPRITESHEET_SIZE.0,
                        bottom: 1.0 -  (y as f32 + 1.0) * 16.0 / SPRITESHEET_SIZE.1,
                        top: 1.0  -  y as f32 * 16.0 / SPRITESHEET_SIZE.1,
                    }
                })
            }
        }

        // Collate the sprite layout information into a sprite sheet
        let sprite_sheet = SpriteSheet {
            texture_id,
            sprites: all_sprites,
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load_from_data(sprite_sheet, (), &sprite_sheet_store)
        };

        sprite_sheet_handle
    }
}

pub fn load_sprite_sheet(world: &mut World, name: &str, filename: &str) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();

    // link texture with spritesheet
    // TODO: Texture id should be unique
    let texture_id = 1;
    world.write_resource::<MaterialTextureSet>()
        .insert(texture_id, {
            loader.load(
                name,
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &world.read_resource::<AssetStorage<Texture>>()
            )   
        });
    
    // spritesheet_handle return
    let sprite_sheethandle = {
        loader.load(
            filename,
            SpriteSheetFormat,
            texture_id,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };

    sprite_sheethandle
}
