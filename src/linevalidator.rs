use piece::{Piece, Colour, Shape};


pub struct LineValidator {
  seen_already: [bool; 6],
  first_piece: Piece,
  is_line_of_colour: Option<bool>,
  pub length: usize,
}

impl Clone for LineValidator {
  fn clone(&self) -> LineValidator {
    return LineValidator {
      seen_already: self.seen_already,
      first_piece: self.first_piece,
      is_line_of_colour: self.is_line_of_colour,
      length: self.length,
    }
  }
}

impl LineValidator {
  pub fn new(first_piece: Piece) -> LineValidator {
    return LineValidator {
      seen_already: [false; 6],
      first_piece: first_piece,
      is_line_of_colour: None,
      length: 1,
    }
  }

  pub fn can_add(&self, new_piece: Piece) -> bool {
    if self.length == 6 || new_piece == self.first_piece {
      return false
    }

    match self.is_line_of_colour {
      None => return self.first_piece.compatible_with(new_piece),
      Some(loc) => {
        if loc {
          if self.first_piece.colour != new_piece.colour || self.seen_already[new_piece.shape.index()] {
            return false
          }
        } else {
          if self.first_piece.shape != new_piece.shape || self.seen_already[new_piece.colour.index()] {
            return false
          }
        }
        return true
      }
    }
  }

  pub fn clone_extend(&self, new_piece: Piece) -> Option<LineValidator> {
    if self.can_add(new_piece) {
      let mut lv2 = self.clone();
      assert!(lv2.add_piece(new_piece)); // TODO optimise!
      return Some(lv2)
    } else {
      return None
    }
  }

  /// prevents any more extensions
  pub fn seal(&mut self) {
    self.length = 6;
  }

  pub fn add_piece(&mut self, new_piece: Piece) -> bool {
    if self.length == 6 || new_piece == self.first_piece {
      return false
    }

    match self.is_line_of_colour {
      None => {
        if self.first_piece.colour == new_piece.colour {
          self.is_line_of_colour = Some(true);
          self.seen_already[self.first_piece.shape.index()] = true;
          self.seen_already[new_piece.shape.index()] = true;
          self.length = self.length + 1;
          return true
        } else if self.first_piece.shape == new_piece.shape {
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
          self.seen_already[new_piece.shape.index()] = true;
        } else {
          if self.first_piece.shape != new_piece.shape || self.seen_already[new_piece.colour.index()] {
            return false
          }
          self.seen_already[new_piece.colour.index()] = true;
        }
        self.length = self.length + 1;
        return true
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
#[should_fail]
fn test_add_identical_fail() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let mut lv = LineValidator::new(p1);
  assert!(lv.add_piece(p1));
}

#[test]
#[should_fail]
fn test_duplicate_first() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let p2 = Piece::new(Colour::G, Shape::A);
  let mut lv = LineValidator::new(p1);
  lv.add_piece(p2);
  assert!(lv.add_piece(p1));
}

#[test]
#[should_fail]
fn test_change_common_feature() {
  let p1 = Piece::new(Colour::R, Shape::A);
  let p2 = Piece::new(Colour::G, Shape::A);
  let p3 = Piece::new(Colour::G, Shape::B);
  let mut lv = LineValidator::new(p1);
  lv.add_piece(p2);
  assert!(lv.add_piece(p3));
}