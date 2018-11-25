use amethyst::ecs::*;
use basics::{
	block::Block,
	stack::Stack,
};

use data::block_data::BLOCKS;

pub struct PlayfieldSystem;
impl<'a> System<'a> for PlayfieldSystem {
    type SystemData = (
		WriteStorage<'a, Block>,
		Read<'a, Stack>,
    );

    fn run(&mut self, (mut blocks, stack): Self::SystemData) {

    }
}
