use piece::{Piece};
use linevalidator::LineValidator;
use direction::{Square, Direction};
use std::{fmt, string};
use player::Score;
use partial::Partial;
use std::collections::HashSet;


/// The representation of an arrangement of Qwirkle pieces.
/// Designed to be mutated.  Only one `Board` needed for a game of qwirkle.
///
/// # Redundancy
///
/// Technically, the `board` array uniquely defines the arrangement of Qwirkle pieces.
/// Nonetheless, the `perimeter` and bounding information is stored here so that it doesn't
/// have to be recomputed for every move.
///
/// # Invariants
///
/// * The `perimeter` hashset stores free locations that are adjacent to an occupied square
/// * The `min_x`, `max_x` etc variables define a bounding box for the whole game's arrangement
/// of pieces. (Coordinates are inclusive)
pub struct Board {
  board:     [[Option<Piece>; DIM_2]; DIM_2],
  perimeter: HashSet<Square>,
  min_x:     isize,
  max_x:     isize,
  min_y:     isize,
  max_y:     isize,
}

/// This constant determines the size of array we allocate for one `Board`.  Theoretically, you could
/// easily overflow this, but in practise, 25 squares in any direction seems to work.
const DIM:isize = 25;
const DIM_2:usize = (2*DIM) as usize;

impl Board {
  pub fn new() -> Board {
    let new_board = [[None; DIM_2]; DIM_2];
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

    for &direction in &Direction::all() {
      for &sq in &self.perimeter {
        if self.get(direction.apply(sq)).is_none() {
          result.push((sq,direction));
        }
      }
    }
    if result.len() == 0 {
      return vec![ ((0,0), Direction::initial()) ]
    } else {
      return result
    }
  }

  /// allows() returns `true` iff the supplied `partial` play is legal for this board.
  ///
  /// Preconditions:
  ///  * The last square that `partial` would write into must be empty
  ///  * The prefix of partial (ie everything except the last piece) must have already been validated
  ///
  /// Postconditions:
  /// Mutates the argument `partial` by populating the `perp_scores` and `mainline_scores` fields and `main_validator` if necessary.
  pub fn allows(&self, partial:&mut Partial) -> bool {
    // assert!(self.get(partial.last_square).is_none()); // this is true because main_validator is sealed if this square is non-empty

    // since the prefix of this line was already passed validation,
    // we just need to check the last perpendicular.
    let last_piece = partial.pieces[partial.pieces.len()-1];
    let new_perp_score = match self.check_perpendicular(partial.last_square, &partial.direction, last_piece) {
      None => return false,
      Some(v) => v
    };

    // if we have no line validator, we must construct one and save it
    if partial.main_validator.is_none() {
      let mut first_lv = LineValidator::new(partial.pieces[0]);
      // TODO: maybe re-express this whole pattern as a fold?
      if !first_lv.extend_from_iter(&mut self.non_blank_iter(partial.start_square, partial.direction.opposite())) {
        return false
      }
      partial.main_validator = Some(first_lv)
    }

    if let Some(ref mut lv) = partial.main_validator.as_mut() {
      if !self.get(partial.direction.apply(partial.last_square)).is_none() {
        // the line doesn't end in a blank, we have to continue validating in that direction
        if !lv.extend_from_iter(&mut self.non_blank_iter(partial.last_square, partial.direction)) {
          return false
        }
        lv.seal() // this also implies that this partial can't be extended
      }
      partial.mainline_score = lv.length + if lv.length == 6 { 6 } else { 0 };
    }

    partial.perp_scores += new_perp_score;
    return true;
  }

  // returns the score won from a perpendicular line or return None if it's invalid.
  fn check_perpendicular(&self, square:Square, direction:&Direction, piece:Piece) -> Option<Score> {
    let mut perp_lv = LineValidator::new(piece);
    let (d1,d2) = direction.perpendiculars();
    if !perp_lv.extend_from_iter(&mut self.non_blank_iter(square, d1).chain(self.non_blank_iter(square, d2))) {
      return None
    }
    if perp_lv.length > 1 {
      return Some(perp_lv.length + if perp_lv.length == 6 { 6 } else { 0 });
    } else {
      return Some(0) // ensures we don't double count each piece!
    }
  }

  fn non_blank_iter(&self, start: Square, direction: Direction) -> NonBlankIterator {
    return NonBlankIterator {
      sq:        start,
      direction: direction,
      board:     self,
    }
  }

  /// Beginning at `start_sq`, place `pieces` onto the board in the specified `direction`.
  /// Preconditions:
  /// The start_sq, direction and pieces must describe a legal move according to the rules of qwirkle.
  pub fn put(&mut self, start_sq: Square, direction: &Direction, pieces: &Vec<Piece>) {
    // compute the new array
    let squares = direction.apply_all(start_sq, pieces.len());
    for (&(x,y),&piece) in squares.iter().zip(pieces.iter()) {
      self.board[(x+DIM) as usize][(y+DIM) as usize] = Some(piece);
    }

    // compute the new perimeter
    for sq in &squares {
      self.perimeter.remove(sq);
    }

    let mut candidates = Vec::new();
    if pieces.len() < 6 {
      candidates.push(direction.apply(squares[pieces.len()-1]));
    }
    candidates.push(direction.opposite().apply(start_sq));
    let (d1,d2) = direction.perpendiculars();
    for &sq in &squares {
      candidates.push(d1.apply(sq));
      candidates.push(d2.apply(sq));
    }

    for &sq in &candidates {
      if self.get(sq).is_none() {
        self.perimeter.insert(sq);
      }
    }

    // update the bounding box.
    self.stretch_bounding_box(squares[0]);
    self.stretch_bounding_box(squares[pieces.len()-1]);
  }

  pub fn get(&self, (x,y):Square) -> Option<Piece> {
    return self.board[(x+DIM) as usize][(y+DIM) as usize];
  }

  fn stretch_bounding_box(&mut self, (x,y): Square) {
    if x < self.min_x { self.min_x = x; };
    if x > self.max_x { self.max_x = x; };
    if y < self.min_y { self.min_y = y; };
    if y > self.max_y { self.max_y = y; };
  }
}

impl fmt::Display for Board {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mut output = string::String::new();

    for y in (self.min_y - 1 .. self.max_y + 2) {
      for x in (self.min_x - 1 .. self.max_x + 2) {
        match self.get((x,y)) {
          None => output.push_str(".."),
          Some(p) => output.push_str(p.to_string().as_ref()),
        }
        output.push_str(" ");
      }
      output.push_str("\n");
    }
    output.fmt(formatter)
  }
}


/// Lazily lookup pieces from the board
///
/// From a specified start square and direction, this iterator will advance one square in the direction
/// and return the board's contents (either `Some(piece)` or `None`).
///
/// Note, the iterator will not return the contents of the start square
pub struct NonBlankIterator<'a> {
  sq:        Square,
  direction: Direction,
  board:     &'a Board
}

impl<'a> Iterator for NonBlankIterator<'a> {
  type Item = Piece;

  fn next(&mut self) -> Option<Piece> {
    self.sq = self.direction.apply(self.sq);
    return self.board.get(self.sq);
  }
}
