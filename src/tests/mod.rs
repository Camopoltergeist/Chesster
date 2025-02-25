use crate::board::moove::Move;

#[cfg(test)]
mod rook_movement;
#[cfg(test)]
mod starting_position;
#[cfg(test)]
mod bishop_movement;
#[cfg(test)]
mod queen_movement;
#[cfg(test)]
mod knight_movement;
#[cfg(test)]
mod pawn_movement;
#[cfg(test)]
mod king_movement;
#[cfg(test)]
mod castling;
#[cfg(test)]
mod checks;
#[cfg(test)]
mod en_passant;
#[cfg(test)]
mod promotion;
#[cfg(test)]
mod slider_collision;
#[cfg(test)]
mod integrity_tests;
#[cfg(test)]
mod mailbox_integrity;


#[cfg(test)]
fn compare_moves(desired_moves: &Vec<Move>, received_moves: &Vec<Move>) -> bool {
    if desired_moves.len() != received_moves.len() {
        return false;
    }

    'outer: for dm in desired_moves {
        for m in received_moves {
            if *m == *dm {
                continue 'outer;
            }
        }

        return false;
    }

    return true;
}