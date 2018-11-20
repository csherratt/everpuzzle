#![allow(dead_code)]
use amethyst::{
    ecs::{
        prelude::{
            Component, 
            DenseVecStorage
        }, 
        shred::FetchMut,
        storage::{
            Storage,
            MaskedStorage,
        },
        join::{Join, JoinIter},
    },
    core::Transform,
    renderer::SpriteRender,
};
use data::{
    helpers::tuple2i,
    block_data::{COLS, ROWS},
};

const LAND_ANIM: [usize; 10] = [2, 2, 2, 3, 3, 3, 4, 4, 4, 0];
const LAND_TIME: i32 = 10;
const FLASH_ANIM: [usize; 4] = [6, 6, 0, 0];
const FLASH_TIME: i32 = 44; 

// All states a blocks can be in
#[derive(PartialEq, Clone)]
pub enum States {
    Idle,
    Hang,
    Fall,
    Land,
    Move,
    Clear,
}

pub struct Block {
    pub id: u32, // id to be used with join().get_unchecked(u32)
    pub kind: i32, // sprite_number or -1
    pub pos: (f32, f32),
    pub offset: (f32, f32),
    pub can_fall: bool,
    pub should_clear: bool, 

    // animation
    pub anim_counter: usize, 
    pub anim_offset: usize,

    // state machine variables
    pub counter: i32,
    pub state: States,
    pub move_dir: f32, // used for move animation movement

    // clear variables
    pub chainable: bool, 
    pub clear_counter: i32, 
    pub clear_anim_counter: i32, 
    pub clearing: bool, 
    pub clear_time: i32,
    pub clear_start_counter: i32,
}

impl Block {
    pub fn new(id: u32, kind: i32, pos: (f32, f32)) -> Block {
        Block {
            id,
            kind,
            pos,
            offset: (0.0, 0.0),
            can_fall: false,
            should_clear: false,

            anim_counter: 0,
            anim_offset: 0,
            counter: 0,
            state: States::Idle,
            move_dir: 1.0, 

            // clearing
            chainable: false,
            clear_counter: 0,
            clear_anim_counter: 0,
            clearing: false,
            clear_time: 0,
            clear_start_counter: 0,
        }
    }

    // changes the contents of this block with a neighbor
    pub fn swap_properties(&mut self, other: &mut Block) {
        other.kind = self.kind;
        other.state = self.state.clone();
        other.counter = self.counter;
        other.chainable = self.chainable;
        other.anim_offset = self.anim_offset;
        other.anim_counter = self.anim_counter;

        self.reset();
    }

    // reset all variables to their standard
    // TODO: maybe add Default::default() instead?
    pub fn reset(&mut self) {
        self.kind = -1;
        self.state = States::Idle;
        self.counter = 0;
        self.anim_counter = 0;
        self.anim_offset = 0;

        // clearing
        self.chainable = false;
        self.clear_time = 0;
        self.clear_counter = 0;
        self.clear_anim_counter = 0;
        self.clearing = false;
        self.clear_start_counter = 0;
    }

    // this is empty when the kind isnt anything and the state is idle
    // you can check for "air" or "invisible" blocks with this
    pub fn is_empty(&mut self) -> bool {
        self.kind == -1 && self.state == States::Idle
    }

    // when a block above isnt hanging
    // other isnt empty and the block is currently falling
    // self.state is idle or kind is -1
    // valid blocks are swapping multiple times
    pub fn is_swappable(
        &mut self, 
        other_block: &mut Block,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) -> bool {
        // TODO garbage
        
        let up_block = self.get_neighbor(blocks, (0, 1));
        if let Some(up) = up_block {
            if up.state == States::Hang {
                return false;
            }
        }

        if !other_block.is_empty() && self.state == States::Fall {
            return true;
        }

        if self.state == States::Land && self.counter < LAND_TIME {
            return true;
        }

        if self.state == States::Idle || self.kind == -1 {
            return true;
        }

        if other_block.kind != -1 {
            if other_block.state == States::Move && self.state == States::Move {
                return true;
            }
        }

        return false;
    }

    // initiates the swapping of this and the right block kinds
    // checks wether a swap is even possible
    // cant swap if both blocks are empty
    // sets the time a swap takes
    pub fn swap(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        let right = self.get_neighbor(blocks, (1, 0)).unwrap();

        let check_right = self.is_swappable(right, blocks);
        let check_left = right.is_swappable(self, blocks);

        if check_right && check_left {
            if self.is_empty() && right.is_empty() {
                return
            }

            // hardcoded offsets, to not make kinds show up for 1 frame
            // in the wrong position
            self.offset.0 = 16.0;  
            right.offset.0 = -16.0;  

            let temp_offset_y: f32 = self.offset.1;
            self.offset.1 = right.offset.1;
            right.offset.1 = temp_offset_y;

            // swap kinds instantly
            let temp_kind: i32 = self.kind;
            self.kind = right.kind;
            right.kind = temp_kind;

            // counter set to tween length
            self.counter = 5;
            right.counter = 5;

            // set direction which will set the move animation direction
            self.move_dir = 1.0;
            right.move_dir = -1.0;

            self.change_state(States::Move);
            right.change_state(States::Move);
        }
    }
       
