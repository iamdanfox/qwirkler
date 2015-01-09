use piece::{Piece};
use linevalidator::LineValidator;
use player::Score;
use direction::{Square, Direction};
use mv::Move;

#[derive(Clone)]
pub struct Partial {
  pub start_square: Square,
  pub direction: Direction,
  pub pieces: Vec<Piece>,
  pub last_square: Square,
  pub mainline_score: Score,
  pub perp_scores: Score,
  pub main_validator: Option<LineValidator>,
}

impl Partial {
  pub fn new(square:Square, direction: &Direction, piece:Piece) -> Partial {
    return Partial {
      start_square: square,
      direction: *direction,
      pieces: vec![piece],
      mainline_score: 0,
      perp_scores: 0,
      last_square: square,
      main_validator: None,
    };
  }

  pub fn total_score(&self) -> Score {
    return self.mainline_score + self.perp_scores;
  }

  pub fn save_as_move(&self) -> Move {
    return Move::PlacePieces(self.start_square, self.direction, self.pieces.clone(), self.total_score());
  }

  pub fn extend(&self, next_piece:Piece) -> Partial {
    let mut extended_partial = self.clone();
    extended_partial.pieces.push(next_piece);
    extended_partial.last_square = self.direction.apply(self.last_square);
    return extended_partial;
  }
}
