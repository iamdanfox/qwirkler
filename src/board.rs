
// #[derive(Show, Clone)]
pub struct Board {
  dim: int,
  board: [[uint; 50]; 50] // compatible with uint Piece
}


impl Board {
  pub fn new() -> Board {
    let new_board = [[0; 50]; 50];
    Board { dim: 0 , board: new_board}
  }
}


impl Clone for Board {
  fn clone(&self) -> Board {
    Board { dim: self.dim, board: self.board }
  }
}