    // checks if any block is hanging below or is falling 
    pub fn check_for_hang(
        &mut self, 
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) -> bool {
        if let Some(other) = self.get_neighbor(blocks, (0, -1)) {
            if !self.is_empty() && (other.is_empty() || other.state == States::Hang) {
                return true;
            }
        }
        
        return false;
    }

    // visible only when the kind isnt -1
    // also sets the sprite_number of the sprite,
    // also decreases the animation counter each frame
    pub fn kind_visible(&mut self, sprite: &mut SpriteRender) {
        if self.anim_counter > 0 {
            self.anim_counter -= 1;
        }

        // if block is at bootom
        if self.pos.1 == 0.0 {
            // set to static blacked out sprite
            self.anim_offset = 1;
        }
   
        // only visible when kind isnt null and clearing ended
        if self.kind == -1 || self.clearing {
            sprite.sprite_number = 8;
        }
        else {
            sprite.sprite_number = self.kind as usize * 9 + self.anim_offset;
        }
    }

    // simply positions the transform in a grid pattern 
    pub fn set_position(&mut self, transform: &mut Transform) {
        transform.translation.x = self.pos.0 * 16.0 * transform.scale.x + self.offset.0;
        transform.translation.y = self.pos.1 * 16.0 * transform.scale.y + self.offset.1;
    }

    // combo detectable when
    // kidn isnt -1
    // currently landing and in its last counter time
    pub fn is_comboable(&mut self) -> bool {
        // TODO add garbage

        if self.pos.1 == 0.0 {
            return false;
        }

        if self.kind != -1 && self.state == States::Idle {
            return true;
        }

        if self.state == States::Land && self.counter < LAND_TIME {
            return true;
        }

        return false;
    }

    // wether this block is comboable and also matches with another kind
    pub fn is_comboable_with(&mut self, other: &mut Block) -> bool {
        if self.is_comboable() {
            if other.kind != -1 && other.kind == self.kind {
                return true;
            }
        }

        // check if kinds exist, then compare them
        return false;
    }

    // returns an array of comboable blocks including this block
    // wether 2 blocks have the same kind as this block - change state to CLEAR
    // returns an empty array otherwhise
    pub fn check_similar_blocks(
        &mut self, 
        b1: Option<&mut Block>, 
        b2: Option<&mut Block>
    ) -> Vec<u32> {
        if self.is_comboable() {
            if let Some(block1) = b1 {
                if let Some(block2) = b2 {
                    if block1.is_comboable_with(self) && block2.is_comboable_with(self) {
                        return vec![
                            self.id,
                            block1.id,
                            block2.id,
                        ]
                    }
                }
            }
        }

        return vec![];
    }

    // returns a block neighbor with the specific type if its in the boundary
    pub fn get_neighbor<T: Join>(
        &mut self, 
        blocks: &mut JoinIter<T>, 
        rel_pos: (i32, i32),
    ) -> Option<<T>::Type> {
        let pos = (self.pos.0 + rel_pos.0 as f32, self.pos.1 + rel_pos.1 as f32);

        if pos.0 > (COLS - 1) as f32 || pos.0 < 0.0 || pos.1 > ROWS as f32 || pos.1 < 0.0 {
            return None;
        }
        
        return blocks.get_unchecked(tuple2i(pos) as u32);
    }

    // only detects if this block can fall - sets the state to hang
    // resets chainable to false if this block cant fall
    pub fn idle_execute(
        &mut self, 
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        if self.check_for_hang(blocks) {
            self.change_state(States::Hang);
        }
        else {
            self.chainable = false;
        }
    }

    // enter hang with the counter set - making the frames count down
    fn hang_enter(&mut self) {
        self.counter = 10;
    }

    // when counter ends change to Fall
    fn hang_counter_end(&mut self) {
        self.change_state(States::Fall);
    }

    // falls to the block below
    // sets the block below to this blocks data
    // while this blocks data gets reset
    fn fall_execute(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        let down_block = self.get_neighbor(blocks, (0, -1));

        if let Some(down) = down_block {
            if down.is_empty() {
                self.swap_properties(down);
            }
            // update this falling block to the same down.counter to sync
            else if down.state == States::Hang {
                self.state = States::Hang; // skip hang_enter
                self.counter = down.counter;
            }
            else {
                self.change_state(States::Land);
            }
        }
        else {
            self.state = States::Idle;
        }
    }

    // set counter to the time it takes to land / animate
    fn land_enter(&mut self) {
        self.counter = LAND_TIME;
        self.anim_counter = LAND_TIME as usize;
    }

