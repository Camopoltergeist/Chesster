use std::collections::HashMap;

pub struct TranspositionTable {
	map: HashMap<u64, Transposition>,
}

impl TranspositionTable {
	pub fn new(capacity: usize) -> Self {
		Self {
			map: HashMap::with_capacity(capacity)
		}
	}

	pub fn get(&self, hash: u64) -> Option<&Transposition> {
		self.map.get(&hash)
	}

	pub fn set(&mut self, hash: u64, transposition: Transposition) {
		self.map.insert(hash, transposition);
	}

	pub fn len(&self) -> usize {
		self.map.len()
	}
}

#[derive(Clone)]
pub struct Transposition {
	depth: u16,
	evaluation: i32
}

impl Transposition {
	pub fn new(depth: u16, evaluation: i32) -> Self {
		Self {
			depth,
			evaluation
		}
	}

	pub fn depth(&self) -> u16 {
		self.depth
	}

	pub fn evaluation(&self) -> i32 {
		self.evaluation
	}
}