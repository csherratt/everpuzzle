#![allow(dead_code, unused_imports)]
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::marker::Copy;
use std::clone::Clone;

const FLASH_ANIM: [usize; 4] = [6, 6, 0, 0];
const FLASH_TIME: i32 = 44; 

#[derive(Copy, Clone)]
pub struct Block {
    pub kind: i32, // sprite_number or -1
    pub x: i32,
    pub y: i32,
    pub state: &'static str,
    pub counter: u32,
    pub chainable: bool,
    pub anim_counter: u32,
    pub anim_offset: u32,
}

impl Default for Block {
    fn default() -> Block {
        Block {
            kind: 0,
            x: 0,
            y: 0,
            state: "IDLE",
            counter: 0,
            chainable: false,
            anim_counter: 0,
            anim_offset: 0,
        }
    }
}

impl Block {
    pub fn new(kind: i32, x: i32, y: i32) -> Block {
        Block { kind, x, y, ..Default::default() }
    }

    pub fn is_empty(&mut self) -> bool {
        self.kind == -1 && self.state == "IDLE"
    }

    pub fn get_properties(&mut self, other: Block) {
        self.kind = other.kind;
        self.state = other.state;
    }

    pub fn reset(&mut self) {
        self.kind = -1; 
        self.state = "IDLE";
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}