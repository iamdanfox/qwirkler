use piece::{Piece, Bag, is_blank};
use piece;
use std::fmt;
use std::string;
use std::int;
use std::str;


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


pub type Square = (int,int);


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

  fn opposite(&self) -> Direction {
    match *self {
      Direction::U => Direction::D,
      Direction::D => Direction::U,
      Direction::L => Direction::R,
      Direction::R => Direction::L,
    }
  }

  fn perpendiculars(&self) -> (Direction,Direction) {
    match *self {
      Direction::U | Direction::D => (Direction::L, Direction::R),
      Direction::L | Direction::R => (Direction::U, Direction::D),
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
  board: [[Piece; 50]; 50]
}

impl fmt::Show for Board {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mut output: string::String = string::String::new();
    let (min_x, max_x, min_y, max_y) = self.get_bounding_box();

    for y in range(min_y - 1, max_y + 2) {
      for x in range(min_x - 1, max_x + 2) {
        let piece = self.get((x,y));
        if !piece::is_blank(piece) {
          output.push_str(piece.to_string().as_slice());
        } else {
          output.push_str("--");
        }
        output.push_str(" ");
      }
      output.push_str("\n");
    }
    output.fmt(formatter)
  }
}

impl Board {
  pub fn new() -> Board {
    let blank = piece::blank();
    let mut new_board = [[blank; 50]; 50];
    // new_board[26][26] = 11;
    // new_board[24][24] = 11;
    Board { board: new_board }
  }

  fn get_bounding_box(&self) -> (int,int,int,int) {
    let mut min_x = int::MAX;
    let mut max_x = int::MIN;

    let mut min_y = int::MAX;
    let mut max_y = int::MIN;

    for j in range(0,50) {
      for i in range(0, 50) {
        let (x,y) = (i-25, j-25);
        if !piece::is_blank(self.get((x,y))) {
          if x < min_x { min_x = x; };
          if x > max_x { max_x = x; };
          if y < min_y { min_y = y; };
          if y > max_y { max_y = y; };
        }
      }
    }
    if min_x == int::MAX { min_x = 0 };
    if max_x == int::MIN { max_x = 0 };
    if min_y == int::MAX { min_y = 0 };
    if max_y == int::MIN { max_y = 0 };
    return (min_x, max_x, min_y, max_y)
  }

  pub fn get_start_squares(&self) -> Vec<(Square, Direction)> {

    // TODO generate the actual perimeter and check all directions

    return vec![((0,0), Direction::R)]
  }

  pub fn get(&self, sq:Square) -> Piece {
    let (x,y) = sq;
    return self.board[(x+25) as uint][(y+25) as uint];
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

  fn get_newly_formed_lines(&self, start_sq:Square, direction:&Direction, pieces:&Vec<Piece>) -> Vec<Vec<Piece>> {

    // compute mainline
    let mut mainline:Vec<Piece> = Vec::new();
    let last_square:Square = direction.apply_all(start_sq, pieces.len())[pieces.len()-1];
    let before = self.pieces_in_direction(&direction.opposite(), start_sq);
    mainline.push_all(before.as_slice());
    for piece in (*pieces).iter() { // replace with push_all(...as_slice)
      mainline.push(*piece);
    }
    let after = self.pieces_in_direction(direction, last_square);
    mainline.push_all(after.as_slice());


    // TODO compute : mainline ++ perpendicular lines.  Use .chain to join

    return vec![]
  }

  fn pieces_in_direction(&self, direction: &Direction, start: Square) -> Vec<Piece> {
    let mut sq = direction.apply(start);
    let mut pieces = vec![];
    while !piece::is_blank(self.get(sq)) {
      pieces.push(self.get(sq));
      sq = direction.apply(sq);
    }
    return pieces;
  }

  fn perp_line(&self, main_direction: Direction, sq: Square, piece: Piece) -> Vec<Piece> {
    let (d1,d2) = main_direction.perpendiculars();
    let line1 = self.pieces_in_direction(&d1, sq);
    let line2 = self.pieces_in_direction(&d2, sq);
    let singleton:Vec<Piece> = vec![piece];
    return line1.into_iter().chain(singleton.into_iter()).chain(line2.into_iter()).collect();
  }

  fn put(&self, square: Square, direction: Direction, pieces: Vec<Piece>) -> Board {
    let (x,y) = square;
    let mut new_board = self.board;

    new_board[(x+25) as uint][(y+25) as uint] = 99;

    Board { board: new_board }
  }

}


// struct NonEmptyCellIterator<'a> {
//   board: &'a Board,
//   direction: Direction,
//   sq: Square
// }

// impl<'a, Iterator<Piece>> Iterator<Piece> for NonEmptyCellIterator<'a> {
//   fn next(&mut self) -> Option<Piece> {
//     let current_piece = (*self.board).get(self.sq);
//     if piece::is_blank(current_piece) {
//       return None;
//     } else {
//       self.sq = self.direction.apply(self.sq);
//       return Some(current_piece);
//     }
//   }
// }


impl Clone for Board {
  fn clone(&self) -> Board {
    Board { board: self.board }
  }
}
