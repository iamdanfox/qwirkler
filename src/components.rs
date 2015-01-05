use piece::{Piece, Bag, is_blank};
use piece;
use std::fmt;
use std::string;
use std::int;


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

  fn all() -> Vec<Direction> {
    return vec![Direction::U, Direction::D, Direction::L, Direction::R]
  }
}



pub struct Board {
  board: [[Piece; DIM_2]; DIM_2]
}

const DIM:int = 40;
const DIM_2:uint = (2*DIM) as uint;

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

impl Clone for Board {
  fn clone(&self) -> Board {
    Board { board: self.board }
  }
}

impl Board {
  pub fn new() -> Board {
    let blank = piece::blank();
    let mut new_board = [[blank; DIM_2]; DIM_2];
    // new_board[25][25] = 99;
    // new_board[26][26] = 11;
    Board { board: new_board }
  }

  fn get_bounding_box(&self) -> (int,int,int,int) {
    let mut min_x = int::MAX;
    let mut max_x = int::MIN;

    let mut min_y = int::MAX;
    let mut max_y = int::MIN;

    for y in range(-DIM,DIM) {
      for x in range(-DIM, DIM) {
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
    let all_directions = Direction::all();
    let mut result: Vec<(Square, Direction)> = Vec::new();
    let (min_x, max_x, min_y, max_y) = self.get_bounding_box();

    for y in range(min_y, max_y+1) {
      for x in range(min_x, max_x+1) {
        let square = (x,y);
        if !piece::is_blank(self.get(square)) {
          // we now know square is occupied
          for direction in all_directions.iter() {
            let adjacent_square = direction.apply(square);
            if piece::is_blank(self.get(adjacent_square)) {
              // we now know adjacent_square is in blank, on the 'perimeter'
              for d2 in all_directions.iter() {
                let target_square = d2.apply(adjacent_square);
                if piece::is_blank(self.get(target_square)) {
                  // now we know that d2 is safe to move in
                  result.push((adjacent_square,d2.clone()));
                }
              }
            };
          }
        };
      }
    }
    // TODO de-duplicate result???
    if result.len() == 0 {
      return vec![ ((0,0), Direction::R) ]
    } else {
      return result
    }
  }


  pub fn get(&self, sq:Square) -> Piece {
    let (x,y) = sq;
    return self.board[(x+DIM) as uint][(y+DIM) as uint];
  }

  pub fn allows_move(&self, m: &Move) -> bool {
    match *m {
      Move::SwapPieces => return true,
      Move::PlacePieces(start_sq, ref direction, ref pieces) => {

        // do a preliminary sanity check on `pieces`
        if !piece::valid_line(pieces) {
          return false
        }

        // check squares are empty
        let all_squares = direction.apply_all(start_sq, pieces.len());
        for sq in all_squares.iter() {
          if !piece::is_blank(self.get(*sq)) {
            return false;
          }
        }

        // do a full mainline check
        let mainline = self.get_mainline(start_sq, direction, pieces);
        if !piece::valid_line(&mainline) {
          return false;
        }

        // do all the perpendicular line checks
        let perps = self.get_all_perpendiculars(start_sq, direction, pieces);
        for line in perps.iter() {
          if !piece::valid_line(line) {
            return false
          }
        }

        return true
      },
    }
  }

  fn get_mainline(&self, start_sq:Square, direction:&Direction, pieces:&Vec<Piece>) -> Vec<Piece> {
    // compute mainline
    let mut mainline:Vec<Piece> = Vec::new();
    let before = self.pieces_in_direction(&direction.opposite(), start_sq);
    mainline.push_all(before.as_slice());
    mainline.push_all(pieces.as_slice());
    let last_square:Square = direction.apply_all(start_sq, pieces.len())[pieces.len()-1];
    let after = self.pieces_in_direction(direction, last_square);
    mainline.push_all(after.as_slice());
    return mainline;
  }

  fn get_all_perpendiculars(&self, start_sq:Square, direction:&Direction, pieces:&Vec<Piece>) -> Vec<Vec<Piece>> {
    let mut perp_lines: Vec<Vec<Piece>> = Vec::new();

    let all_squares = direction.apply_all(start_sq, pieces.len());
    for (square,piece) in all_squares.iter().zip(pieces.iter()) {
      let l = self.perp_line(direction, *square, *piece);
      perp_lines.push(l);
    }
    return perp_lines
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

  fn perp_line(&self, main_direction: &Direction, sq: Square, piece: Piece) -> Vec<Piece> {
    let (d1,d2) = main_direction.perpendiculars();
    let line1 = self.pieces_in_direction(&d1, sq);
    let line2 = self.pieces_in_direction(&d2, sq);
    let singleton:Vec<Piece> = vec![piece];
    return line1.into_iter().chain(singleton.into_iter()).chain(line2.into_iter()).collect();
  }

  pub fn compute_score(&self, start_sq: Square, direction: &Direction, pieces: &Vec<Piece>) -> int {
    let mut score = 0;
    let mainline = self.get_mainline(start_sq, direction, pieces);
    let perps = self.get_all_perpendiculars(start_sq, direction, pieces);
    for line in vec![mainline].iter().chain(perps.iter()) {
      if line.len() > 1 {
        score = score + line.len() + (if line.len() == 6 { 6 } else { 0 });
      }
    }
    return score as int
  }

  // Places pieces on the board and also returns the score for that move
  pub fn put(&self, start_sq: Square, direction: &Direction, pieces: &Vec<Piece>) -> (Board, int) {
    let mut new_board = self.board;

    let squares = direction.apply_all(start_sq, pieces.len());
    for (start_sq,piece) in squares.iter().zip(pieces.iter()) {
      let (x,y) = *start_sq;
      new_board[(x+DIM) as uint][(y+DIM) as uint] = *piece;
    }

    let score = self.compute_score(start_sq, direction, pieces);
    return (Board { board: new_board }, score)
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

