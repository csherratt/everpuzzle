use amethyst::ecs::{Component, DenseVecStorage};

pub struct PlayfieldClear {
	pub clear_queue: Vec<u32>,
	pub combo_counter: u32, 
	pub chain: u32,
	pub last_chain: u32,
	pub blocks_cleared: u32,
}

impl Default for PlayfieldClear {
	fn default() -> PlayfieldClear {
		PlayfieldClear {
			clear_queue: Vec::new(),
			combo_counter: 0,
			chain: 1,
			last_chain: 1,
			blocks_cleared: 0,
		}
	}
}

impl Component for PlayfieldClear {
    type Storage = DenseVecStorage<Self>;
}
