use amethyst::ecs::Entity;

// A resource holding all block entities, used to go through entities later on
pub struct Stack {
	pub entities: Vec<Entity>
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
			entities
		}
	}
}

