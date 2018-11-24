use amethyst::ecs::prelude::*;

pub struct Stack {
	pub entities: Vec<Entity>,
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
}