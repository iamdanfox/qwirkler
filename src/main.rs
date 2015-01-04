use board::{Board, Square, Direction};
use piece::{Bag, Piece};

// imports all the `pub` stuff from piece.rs
mod piece;
mod board;




fn main() {

  let mut game_state = GameState::new(2);

  loop {
    let mut moves = game_state.generate_moves();
    match moves.pop() {
      None => break,
      Some(chosen_move) => {
        println!("player {} turn", game_state.turn);
        game_state = game_state.apply_move(chosen_move);
      },
    }
  }

  println!("Game finished.")

}



#[derive(Show, Clone)]
struct PlayerState {
  bag: Bag,
  score: int,
}

impl PlayerState {
  fn new() -> PlayerState {
    PlayerState { bag: vec![], score: 0 }
  }
}



// #[derive(Show)]
struct GameState {
  board: Board,
  players: Vec<PlayerState>,
  bag: Bag,
  turn: uint,
}



impl GameState {
  // factory method
  fn new(num_players: int) -> GameState {
    let initial_bag = piece::make_bag();

    GameState {
      board: Board::new(),
      players: range(0, num_players).map(|_| PlayerState::new()).collect(),
      bag: initial_bag,
      turn: 0,
    }
  }

  // instance method
  fn generate_moves(&self) -> Vec<Move> {

    let mut moves:Vec<Move> = Vec::new();

    // figure out possible start squares (and directions).
    // FOR EACH POSSIBLE START CONFIG:
      // iterate upwards through permutations and combinations of current players' bag (using a Queue)
        // ie start with [p1], [p2], [p3] then if [p1] works try [p1,px], [p1,py], [p1,pz]...
        // if the board allows the move, add it to our list of moves


    if self.bag.len() > 0 {
      moves.push(Move::SwapPieces)
    }

    return moves
  }

  fn apply_move(&self, chosen_move: Move) -> GameState {
    let mut new_bag = self.bag.clone();
    new_bag.pop();
    println!("chosen move {}", chosen_move);

    // TODO: real code


    GameState {
      board: self.board.clone(),
      players: self.players.clone(),
      bag: new_bag,
      turn: (self.turn + 1) % self.players.len()
    }
  }
}

#[derive(Show)]
enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>)
}