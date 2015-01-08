use piece::{Piece};
use linevalidator::LineValidator;
use direction::{Square, Direction};
use std::{fmt, string};
use player::Score;
use partial::PartialScored;
use std::collections::HashSet;


#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>)
}

pub struct Board {
  board: [[Piece; DIM_2]; DIM_2],
  perimeter: HashSet<Square>,
  min_x: int,
  max_x: int,
  min_y: int,
  max_y: int,
}

const DIM:int = 25;
const DIM_2:uint = (2*DIM) as uint;

impl fmt::Show for Board {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mut output = string::String::new();

    for y in range(self.min_y - 1, self.max_y + 2) {
      for x in range(self.min_x - 1, self.max_x + 2) {
        let piece = self.get((x,y));
        if !piece.is_blank() {
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

impl Board {
  pub fn new() -> Board {
    let blank = Piece::blank();
    let new_board = [[blank; DIM_2]; DIM_2];
    Board {
      board: new_board,
      perimeter: HashSet::new(),
      min_x: 0,
      max_x: 0,
      min_y: 0,
      max_y: 0,
    }
  }

  pub fn get_start_squares(&self) -> Vec<(Square, Direction)> {
    let mut result: Vec<(Square, Direction)> = Vec::new();

    for direction in Direction::all().iter() {
      for &sq in self.perimeter.iter() {
        if self.get(direction.apply(sq)).is_blank() {
          result.push((sq,direction.clone()));
        }
      }
    }
    if result.len() == 0 {
      return vec![ ((0,0), Direction::initial()) ]
    } else {
      return result
    }
  }

  pub fn get(&self, (x,y):Square) -> Piece {
    return self.board[(x+DIM) as uint][(y+DIM) as uint];
  }

  pub fn allows(&self, start_sq:Square, direction:&Direction, partial:&mut PartialScored) -> Option<(Score,Score)> {
    if !self.get(partial.last_square).is_blank() {
      return None
    }
    let last_piece = partial.pieces[partial.pieces.len()-1];

    // only do full mainline check once:
    let mut new_mainline_score;
    let new_validator:Option<LineValidator> = match partial.main_validator.as_mut() {
      None => {
        // since this is a singleton, we must build the validator
        let mut lv = LineValidator::new(partial.pieces[0]);

        // check befores and afters
        let mut count = 1;
        for p in self.pieces_in_direction2((*direction).clone(), start_sq).chain(self.pieces_in_direction2(direction.opposite(), start_sq)) {
          if !lv.accepts(p) {
            return None
          }
          count += 1;
        }
        new_mainline_score = count + if count == 6 { 6 } else { 0 };
        Some(lv)
      },
      Some(lv) => {
        let after_last_square = direction.apply(partial.last_square);
        if self.get(after_last_square).is_blank() {
          // blank space at the end - re-use the old LineValidator
          if !lv.accepts(last_piece) { // also updates lv.
            return None
          }
        } else {
          // otherwise, extend the mainline...
          let mut curr_square = after_last_square;
          let mut curr_piece = self.get(curr_square);
          while !curr_piece.is_blank() {
            if !lv.accepts(curr_piece) {
              return None
            } else {
              curr_square = direction.apply(curr_square);
              curr_piece = self.get(curr_square);
            }
          }
        }
        new_mainline_score = lv.length + if lv.length == 6 { 6 } else { 0 };
        None
      }
    };

    if !new_validator.is_none() {
      partial.main_validator = new_validator;
    }

    // since the prefix of this line was already passed validation,
    // we just need to check the last perpendicular.
    let mut perp_size = 1;
    let mut perp_lv = LineValidator::new(last_piece);
    let (d1,d2) = direction.perpendiculars();
    for p in self.pieces_in_direction2(d1, partial.last_square).chain(self.pieces_in_direction2(d2, partial.last_square)) {
      if !perp_lv.accepts(p) {
        return None
      }
      perp_size += 1;
    };

    let new_perp_score = if perp_size > 1 {
      perp_size + if perp_size == 6 { 6 } else { 0 }
    } else {
      0 // ensures we don't double count each piece!
    };

    return Some((new_mainline_score, new_perp_score));
  }

  fn pieces_in_direction2(&self, direction: Direction, start: Square) -> NonBlankIterator {
    return NonBlankIterator {
      sq: start,
      direction: direction,
      board: self
    }
  }

  pub fn put(&mut self, start_sq: Square, direction: &Direction, pieces: &Vec<Piece>) {
    // compute the new array
    let squares = direction.apply_all(start_sq, pieces.len());
    for (&(x,y),&piece) in squares.iter().zip(pieces.iter()) {
      self.board[(x+DIM) as uint][(y+DIM) as uint] = piece;
    }

    // compute the new perimeter
    for sq in squares.iter() {
      self.perimeter.remove(sq);
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
      if self.get(sq).is_blank() {
        self.perimeter.insert(sq);
      }
    }

    // update the bounding box.
    self.stretch_bounding_box(squares[0]);
    self.stretch_bounding_box(squares[pieces.len()-1]);
  }

  fn stretch_bounding_box(&mut self, (x,y): Square) {
    if x < self.min_x { self.min_x = x; };
    if x > self.max_x { self.max_x = x; };
    if y < self.min_y { self.min_y = y; };
    if y > self.max_y { self.max_y = y; };
  }
}


struct NonBlankIterator<'a> {
  sq: Square,
  direction: Direction,
  board: &'a Board
}

impl<'a> Iterator for NonBlankIterator<'a> {
  type Item = Piece;

  fn next(&mut self) -> Option<Piece> {
    let next_sq = self.direction.apply(self.sq);
    let contents = self.board.get(next_sq);
    if contents.is_blank() {
      return None
    } else {
      self.sq = next_sq;
      return Some(contents)
    }
  }
}
