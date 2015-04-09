use piece::{Piece, Colour, Shape};

#[derive(Copy, Clone)]
pub struct LineValidator {
  seen_already:      [bool; 6],
  first_piece:       Piece,
  is_line_of_colour: Option<bool>,
  pub length:        usize,
}

impl LineValidator {
  pub fn new(first_piece: Piece) -> LineValidator {
    return LineValidator {
      seen_already:      [false; 6],
      first_piece:       first_piece,
      is_line_of_colour: None,
      length:            1,
    }
  }

  /// Clones and extends this validator if the new_piece is compatible, returns None otherwise
  pub fn clone_extend(&self, new_piece: Piece) -> Option<LineValidator> {
    if self.length == 6 || new_piece == self.first_piece {
      return None
    }

    match self.is_line_of_colour {
      None => {
        if self.first_piece.colour == new_piece.colour{
          let mut lv2 = *self;
          lv2.is_line_of_colour = Some(true);
          lv2.seen_already[self.first_piece.shape.index()] = true;
          lv2.seen_already[new_piece.shape.index()] = true;
          lv2.length += 1;
          return Some(lv2)
        } else if self.first_piece.shape == new_piece.shape {
          let mut lv2 = *self;
          lv2.is_line_of_colour = Some(false);
          lv2.seen_already[self.first_piece.colour.index()] = true;
          lv2.seen_already[new_piece.colour.index()] = true;
          lv2.length += 1;
          return Some(lv2)
        } else {
          return None
        }
      }
      Some(loc) => {
        if loc {
          if self.first_piece.colour != new_piece.colour || self.seen_already[new_piece.shape.index()] {
            return None
          }
          let mut lv2 = *self;
          lv2.seen_already[new_piece.shape.index()] = true;
          lv2.length += 1;
          return Some(lv2)
        } else {
          if self.first_piece.shape != new_piece.shape || self.seen_already[new_piece.colour.index()] {
            return None
          }
          let mut lv2 = *self;
          lv2.seen_already[new_piece.colour.index()] = true;
          lv2.length += 1;
          return Some(lv2)
        }
      }
    }
  }

  /// prevents any more extensions
  pub fn seal(&mut self) {
    self.length = 6;
  }

  pub fn add_piece(&mut self, new_piece: Piece) -> bool {
    match self.is_line_of_colour {
      None => {
        if self.first_piece.colour == new_piece.colour {
          if self.length == 6 || new_piece == self.first_piece {
            return false
          }
          self.is_line_of_colour = Some(true);
          self.seen_already[self.first_piece.shape.index()] = true;
          self.seen_already[new_piece.shape.index()] = true;
          self.length = self.length + 1;
          return true
        } else if self.first_piece.shape == new_piece.shape {
          if self.length == 6 || new_piece == self.first_piece {
            return false
          }
          self.is_line_of_colour = Some(false);
          self.seen_already[self.first_piece.colour.index()] = true;
          self.seen_already[new_piece.colour.index()] = true;
          self.length = self.length + 1;
          return true
        } else {
          return false
        }
      },
      Some(loc) => {
        if loc {
          if self.first_piece.colour != new_piece.colour || self.seen_already[new_piece.shape.index()] {
            return false
          }
          if self.length == 6 || new_piece == self.first_piece {
            return false
          }
          self.seen_already[new_piece.shape.index()] = true;
        } else {
          if self.first_piece.shape != new_piece.shape || self.seen_already[new_piece.colour.index()] {
            return false
          }
          if self.length == 6 || new_piece == self.first_piece {
            return false
          }
          self.seen_already[new_piece.colour.index()] = true;
        }
        self.length = self.length + 1;
        return true
      }
    }
  }

  pub fn extend_from_iter<'a, T: Iterator<Item=Piece>>(&mut self, iter: &mut T) -> bool{
    loop {
      match iter.next() {
        None => return true,
        Some(p) =>  {
          if !self.add_piece(p) {
            return false
          }
        }
      }
    }
  }
}

#[test]
fn test_length1() {
  let ra = Piece::new(Colour::R, Shape::A);
  let lv = LineValidator::new(ra);
  assert!(lv.length == 1);
}

#[test]
fn test_add_same_colour() {
  let ra = Piece::new(Colour::R, Shape::A);
  let rb = Piece::new(Colour::R, Shape::B);
  let mut lv = LineValidator::new(ra);
  assert!(lv.add_piece(rb));
}

#[test]
fn test_add_same_shape() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let p2 = Piece::new(Colour::G, Shape::A);
  let mut lv = LineValidator::new(p1);
  assert!(lv.add_piece(p2));
}

#[test]
fn test_add_identical_fail() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let mut lv = LineValidator::new(p1);
  assert_eq!(lv.add_piece(p1), false);
}

#[test]
fn test_duplicate_first() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let p2 = Piece::new(Colour::G, Shape::A);
  let mut lv = LineValidator::new(p1);
  lv.add_piece(p2);
  assert_eq!(lv.add_piece(p1), false);
}

#[test]
fn test_change_common_feature() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let p2 = Piece::new(Colour::G, Shape::A);
  let p3 = Piece::new(Colour::G, Shape::B);
  let mut lv = LineValidator::new(p1);
  lv.add_piece(p2);
  assert_eq!(lv.add_piece(p3), false);
}