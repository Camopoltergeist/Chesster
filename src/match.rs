use std::{thread::{self, JoinHandle}, time::Duration};

use crate::{board::{game_state::GameState, moove::Move, position::Position}, bot::Bot, player::Player};

pub struct Match {
    position: Vec<Position>,
    
    white_bot: Option<Box<dyn Bot>>,
    black_bot: Option<Box<dyn Bot>>,

    search_thread: Option<JoinHandle<(i32, Move)>>,
    search_time: Duration,
}

impl Match {
    pub fn new(position: &Position, white_bot: Option<Box<dyn Bot>>, black_bot: Option<Box<dyn Bot>>, search_time: Duration) -> Self {
        Self {
            position: vec![position.clone()],
            white_bot,
            black_bot,
            search_thread: None,
            search_time
        }
    }

    pub fn position(&self) -> &Position {
        &self.position.last().unwrap()
    }

    pub fn set_position(&mut self, position: &Position) {
        self.position = vec![position.clone()];
    }

    pub fn make_move(&mut self, moove: Move) {
        let mut moved_position = self.position().clone();

        moved_position.make_move(moove);

        self.position.push(moved_position);

        self.calculate_bot_move();
    }

    pub fn get_position_n_moves_ago(&self, n: usize) -> Option<&Position> {
        let index = self.position.len() - 1 - n;

        self.position.get(index)
    }

    pub fn position_count(&self) -> usize {
        self.position.len()
    }

    pub fn undo_to_n_moves_ago(&mut self, n: usize) -> &Position {
        if !self.move_can_be_made() {
            return self.position();
        }

        let final_length = self.position.len().saturating_sub(n);

        self.position.truncate(final_length);

        self.position()
    }

    pub fn calculate_bot_move(&mut self) {
        if self.position().get_game_state() != GameState::Ongoing {
            return;
        }

        let bot = match self.position.last().unwrap().current_player() {
            Player::White => {
                if let Some(bot) = &self.white_bot {
                    bot
                }
                else {
                    return;
                }
            },
            Player::Black => {
                if let Some(bot) = &self.black_bot {
                    bot
                }
                else {
                    return;
                }
            }
        };

        let pos = self.position().clone();
        let st = self.search_time.clone();
        let b = dyn_clone::clone_box(&(**bot));

        self.search_thread = Some(thread::spawn(move || {
            return b.search_best_move(&pos, st);
        }));
    }

    pub fn move_can_be_made(&self) -> bool {
        if let Some(thread_handle) = &self.search_thread {
            thread_handle.is_finished()
        }
        else {
            true
        }
    }

    pub fn bot_move_available(&self) -> bool {
        if let Some(thread_handle) = &self.search_thread {
            thread_handle.is_finished()
        }
        else {
            false
        }
    }

    pub fn wait_until_calculation_finished(&self) {
        if let Some(t) = &self.search_thread {
            while !t.is_finished() { }
        }
    }

    pub fn get_searched_move(&mut self) -> (i32, Move) {
        if let Some(t) = self.search_thread.take() {
            return t.join().unwrap().clone();
        }

        panic!("no search thread in progress");
    }
}