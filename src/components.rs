use piece::{Piece, Bag};


#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>)
}


#[derive(Show, Clone)]
pub struct PlayerState {
  pub bag: Bag,
  pub score: int,
}

impl PlayerState {
  pub fn new(bag: Bag) -> PlayerState {
    PlayerState { bag: bag, score: 0 }
  }
}


pub type Square = (uint,uint);


#[derive(Show, Clone)]
pub enum Direction {
  U, D, L, R
}


pub struct Board {
  board: [[uint; 50]; 50] // compatible with uint Piece
}

impl Board {
  pub fn new() -> Board {
    let new_board = [[0; 50]; 50];
    Board { board: new_board}
  }

  pub fn get_start_squares(&self) -> Vec<(Square, Direction)> {
    return vec![((0,0), Direction::R)]
  }

  pub fn allows_move(&self) -> bool {
    return true
  }

  // fn put(&self, square: Square, direction: Direction, pieces: Vec<Piece>) -> Board {
  //   let (x,y) = square;
  //   let mut new_board = self.board;

  //   new_board[x][y] = 99;

  //   Board { board: new_board }
  // }
}

impl Clone for Board {
  fn clone(&self) -> Board {
    Board { board: self.board }
  }
}
