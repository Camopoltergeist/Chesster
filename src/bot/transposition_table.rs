use std::{collections::HashMap, sync::RwLock};

pub struct TranspositionTable {
	map: RwLock<HashMap<u64, Transposition>>,
}

impl TranspositionTable {
	pub fn new(capacity: usize) -> Self {
		Self {
			map: RwLock::new(HashMap::with_capacity(capacity))
		}
	}

	pub fn get(&mut self, hash: u64) -> Option<i32> {
		let rwguard = self.map.read().unwrap();
		let tp = rwguard.get(&hash).cloned();
		drop(rwguard);

		if tp.is_none() {
			return None;
		};

		let tp = tp.unwrap();

		return Some(tp.evaluation);
	}

	pub fn set(&mut self, hash: u64, transposition: Transposition) {
		let mut rwguard = self.map.write().unwrap();
		rwguard.insert(hash, transposition);
	}
}

#[derive(Clone)]
pub struct Transposition {
	age: u16,
	evaluation: i32
}