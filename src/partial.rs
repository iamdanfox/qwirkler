use piece::{Piece};
use linevalidator::LineValidator;
use player::Score;
use direction::{Square, Direction};
use mv::Move;

pub struct Partial {
  pub start_square:   Square,
  pub direction:      Direction,
  pub pieces:         Vec<Piece>,
  pub last_square:    Square,
  pub mainline_score: Score,
  pub perp_scores:    Score,
  pub main_validator: Option<LineValidator>,
}

impl Partial {
  pub fn new(square:Square, direction: &Direction, piece:Piece) -> Partial {
    return Partial {
      start_square:   square,
      direction:      *direction,
      pieces:         vec![piece],
      mainline_score: 0,
      perp_scores:    0,
      last_square:    square,
      main_validator: None,
    };
  }

  pub fn total_score(&self) -> Score {
    return self.mainline_score + self.perp_scores;
  }

  pub fn save_as_move(&self) -> Move {
    return Move::PlacePieces(self.start_square, self.direction, self.pieces.clone(), self.total_score());
  }

  pub fn try_extend(&self, next_piece:Piece) -> Option<Partial> {
    match self.main_validator {
      None => None,
      Some(ref lv) => {
        match lv.clone_extend(next_piece) {
          None => None,
          Some(lv2) => {
            let mut new_pieces = self.pieces.clone();
            new_pieces.push(next_piece);
            Some(Partial {
              start_square:   self.start_square,
              direction:      self.direction,
              pieces:         new_pieces,
              mainline_score: self.mainline_score,
              perp_scores:    self.perp_scores,
              last_square:    self.direction.apply(self.last_square),
              main_validator: Some(lv2),
            })
          }
        }
      }
    }
  }
}
