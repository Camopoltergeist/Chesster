use std::{mem, sync::atomic::{AtomicU64, Ordering}};

const TABLE_SIZE: usize = 200000000;

pub struct TranspositionTable {
	map: Vec<Transposition>,
	lookups: u64,
	hits: u64
}

impl TranspositionTable {
	pub fn new() -> Self {
		let mut map = Vec::with_capacity(TABLE_SIZE);

		for _ in 0..TABLE_SIZE {
			map.push(Transposition::new(0, 0, 0));
		}

		Self {
			map,
			lookups: 0,
			hits: 0
		}
	}

	pub fn get(&mut self, hash: u64) -> &mut Transposition {
		let index = hash as usize % TABLE_SIZE;
		let res = &mut self.map[index];

		return res;
	}

	// pub fn set(&mut self, hash: u64, transposition: Transposition) {
	// 	let index = hash as usize % TABLE_SIZE;

	// 	self.map[index] = transposition;
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

pub struct Transposition {
	hash_check: AtomicU64,
	value: AtomicU64,
	// position: Position
}

impl Transposition {
	pub fn new(hash: u64, depth: u32, evaluation: i32) -> Self {
		let depth_u64 = (depth as u64) << 32;
		let eval_u64 = unsafe { mem::transmute::<_, u32>(evaluation) } as u64;

		let value = depth_u64 | eval_u64;
		let hash_check = hash ^ value;

		Self {
			hash_check: AtomicU64::new(hash_check),
			value: AtomicU64::new(value)
		}
	}

	pub fn depth(&self) -> u32 {
		let depth = self.value.load(Ordering::Relaxed) >> 32;

		return depth as u32;
	}

	pub fn evaluation(&self) -> i32 {
		let eval = (self.value.load(Ordering::Relaxed) | 0xFFFFFFFF) as u32;

		return unsafe { mem::transmute(eval) }
	}

	pub fn hash_matches(&self, hash: u64) -> bool {
		return (self.value.load(Ordering::Relaxed) ^ self.hash_check.load(Ordering::Relaxed)) == hash;
	}

	// pub fn position(&self) -> &Position {
	// 	&self.position
	// }
}

impl Clone for Transposition {
	fn clone(&self) -> Self {
		Self {
			hash_check: AtomicU64::new(self.hash_check.load(Ordering::Relaxed)),
			value: AtomicU64::new(self.value.load(Ordering::Relaxed)),
		}
	}
}
