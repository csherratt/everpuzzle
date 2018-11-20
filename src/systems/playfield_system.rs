use amethyst::ecs::*;
use amethyst::core::Transform;
use basics::{
    block::{Block, States},
    cursor::Cursor,
    playfield::Playfield,
};
use data::{
    helpers::tuple2i,
    block_data::{
        ROWS,
        COLS,
    },
};

pub struct PlayfieldSystem {
    signal_raise: bool,
    smooth_raise: bool,
    raise_blocked_counter: i32,
    offset_counter: f32,
    any_clears: bool,
    any_top_blocks: bool,
}

impl Default for PlayfieldSystem {
    fn default() -> PlayfieldSystem {
        PlayfieldSystem {
            signal_raise: false,
            smooth_raise: false,
            raise_blocked_counter: 0,
            offset_counter: 0.0,
            any_clears: false,
            any_top_blocks: false,
        }
    }
}

impl<'a> System<'a> for PlayfieldSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Playfield>,
        WriteStorage<'a, Block>,
        WriteStorage<'a, Cursor>,
    );

    fn run(&mut self, (
        mut transforms,
        playfields,
        mut blocks,
        mut cursors,
    ): Self::SystemData) {
        // just pushes the blocks and cursors by an offset
        for (transform, _playfield) in (&mut transforms, &playfields).join() {
            for block in (&mut blocks).join() {
                block.offset.1 = transform.translation.y;
            }

            for cursor in (&mut cursors).join() {
                cursor.offset.1 = transform.translation.y;
            }

            transform.translation.y += 0.1;
        }

        // setter for any current clears
        self.any_clears = false;
        for b in (&mut blocks).join() {
            if b.state == States::Clear { // Garbage
                self.any_clears = true;
            }
        }

        self.any_top_blocks = false;
        let mut search_blocks = (&mut blocks).join();
        for x in 0..COLS {
            let b = search_blocks.get_unchecked(tuple2i((x as f32, (ROWS - 1) as f32)) as u32).unwrap();

            if b.kind != -1 && b.state == States::Idle { // garbage
                self.any_top_blocks = true;
            }
        }
    }
}

