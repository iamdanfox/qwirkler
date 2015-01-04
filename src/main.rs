// TODO make these real types
type Board = int;
type Bag = int;
type Move = int;
type Piece = int;


#[derive(Show, Clone)]
struct PlayerState {
  bag: Vec<Piece>,
  score: int,
}

impl PlayerState {
  fn new() -> PlayerState {
    PlayerState { bag: vec![], score: 0 }
  }
}


#[derive(Show)]
struct GameState {
  board: Board,
  players: Vec<PlayerState>, // TODO make this an array?
  bag: Bag,
  turn: uint,
}


impl GameState {
  // factory method
  fn new(num_players: int) -> GameState {
    GameState {
      board: 0,
      players: range(0, num_players).map(|_| PlayerState::new()).collect(),
      bag: 30,
      turn: 0,
    }
  }

  // instance method
  fn generate_moves(&self) -> Vec<Move> {
    range(0, self.bag).collect()
  }

  fn apply_move(&self, chosen_move: Move) -> GameState {
    GameState {
      board: self.board,
      players: self.players.clone(),
      bag: self.bag - 1,
      turn: (self.turn + 1) % self.players.len()
    }
  }
}



fn main() {

  let mut game_state = GameState::new(2);

  loop {
    let moves = game_state.generate_moves();
    if moves.len() > 0 {
      println!("player {} turn", game_state.turn);
      let chosen_move = moves[0]; // choses first move
      game_state = game_state.apply_move(chosen_move);
    } else {
      break
    }
  }

  println!("Game finished.")

}
