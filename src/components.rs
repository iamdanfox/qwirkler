use piece;
use piece::Piece;
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

impl Board {
  pub fn new() -> Board {
    let blank = piece::blank();
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
        if piece::is_blank(self.get(direction.apply(sq))) {
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

  pub fn allows(&self, start_sq:Square, direction:&Direction, partial:&PartialScored) -> Option<(Score,Score)> {
    if !piece::is_blank(self.get(partial.last_square)) {
      return None
    }

    // TODO: build a LineValidator struct that stores first element, already_seen and length.
    // save a LineValidator in the partial so that we don't duplicate work on the mainline each time.

    // do a full mainline check
    let mainline = self.get_mainline(start_sq, direction, &partial.pieces);
    if !piece::valid_line(&mainline) {
      return None;
    }
    let new_mainline_score = mainline.len() + if mainline.len() == 6 { 6 } else { 0 };

    // since the prefix of this line was already passed validation,
    // we just need to check the last perpendicular.
    let last_piece = partial.pieces[partial.pieces.len()-1];
    let line = self.perp_line(direction, partial.last_square, last_piece);
    if !piece::valid_line(&line) { // TODO: introduce laziness! (we could return false without reading all)
      return None
    }
    let new_perp_score = if line.len() > 1 {
      line.len() + if line.len() == 6 { 6 } else { 0 }
    } else {
      0 // ensures we don't double count each piece!
    };

    return Some((new_mainline_score, new_perp_score));
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
      if piece::is_blank(self.get(sq)) {
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