    // run down the animation
    fn land_execute(&mut self) {
        self.anim_offset = LAND_ANIM[LAND_TIME as usize - self.anim_counter - 1];
    }

    // once counter hits 0 change to Hang or Idle
    fn land_counter_end(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        let up_block = self.get_neighbor(blocks, (0, 1));  

        if let Some(up) = up_block {
            if up.state == States::Hang {
                self.change_state(States::Hang);
                self.counter = up.counter;
            }
            else {
                self.change_state(States::Idle);
            }
        }
    }

    // stop counting as a chainable block, stop animation
    fn land_exit(&mut self) {
        self.anim_offset = 0;
        self.chainable = false;
    }

    // start animating offset movement
    fn move_execute(&mut self) {
        self.offset.0 = self.move_dir * 16.0 + -self.move_dir * Block::ease_out_quad(
            5.0 - self.counter as f32,
            0.0, 16.0,
            5.0
        );
    }

    fn ease_out_quad(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let val = t / d;
        c * val * val + b
    }

    // set State to Idle and offset.x to 0, or transitions to Hang
    fn move_counter_end(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        // needs to check for hang so no state changes result 1 frame issues
        if self.check_for_hang(blocks) {
            self.change_state(States::Hang);
        }
        else {
            self.state = States::Idle;
            self.offset.0 = 0.0;
        }
    }

    // for safety reset the anim_counter
    fn clear_enter(&mut self) {
        self.anim_counter = 0;
    }

    // just animating the clearing via code
    fn clear_execute(&mut self) {
        if self.clear_time - self.clear_counter <= 0 && !self.clearing {
//            self.particles.spawn = true;
            self.clearing = true;
            //input state vibration
        }
        else {
            self.clear_counter += 1; 
            self.clear_anim_counter += 1;

            // split animations in 2 actions
            if self.clear_anim_counter < FLASH_TIME {
                // flashy anim
                if self.anim_counter == 0 {
                    self.anim_counter = 4;
                }
                else {
                    self.anim_offset = FLASH_ANIM[self.anim_counter];
                }
            }
            else {
                // just the face part
                self.anim_offset = 6;  
            }
        }
    }

    // simply go back to being idle
    fn clear_counter_end(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        // set all above this block to chain and hang
        for i in self.pos.1 as usize..ROWS {
            let up_block = blocks.get_unchecked(tuple2i((self.pos.0, i as f32)) as u32);

            if let Some(up) = up_block {
                if up.kind != -1 && up.state == States::Idle {
                    up.chainable = true;
                }
            }
        }

        self.change_state(States::Idle);
    }

    fn clear_exit(&mut self) {
        self.kind = -1;
        self.counter = 0;
        self.anim_offset = 0;

        // clear variables 
        self.clearing = false;
        self.clear_counter = 0;
        self.clear_anim_counter = 0;
    }

    // change the state to a new one, call state functions that implemnent
    // enter or exit, these dont require knowledge of other blocks
    pub fn change_state(&mut self, new_state: States) {
        if self.state != new_state {
            // exit functions called on old state
            match self.state {
                States::Land => self.land_exit(),
                States::Clear => self.clear_exit(),
                _ => ()
            }

            self.state = new_state;

            // enter functions on new state called
            match self.state {
                States::Hang => self.hang_enter(),
                States::Land => self.land_enter(),
                States::Clear => self.clear_enter(),
                _ => ()
            }
        }
    }

    // update the current FSM state
    // calls execute function every frame
    // counter_end when the counter hits 0 - used for transitions or setters
    //
    // takes a JoinIter to be able to access different &mut Blocks
    // which is really helpful to code logic with
    pub fn update_state(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        if self.counter > 0 {
            self.counter -= 1;
        }

        // calls the execute functions of the current state
        match self.state {
            States::Idle => self.idle_execute(blocks),
            States::Fall => self.fall_execute(blocks),
            States::Land => self.land_execute(),
            States::Move => self.move_execute(),
            States::Clear => self.clear_execute(),
            _ => ()
        }

        // state counter_end needs to happen after execute!
        if self.counter <= 0 {
            match self.state {
                States::Hang => self.hang_counter_end(),
                States::Land => self.land_counter_end(blocks),
                States::Move => self.move_counter_end(blocks),
                States::Clear => self.clear_counter_end(blocks),
                _ => ()
            }
        }
    }

    // wether up and down or left and right are the same kind 
    // all detected clears are returned in their id of the blocks 
    // so that they can be backtracked
    pub fn check_clear(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) -> Vec<u32> {
        let mut checks: Vec<u32> = Vec::new();

        let r = self.get_neighbor(blocks, (1, 0));
        let rr = self.get_neighbor(blocks, (2, 0));
        let d = self.get_neighbor(blocks, (0, -1));
        let dd = self.get_neighbor(blocks, (0, -2));

        checks.append(&mut self.check_similar_blocks(r, rr)); 
        checks.append(&mut self.check_similar_blocks(d, dd)); 

        checks
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
