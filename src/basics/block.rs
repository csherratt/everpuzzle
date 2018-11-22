#![allow(dead_code)]
use amethyst::{
    ecs::{
        prelude::{
            Entity,
            Component, 
            DenseVecStorage
        }, 
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
    pub kind: i32, // sprite_number or -1
    pub x: i32,
    pub y: i32,
    pub down: Option<Entity>,
    pub can_fall: bool,
    pub state: States,
}

impl Default for Block {
    fn default() -> Block {
        Block {
            kind: 0,
            x: 0,
            y: 0,
            down: None,
            can_fall: false,
            state: States::Idle,
        }
    }
}

impl Block {
    pub fn new(kind: i32, x: i32, y: i32) -> Block {
        Block { kind, x, y, ..Default::default() }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}
