use piece::{Piece, Bag, is_blank};
use piece;
use std::collections::RingBuf;



#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, RingBuf<Piece>)
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
  U, D, L, R // these are causing lots of pain. maybe switch them to 1,2,3,4??
}

impl Direction {
  fn apply(&self, sq: Square) -> Square {
    let (x,y) = sq;
    match *self {
      Direction::U => (x, y+1),
      Direction::D => (x, y-1),
      Direction::L => (x-1, y),
      Direction::R => (x+1, y),
    }
  }

  fn apply_all(&self, sq: Square, len: uint) -> Vec<Square> {
    let mut squares = vec![];
    let mut last = sq;
    for _ in range(0, len) {
      squares.push(last);
      last = self.apply(last);
    }
    return squares
  }
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

    // TODO generate the actual perimeter and check all directions

    return vec![((0,0), Direction::R)]
  }

  fn get(&self, sq:Square) -> Piece {
    let (x,y) = sq;
    return self.board[x+25][y+25];
  }

  pub fn allows_move(&self, m: &Move) -> bool {
    match *m {
      Move::SwapPieces => return true,
      Move::PlacePieces(start_sq, ref direction, ref pieces) => {
        let all_squares = direction.apply_all(start_sq, pieces.len());
        for sq in all_squares.iter() {
          if !piece::is_blank(self.get(*sq)) {
            return false;
          }
        }

        // for every newly formed line:
        for line in self.get_newly_formed_lines(start_sq, direction, pieces).iter() {
          // pieces must form a line of one color/shape
          // use `piece::all_unique`
          if !piece::all_unique(line) {
            return false
          }

          // no repeated pieces allowed
          if !piece::all_same_colour(line) && !piece::all_same_shape(line) {
            return false
          }
        }

        return true
      },
    }
  }

  fn get_newly_formed_lines(&self, start_sq:Square, direction:&Direction, pieces:&RingBuf<Piece>) -> Vec<RingBuf<Piece>> {

    // TODO compute : mainline ++ perpendicular lines

    return vec![(*pieces).clone()]
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
