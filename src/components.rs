use piece;
use piece::Piece;
use direction::{Square, Direction};
use std::{fmt, int, string};
use player::Score;
use std::collections::HashSet;


#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>)
}


pub struct Board {
  board: [[Piece; DIM_2]; DIM_2],
  perimeter: HashSet<Square>
}

const DIM:int = 25;
const DIM_2:uint = (2*DIM) as uint;

impl fmt::Show for Board {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mut output = string::String::new();
    let (min_x, max_x, min_y, max_y) = self.get_bounding_box();

    for y in range(min_y - 1, max_y + 2) {
      for x in range(min_x - 1, max_x + 2) {
        let piece = self.get((x,y));
        if !piece::is_blank(piece) {
          output.push_str(piece.to_string().as_slice());
        } else {
          output.push_str("..");
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
    Board {
      board: self.board,
      perimeter: self.perimeter.clone()
    }
  }
}

impl Board {
  pub fn new() -> Board {
    let blank = piece::blank();
    let new_board = [[blank; DIM_2]; DIM_2];
    Board {
      board: new_board,
      perimeter: HashSet::new()
    }
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
    let mut result: Vec<(Square, Direction)> = Vec::new();

    for direction in Direction::all().iter() {
      for &sq in self.perimeter.iter() {
        if piece::is_blank(self.get(direction.apply(sq))) {
          result.push((sq,direction.clone()));
        }
      }
    }
    // TODO de-duplicate result???
    if result.len() == 0 {
      return vec![ ((0,0), Direction::initial()) ]
    } else {
      return result
    }
  }

  pub fn get(&self, (x,y):Square) -> Piece {
    return self.board[(x+DIM) as uint][(y+DIM) as uint];
  }

  pub fn allows_move(&self, m: &Move) -> bool {
    match *m {
      Move::SwapPieces => return true,
      Move::PlacePieces(start_sq, ref direction, ref pieces) => {
        let all_squares = direction.apply_all(start_sq, pieces.len());
        let last_square:Square = all_squares[pieces.len() - 1];

        // check the last piece is laid onto an empty square.
        if !piece::is_blank(self.get(last_square)) {
          return false
        }

        // do a preliminary sanity check on `pieces`
        if !piece::valid_line(pieces) {
          return false
        }

        // since the prefix of this line was already passed validation,
        // we just need to check the last perpendicular.
        let last_piece:Piece = pieces[pieces.len()-1];
        let line = self.perp_line(direction, last_square, last_piece);
        if !piece::valid_line(&line) {
          return false
        }

        // do a full mainline check
        let mainline = self.get_mainline(start_sq, direction, pieces);
        if !piece::valid_line(&mainline) {
          return false;
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
    let mut result = Vec::new();
    let (d1,d2) = main_direction.perpendiculars();
    result.push_all(self.pieces_in_direction(&d1, sq).as_slice());
    result.push(piece);
    result.push_all(self.pieces_in_direction(&d2, sq).as_slice());
    return result;
  }

  pub fn score_move(&self, mv: &Move) -> Score {
    match mv {
      &Move::SwapPieces => 0,
      &Move::PlacePieces(start_sq, ref direction, ref pieces) => {
        return self.compute_score(start_sq, direction, pieces);
      }
    }
  }

  pub fn compute_score(&self, start_sq: Square, direction: &Direction, pieces: &Vec<Piece>) -> Score {
    let mut score = 0;
    let mainline = self.get_mainline(start_sq, direction, pieces);
    let perps = self.get_all_perpendiculars(start_sq, direction, pieces);
    for line in vec![mainline].iter().chain(perps.iter()).filter(|line| line.len() > 1) {
      score = score + line.len() + (if line.len() == 6 { 6 } else { 0 });
    }
    return score
  }

  pub fn put(&self, start_sq: Square, direction: &Direction, pieces: &Vec<Piece>) -> (Board, Score) {
    // compute the new array
    let mut new_array = self.board;
    let squares = direction.apply_all(start_sq, pieces.len());
    for (&(x,y),&piece) in squares.iter().zip(pieces.iter()) {
      new_array[(x+DIM) as uint][(y+DIM) as uint] = piece;
    }

    // compute the new perimeter
    let mut new_perimeter = self.perimeter.clone();
    for sq in squares.iter() {
      new_perimeter.remove(sq);
    }

    let mut candidates = Vec::new();
    candidates.push(direction.apply(squares[pieces.len()-1]));
    candidates.push(direction.opposite().apply(start_sq));
    let (d1,d2) = direction.perpendiculars();
    for &sq in squares.iter() {
      candidates.push(d1.apply(sq));
      candidates.push(d2.apply(sq));
    }

    for &sq in candidates.iter() {
      if piece::is_blank(self.get(sq)) {
        new_perimeter.insert(sq);
      }
    }

    let new_board = Board {
      board: new_array,
      perimeter: new_perimeter,
    };
    let score = self.compute_score(start_sq, direction, pieces);
    return (new_board, score)
  }

}
