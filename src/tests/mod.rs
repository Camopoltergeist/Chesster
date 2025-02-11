use crate::board::moove::Move;

#[cfg(test)]

pub mod starting_position;

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