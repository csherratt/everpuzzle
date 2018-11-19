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

#[derive(PartialEq, Clone)]
pub enum States {
    Idle,
    Hang,
    Fall,
}

pub struct Block {
    pub id: u32, // id to be used with join().get_unchecked(u32)
    pub kind: i32, // sprite_number or -1
    pub pos: (f32, f32),
    pub can_fall: bool,
    pub should_clear: bool, 
    pub chainable: bool, 

    // animation
    pub anim_counter: usize, 
    pub anim_offset: usize,

    // state machine variables
    pub counter: i32,
    pub state: States,
}

impl Block {
    pub fn new(id: u32, kind: i32, pos: (f32, f32)) -> Block {
        Block {
            id,
            kind,
            pos,
            can_fall: false,
            should_clear: false,
            chainable: false,

            anim_counter: 0,
            anim_offset: 0,
            counter: 0,
            state: States::Idle,
        }
    }

    pub fn swap_properties(&mut self, other: &mut Block) {
        other.kind = self.kind;
        other.state = self.state.clone();
        other.counter = self.counter;
        other.chainable = self.chainable;
        other.anim_offset = self.anim_offset;
        other.anim_counter = self.anim_counter;

        self.reset();
    }

    pub fn reset(&mut self) {
        self.kind = -1;
        self.state = States::Idle;
        self.counter = 0;
        self.anim_counter = 0;
        self.anim_offset = 0;

        // clearing
        self.chainable = false;
    }

    pub fn is_empty(&mut self) -> bool {
        self.kind == -1 && self.state == States::Idle
    }
       
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
        else {
            self.anim_offset = 0;
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
        transform.translation.x = self.pos.0 * 16.0 * transform.scale.x;
        transform.translation.y = self.pos.1 * 16.0 * transform.scale.y;
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

    fn hang_enter(&mut self) {
        self.counter = 10;
    }

    fn hang_counter_end(&mut self) {
        self.change_state(States::Fall);
    }

    fn fall_execute(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        let down_block = self.get_neighbor(blocks, (0, -1));

        if let Some(down) = down_block {
            if down.is_empty() {
                self.swap_properties(down);
            }
            else if down.state == States::Hang {
                self.state = States::Hang; // skip hang_enter
                self.counter = down.counter;
            }
            else {
                //TODO: Land
            }
        }
        else {
            self.state = States::Idle;
        }
    }

    // change the state to a new one, call state functions that implemnent
    // enter or exit, these dont require knowledge of other blocks
    pub fn change_state(&mut self, new_state: States) {
        if self.state != new_state {
            // exit functions called on old state
            match self.state {
                States::Hang => self.hang_enter(),
                _ => ()
            }

            self.state = new_state;

            // enter functions on new state called
            match self.state {
                _ => ()
            }
        }
    }

    pub fn update_state(
        &mut self,
        blocks: &mut JoinIter<&mut Storage<'_, Block, FetchMut<'_, MaskedStorage<Block>>>>
    ) {
        if self.counter > 0 {
            self.counter -= 1;
        }

        match self.state {
            States::Idle => self.idle_execute(blocks),
            States::Fall => self.fall_execute(blocks),
            _ => println!("STATE IMPLEMENTATION DOESNT EXIST")
        }

        // state counter_end needs to happen after execute!
        if self.counter <= 0 {
            match self.state {
                States::Hang => self.hang_counter_end(),
                _ => ()
            }
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
