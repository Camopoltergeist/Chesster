use std::{cmp::Ordering, fmt::Display, ops::Neg};

#[derive(Debug, PartialEq)]
pub enum Evaluation {
	Score(f32),
	Checkmate(i32),
	Stalemate
}

impl PartialOrd for Evaluation {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			// Comparisons with same match
			(Self::Score(self_score), Self::Score(other_score)) => self_score.partial_cmp(other_score),
			(Self::Checkmate(self_move_count), Self::Checkmate(other_move_count)) => self_move_count.partial_cmp(other_move_count),
			(Self::Stalemate, Self::Stalemate) => Some(Ordering::Equal),

			// Score to checkmate comparisons
			(Self::Score(_), Self::Checkmate(other_move_count)) => {
				if *other_move_count < 0 {
					Some(Ordering::Greater)
				}
				else {
					Some(Ordering::Less)
				}
			},

			(Self::Checkmate(self_move_count), Self::Score(_)) => {
				if *self_move_count < 0 {
					Some(Ordering::Less)
				}
				else {
					Some(Ordering::Greater)
				}
			},

			// Score to stalemate comparisons
			(Self::Score(self_score), Self::Stalemate) => self_score.partial_cmp(&0.0),
			(Self::Stalemate, Self::Score(other_score)) => 0.0.partial_cmp(other_score),

			// Checkmate to stalemate comparisons
			(Self::Stalemate, Self::Checkmate(other_move_count)) => {
				if *other_move_count < 0 {
					Some(Ordering::Greater)
				}
				else {
					Some(Ordering::Less)
				}
			},

			(Self::Checkmate(self_move_count), Self::Stalemate) => {
				if *self_move_count < 0 {
					Some(Ordering::Less)
				}
				else {
					Some(Ordering::Greater)
				}
			},
		}
	}
}

impl Neg for Evaluation {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Self::Score(score) => Self::Score(-score),
			Self::Checkmate(move_count) => Self::Checkmate(-move_count),
			Self::Stalemate => Self::Stalemate
		}
	}
}
