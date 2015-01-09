use piece::Piece;


pub struct LineValidator {
  seen_already: [bool; 36],
  first_piece: Piece,
  second_piece: Option<Piece>,
  pub length: uint,
}

impl Clone for LineValidator {
  fn clone(&self) -> LineValidator {
    return LineValidator {
      seen_already: self.seen_already,
      first_piece: self.first_piece,
      second_piece: self.second_piece,
      length: self.length,
    }
  }
}

impl LineValidator {
  pub fn new(first_piece: Piece) -> LineValidator {
    let mut seen_already = [false; 36];
    seen_already[first_piece.index()] = true;
    return LineValidator {
      seen_already: seen_already,
      first_piece: first_piece,
      second_piece: None,
      length: 1,
    }
  }

  pub fn add_piece(&mut self, new_piece: Piece) -> bool {
    if self.length == 6 {
      return false
    } else {
      if self.seen_already[new_piece.index()] {
        return false
      } else {
        match self.second_piece {
          None => {
            if !self.first_piece.compatible_with(new_piece) {
              return false
            }
            self.second_piece = Some(new_piece);
          },
          Some(p2) => {
            if !self.first_piece.compatible3(p2, new_piece) {
              return false
            }
          }
        };
        self.length = self.length + 1;
        self.seen_already[new_piece.index()] = true;
        return true
      }
    }
  }
}
