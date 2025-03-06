use std::{array, collections::HashMap, mem, sync::Mutex};

const TABLE_SIZE: usize = 400000000;

pub struct TranspositionTable {
	map: Vec<Mutex<Option<Transposition>>>,
	lookups: u64,
	hits: u64
}

impl TranspositionTable {
	pub fn new() -> Self {
		let mut map = Vec::with_capacity(TABLE_SIZE);

		for _ in 0..TABLE_SIZE {
			map.push(Mutex::new(None));
		}

		Self {
			map,
			lookups: 0,
			hits: 0
		}
	}

	pub fn get(&self, hash: u64) -> &Mutex<Option<Transposition>> {
		let index = hash as usize % TABLE_SIZE;
		let res = &self.map[index];

		// self.lookups += 1;

		// if res.is_some() {
		// 	self.hits += 1;
		// }

		return res;
	}

	// pub fn set(&mut self, hash: u64, transposition: Transposition) {
	// 	self.map.insert(hash, transposition);
	// }

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
	hash_check: u64,
	value: u64
	// position: Position
}

impl Transposition {
	pub fn new(hash: u64, depth: u32, evaluation: i32) -> Self {
		let depth_u64 = (depth as u64) << 32;
		let eval_u64 = unsafe { mem::transmute::<_, u32>(evaluation) } as u64;

		let value = depth_u64 | eval_u64;

		Self {
			hash_check: hash ^ value,
			value
		}
	}

	pub fn depth(&self) -> u32 {
		let depth = self.value >> 32;

		return depth as u32;
	}

	pub fn evaluation(&self) -> i32 {
		let eval = (self.value | 0xFFFFFFFF) as u32;

		return unsafe { mem::transmute(eval) }
	}

	pub fn hash_matches(&self, hash: u64) -> bool {
		return (self.value ^ self.hash_check) == hash;
	}

	// pub fn position(&self) -> &Position {
	// 	&self.position
	// }
}