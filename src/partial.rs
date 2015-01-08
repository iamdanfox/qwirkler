use piece::{Piece, LineValidator};
use player::Score;
use direction::{Square, Direction};

pub struct PartialScored {
  pub pieces: Vec<Piece>,
  pub last_square: Square,
  pub mainline_score: Score,
  pub perp_scores: Score,
  main_validator: LineValidator
}

impl PartialScored {
  pub fn new(piece:Piece, square:Square) -> PartialScored {
    return PartialScored {
      pieces: vec![piece],
      mainline_score: 0,
      perp_scores: 0,
      last_square: square,
      main_validator: LineValidator::new(piece),
    };
  }

  pub fn extend(&self, new_mainline_score:uint, new_perp_score:uint, direction:&Direction, next_piece:Piece) -> PartialScored {
    let mut appended = self.pieces.clone();
    appended.push(next_piece);

    return PartialScored {
      pieces: appended,
      last_square: direction.apply(self.last_square),
      mainline_score: new_mainline_score,
      perp_scores: self.perp_scores + new_perp_score,
      main_validator: self.main_validator.clone(),
    };
  }
}
