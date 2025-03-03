use std::{thread::{self, JoinHandle}, time::Duration};

use crate::{board::{moove::Move, position::Position}, bot::Bot, player::Player};

pub struct Match {
    position: Position,
    
    white_bot: Option<Box<dyn Bot>>,
    black_bot: Option<Box<dyn Bot>>,

    search_thread: Option<JoinHandle<(i32, Move)>>,
    search_time: Duration,
}

impl Match {
    pub fn new(white_bot: Option<Box<dyn Bot>>, black_bot: Option<Box<dyn Bot>>, search_time: Duration) -> Self {
        Self {
            position: Position::default(),
            white_bot,
            black_bot,
            search_thread: None,
            search_time
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: &Position) {
        self.position = position.clone();
    }

    pub fn make_move(&mut self, moove: Move) {
        self.position.make_move(moove);

        let bot = match self.position.current_player() {
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

        let pos = self.position.clone();
        let st = self.search_time.clone();
        let f = bot.search_func();

        self.search_thread = Some(thread::spawn(move || {
            f(&pos, st)
        }));
    }

    pub fn is_ready(&self) -> bool {
        if let Some(thread_handle) = &self.search_thread {
            thread_handle.is_finished()
        }
        else {
            true
        }
    }

    pub fn get_searched_move(&mut self) -> (i32, Move) {
        if let Some(t) = self.search_thread.take() {
            return t.join().unwrap().clone();
        }

        panic!("no search thread in progress");
    }
}