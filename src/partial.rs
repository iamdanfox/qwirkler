use piece::{Piece};
use linevalidator::LineValidator;
use player::Score;
use direction::{Square, Direction};

pub struct Partial {
  pub pieces: Vec<Piece>,
  pub last_square: Square,
  pub mainline_score: Score,
  pub perp_scores: Score,
  pub main_validator: Option<LineValidator>
}

impl Partial {
  pub fn new(piece:Piece, square:Square) -> Partial {
    return Partial {
      pieces: vec![piece],
      mainline_score: 0,
      perp_scores: 0,
      last_square: square,
      main_validator: None,
    };
  }

  pub fn extend(&self, new_mainline_score:uint, new_perp_score:uint, direction:&Direction, next_piece:Piece) -> Partial {
    let mut appended = self.pieces.clone();
    appended.push(next_piece);
    let lv = match self.main_validator {
      None => None,
      Some(ref validator) => Some(validator.clone()),
    };
    return Partial {
      pieces: appended,
      last_square: direction.apply(self.last_square),
      mainline_score: new_mainline_score,
      perp_scores: self.perp_scores + new_perp_score,
      main_validator: lv,
    };
  }
}
