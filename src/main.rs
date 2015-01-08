use gamestate::GameState;
// use components::Move;
// use player::Score;

mod piece;
mod components;
mod gamestate;
mod direction;
mod player;
mod partial;
mod bench;
mod linevalidator;


fn main() {
  let mut game_state = GameState::new(2);
  loop {
    let best = game_state.generate_best_move();
    match best {
      None => break,
      Some((score, chosen_move)) => {
        game_state.apply_move(&chosen_move, score);
      },
    }
  }

  println!("\n\n\n\nGame finished, total score = {}", game_state.total_score());
  println!("{}", game_state.board);
}
