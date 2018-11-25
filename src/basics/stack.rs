#![allow(dead_code)]
use amethyst::ecs::prelude::{Entity, Component, DenseVecStorage};
use data::block_data::COLS;

pub struct Stack {
	entities: Vec<Entity>,
}

impl Default for Stack {
	fn default() -> Stack {
		Stack {
			entities: Vec::new(),
		}
	}
}

impl Stack {
	pub fn new(entities: Vec<Entity>) -> Stack {
		Stack {
			entities,
			..Default::default()
		}
	}

	// simple way to get an entity back
	pub fn from_i(&self, i: usize) -> Entity {
		self.entities[i]
	}

	// shouldnt be used too often, rather use i2xy to get the iterator calculated once
	pub fn from_xy(&self, x: usize, y: usize) -> Entity {
		self.entities[Stack::xy2i(x, y)]
	}

	// convert an x and y coordinate to i
	// use this if you want to back convert from an x and y
	// this is most often used when only one parameter changes and the other one stays
	// example: for x in 0..10 {
	// 		xy2i(x, 0) // searches through 0 until 10 from y at 0	
	// }
	pub fn xy2i(x: usize, y: usize) -> usize {
		y * COLS + x	
	}

	// use this instead of calling from_xy multiple times
	// converts an iterator i back to x and y
	pub fn i2xy(i: usize) -> (usize, usize) {
		(
			i % COLS,
			((i / COLS) as f64).floor() as usize
			// f32 floor changes to f64,
			// so why not go to f64 instantly
		)	
	}
}

impl Component for Stack {
    type Storage = DenseVecStorage<Self>;
}
