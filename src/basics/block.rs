#![allow(dead_code, unused_imports)]
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::marker::Copy;
use std::clone::Clone;
use block_states::land::LAND_TIME;

#[derive(Copy, Clone)]
pub struct Block {
    pub kind: i32, // sprite_number or -1
    pub x: i32,
    pub y: i32,
    pub offset: (f32, f32),
    pub move_dir: f32,
    pub state: &'static str,
    pub counter: u32,

    // clear variables
    pub chainable: bool,
    pub clearing: bool,
    pub clear_counter: u32,
    pub clear_anim_counter: u32,
    pub clear_time: u32,

    // anim counters
    pub anim_counter: u32,
    pub anim_offset: u32,
}

impl Default for Block {
    fn default() -> Block {
        Block {
            kind: 0,
            x: 0,
            y: 0,
            offset: (0.0, 0.0),
            move_dir: 1.0,
            state: "IDLE",
            counter: 0,
            
            // clear variables
            chainable: false,
            clearing: false,
            clear_counter: 0,
            clear_anim_counter: 0,
            clear_time: 0,

            // anim counters
            anim_counter: 0,
            anim_offset: 0,
        }
    }
}

impl Block {
    pub fn new(kind: i32, x: i32, y: i32) -> Block {
        Block { kind, x, y, ..Default::default() }
    }

    // a block is empty when its kind is -1 so it turns invisible and
    // its state is always idle 
    pub fn is_empty(&self) -> bool {
        self.kind == -1 && self.state == "IDLE"
    }

    // a block is swappable when: 
    // its state isnt idle or its invisible,
    // other block isnt empty and currently in fall,
    // its state is land and its counter still below land time 
    // valid blocks are currently swapping 
    pub fn is_swappable(&self, other: &Block, above_block: Option<&Block>) -> bool {
        if let Some(above) = above_block {
            if above.state == "HANG" {
                return true;
            }
        } 

        if !other.is_empty() && self.state == "FALL" {
            return true;
        }

        if self.state == "LAND" && self.counter < LAND_TIME {
            return true;
        }

        if self.state == "IDLE" || self.kind == -1 {
            return true;
        }

        if other.kind != -1 && other.state == "MOVE" && self.state == "MOVE" {
            return true;
        }

        return false;
    }

    // set properties from another block
    pub fn set_properties(&mut self, other: Block) {
        self.kind = other.kind;
        self.state = other.state;
    }

    // reset distinct values 
    pub fn reset(&mut self) {
        self.kind = -1; 
        self.state = "IDLE";
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}