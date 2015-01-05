use gamestate::GameState;


mod piece;
mod components;
mod gamestate;


fn main() {
  let mut game_state = GameState::new(2);
  let mut i = 0u;

  loop {
    i = i + 1;
    println!("\n\n{}\n", game_state.board);
    println!("{}: player {} turn (score = {})", i, game_state.turn, game_state.players[game_state.turn].score);

    let mut moves = game_state.generate_moves();
    match moves.pop() {
      None => break,
      Some(ref chosen_move) => {
        game_state = game_state.apply_move(chosen_move);
      },
    }
  }

  // compute total score
  println!("\n\n\n\nGame finished.");
  let mut sum = 0;
  let mut i = 0;
  for player in game_state.players.iter() {
    println!("  player {} score={}", i, player.score);
    sum = sum + player.score;
    i += 1;
  }
  println!("\n  total = {}", sum);
}
