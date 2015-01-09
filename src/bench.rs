extern crate test;
use gamestate::GameState;

#[bench]
fn entire_game(b: &mut test::Bencher) {
  b.iter(|| {
    let mut game_state = GameState::new(2);
    loop {
      match game_state.generate_best_move() {
        None => break,
        Some(chosen_move) => {
          game_state.apply_move(&chosen_move);
        },
      }
    }
    println!("{}, total = {}", game_state.players.iter().map(|s| s.score).collect::<Vec<uint>>(), game_state.total_score());
  })
}
