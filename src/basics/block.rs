use amethyst::{
    ecs::prelude::{Component, DenseVecStorage}, 
    core::Transform,
    renderer::SpriteRender
};
use std::collections::HashMap;
use states::{
    idle::Idle,
    fall::Fall,
    state::State,
};

pub struct Block {
    pub id: u32, // id to be used with join().get_unchecked(u32)
    pub kind: i32, // sprite_number or -1
    pub pos: (f32, f32),
    pub can_fall: bool,
    pub should_clear: bool, 

    // animation
    pub anim_counter: usize, 
    pub anim_offset: usize,

    pub states_execute: HashMap<String, fn(&mut Block)>,
    pub states_enter: HashMap<String, fn(&mut Block)>,
    pub states_exit: HashMap<String, fn(&mut Block)>,
    pub states_counter_end: HashMap<String, fn(&mut Block)>,
    pub counter: i32,
}

impl Block {
    pub fn new(id: u32, kind: i32, pos: (f32, f32)) -> Block {
        Block {
            id,
            kind,
            pos,
            can_fall: false,
            should_clear: false,
            anim_counter: 0,
            anim_offset: 0,
            states_execute: HashMap::new(),
            states_enter: HashMap::new(),
            states_exit: HashMap::new(),
            states_counter_end: HashMap::new(),
            counter: 0,
        }
    }

    // inits a certain states function and maps it to 4 hash maps 
    // this saves some copy pasting
    pub fn init_state<T: State>(&mut self, state_name: String) {
        self.states_execute.insert(state_name.clone(), T::execute);
        self.states_enter.insert(state_name.clone(), T::enter);
        self.states_exit.insert(state_name.clone(), T::exit);
        self.states_counter_end.insert(state_name.clone(), T::counter_end);
    }

    // inits all State functions to a certain String
    // Everything can now be called via a single String down the line
    pub fn init_events(&mut self) {
        self.init_state::<Idle>(String::from("IDLE"));
        self.init_state::<Fall>(String::from("FALL"));
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
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
