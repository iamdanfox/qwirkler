// Unstable library features:
#![feature(collections)]
#![feature(test)]
#![feature(convert)]

extern crate rand;

use gamestate::GameState;

mod piece;
mod board;
mod gamestate;
mod direction;
mod player;
mod partial;
mod bench;
mod linevalidator;
mod mv;

fn main() {
  let mut game_state = GameState::new(2);
  loop {
    match game_state.generate_best_move() {
      None => break,
      Some(chosen_move) => {
        game_state.apply_move(&chosen_move);
      },
    }
  }

  println!("{}", game_state.board);
  println!("Game finished, total score = {}\n", game_state.total_score());
}
