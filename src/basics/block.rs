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
    block_data::COLS,
};

// All states a blocks can be in
#[derive(PartialEq, Clone)]
pub enum States {
    Idle,
    Hang,
    Fall,
    Land,
    Move,
}

pub struct Block {
    pub id: u32, // id to be used with join().get_unchecked(u32)
    pub kind: i32, // sprite_number or -1
    pub pos: (f32, f32),
    pub offset: (f32, f32),
    pub can_fall: bool,
    pub should_clear: bool, 
    pub chainable: bool, 

    // animation
    pub anim_counter: usize, 
    pub anim_offset: usize,

    // state machine variables
    pub counter: i32,
    pub state: States,
    pub land_anim: [usize; 10], 

    pub move_dir: f32, // used for move animation movement
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
            chainable: false,

            anim_counter: 0,
            anim_offset: 0,
            counter: 0,
            state: States::Idle,
            land_anim: [2, 2, 2, 3, 3, 3, 4, 4, 4, 0],
            move_dir: 1.0, 
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

        if self.state == States::Land && self.counter < self.land_anim.len() as i32 {
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
   
        if self.kind != -1 {
            sprite.sprite_number = self.kind as usize * 9 + self.anim_offset;
        }
        else {
            sprite.sprite_number = 8;
        }
    }

    // simply positions the transform in a grid pattern 
    pub fn set_position(&mut self, transform: &mut Transform) {
        transform.translation.x = self.pos.0 * 16.0 * transform.scale.x + self.offset.0;
        transform.translation.y = self.pos.1 * 16.0 * transform.scale.y + self.offset.1;
    }

    // combo detectable when
    // kidn isnt -1
    pub fn is_comboable(&mut self) -> bool {
        if self.pos.1 == 0.0 {
            return false
        }

        if self.kind != -1 {
            return true
        }

        return false
    }

    // wether this block is comboable and also matches with another kind
    pub fn is_comboable_with(&mut self, other: &mut Block) -> bool {
        if self.is_comboable() {
            if other.kind != -1 && other.kind == self.kind {
                return true
            }
        }

        // check if kinds exist, then compare them
        return false
    }

    // returns an array of comboable blocks including this block
    // wether 2 blocks have the same kind as this block - change state to CLEAR
    // returns an empty array otherwhise
    pub fn check_similar_blocks(&mut self, b1: Option<&mut Block>, b2: Option<&mut Block>) {
        if self.is_comboable() {
            if let Some(block1) = b1 {
                if let Some(block2) = b2 {
                    if block1.is_comboable_with(self) && block2.is_comboable_with(self) {
                        self.should_clear = true;
                        block1.should_clear = true;
                        block2.should_clear = true;
                    }
                }
            }
        }
    }

    // returns a block neighbor with the specific type if its in the boundary
    pub fn get_neighbor<T: Join>(
        &mut self, 
        blocks: &mut JoinIter<T>, 
        rel_pos: (i32, i32),
    ) -> Option<<T>::Type> {
        let range = tuple2i(self.pos) as i32 + rel_pos.0 + rel_pos.1 * COLS as i32;
    
        if range > 0 {
            blocks.get_unchecked(range as u32) 
        }
        else {
            None
        }
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
        self.counter = self.land_anim.len() as i32;
        self.anim_counter = self.land_anim.len();
    }

    // run down the animation
    fn land_execute(&mut self) {
        self.anim_offset = self.land_anim[self.land_anim.len() - self.anim_counter - 1];
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

    // change the state to a new one, call state functions that implemnent
    // enter or exit, these dont require knowledge of other blocks
    pub fn change_state(&mut self, new_state: States) {
        if self.state != new_state {
            // exit functions called on old state
            match self.state {
                States::Land => self.land_exit(),
                _ => ()
            }

            self.state = new_state;

            // enter functions on new state called
            match self.state {
                States::Hang => self.hang_enter(),
                States::Land => self.land_enter(),
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
            _ => ()
        }

        // state counter_end needs to happen after execute!
        if self.counter <= 0 {
            match self.state {
                States::Hang => self.hang_counter_end(),
                States::Land => self.land_counter_end(blocks),
                States::Move => self.move_counter_end(blocks),
                _ => ()
            }
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
