//! Unused enum formerly used to represent evaluation.

use std::{cmp::Ordering, ops::Neg};

#[derive(Debug, Clone, PartialEq)]
pub enum Evaluation {
	Score(f32),
	Checkmate(bool),
	Stalemate,
	Initial,
}

impl PartialOrd for Evaluation {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			// Comparisons with same match
			(Self::Score(self_score), Self::Score(other_score)) => self_score.partial_cmp(other_score),
			(Self::Checkmate(self_is_winning), Self::Checkmate(other_is_winning)) => self_is_winning.partial_cmp(other_is_winning),
			(Self::Stalemate, Self::Stalemate) => Some(Ordering::Equal),

			// Score to checkmate comparisons
			(Self::Score(_), Self::Checkmate(is_winning)) => {
				if *is_winning {
					Some(Ordering::Less)
				}
				else {
					Some(Ordering::Greater)
				}
			},

			(Self::Checkmate(is_winning), Self::Score(_)) => {
				if *is_winning {
					Some(Ordering::Greater)
				}
				else {
					Some(Ordering::Less)
				}
			},

			// Score to stalemate comparisons
			(Self::Score(self_score), Self::Stalemate) => self_score.partial_cmp(&0.0),
			(Self::Stalemate, Self::Score(other_score)) => 0.0.partial_cmp(other_score),

			// Checkmate to stalemate comparisons
			(Self::Stalemate, Self::Checkmate(is_winning)) => {
				if *is_winning {
					Some(Ordering::Less)
				}
				else {
					Some(Ordering::Greater)
				}
			},

			(Self::Checkmate(is_winning), Self::Stalemate) => {
				if *is_winning {
					Some(Ordering::Greater)
				}
				else {
					Some(Ordering::Less)
				}
			},

			// Initial value comparisons
			(Self::Initial, Self::Score(_)) => Some(Ordering::Less),
			(Self::Initial, Self::Checkmate(_)) => Some(Ordering::Less),
			(Self::Initial, Self::Stalemate) => Some(Ordering::Less),

			(Self::Score(_), Self::Initial) => Some(Ordering::Greater),
			(Self::Checkmate(_), Self::Initial) => Some(Ordering::Greater),
			(Self::Stalemate, Self::Initial) => Some(Ordering::Greater),

			(Self::Initial, Self::Initial) => Some(Ordering::Equal)
		}
	}
}

impl Neg for Evaluation {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Self::Score(score) => Self::Score(-score),
			Self::Checkmate(is_winning) => Self::Checkmate(!is_winning),
			Self::Stalemate => Self::Stalemate,
			Self::Initial => Self::Initial
		}
	}
}
