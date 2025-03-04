use std::collections::HashMap;

pub struct TranspositionTable {
	map: HashMap<u64, Transposition>,
	lookups: u64,
	hits: u64
}

impl TranspositionTable {
	pub fn new(capacity: usize) -> Self {
		Self {
			map: HashMap::with_capacity(capacity),
			lookups: 0,
			hits: 0
		}
	}

	pub fn get(&self, hash: u64) -> Option<&Transposition> {
		let res = self.map.get(&hash);

		// self.lookups += 1;

		// if res.is_some() {
		// 	self.hits += 1;
		// }

		return res;
	}

	pub fn set(&mut self, hash: u64, transposition: Transposition) {
		self.map.insert(hash, transposition);
	}

	pub fn len(&self) -> usize {
		self.map.len()
	}

	pub fn lookups(&self) -> u64 {
		self.lookups
	}

	pub fn hits(&self) -> u64 {
		self.hits
	}

	pub fn hit_percent(&self) -> f32 {
		self.hits as f32 / self.lookups as f32
	}

	pub fn reset_stats(&mut self) {
		self.lookups = 0;
		self.hits = 0;
	}
}

#[derive(Clone)]
pub struct Transposition {
	depth: u16,
	evaluation: i32,
	// position: Position
}

impl Transposition {
	pub fn new(depth: u16, evaluation: i32) -> Self {
		Self {
			depth,
			evaluation,
			// position
		}
	}

	pub fn depth(&self) -> u16 {
		self.depth
	}

	pub fn evaluation(&self) -> i32 {
		self.evaluation
	}

	// pub fn position(&self) -> &Position {
	// 	&self.position
	// }
}