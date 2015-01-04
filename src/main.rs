// imports all the `pub` stuff from piece.rs
mod piece;


// TODO make these real types
type Board = int;
type Bag = int;
type Move = int;



#[derive(Show, Clone)]
struct PlayerState {
  bag: Vec<piece::Piece>,
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
    let initial_bag = piece::make_bag();
    println!("{}", initial_bag);

    GameState {
      board: 0,
      players: range(0, num_players).map(|_| PlayerState::new()).collect(),
      bag: 30,
      turn: 0,
    }
  }

  // instance method
  fn generate_moves(&self) -> Vec<Move> {

    // initialise moves = []

    // figure out possible start squares (and directions).
    // FOR EACH POSSIBLE START CONFIG:
      // iterate upwards through permutations and combinations of current players' bag (using a Queue)
        // ie start with [p1], [p2], [p3] then if [p1] works try [p1,px], [p1,py], [p1,pz]...
        // if the board allows the move, add it to our list of moves

    // return moves ( maybe + SwapPieces)

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
