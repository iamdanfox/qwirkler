use piece::{Piece};
use direction::{Square, Direction};

#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>)
}
