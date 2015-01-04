use std::iter::Range;

// TODO make these real types
type Board = int;
type PlayerState = int;
type Bag = int;
type Move = int;


#[deriving(Show)]
struct GameState {
  board: Board,
  players: Vec<PlayerState>,
  bag: Bag,
  turn: uint,
}


impl GameState {
  // kinda static constructor
  fn new() -> GameState {
    GameState { board: 0, players: vec![1,2], bag: 30, turn: 0 }
  }

  // instance method
  fn generate_moves(&self) -> Vec<Move> {
    range(0, self.bag).collect()
  }

  fn apply_move(&self, chosenMove: Move) -> GameState {
    GameState {
      board: self.board,
      players: self.players.clone(),
      bag: self.bag,
      turn: (self.turn + 1) % self.players.len()
    }
  }
}



fn main() {
  println!("Hello, world!");

  let mut gameState = GameState::new();

  loop {
    let moves = gameState.generate_moves();
    if moves.len() > 0 {
      println!("player {} turn", gameState.turn);
      let chosenMove = moves[0]; // choses first move
      gameState = gameState.apply_move(chosenMove);
    } else {
      break
    }
  }


}
