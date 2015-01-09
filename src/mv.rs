use piece::{Piece};
use direction::{Square, Direction};
use player::Score;

#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>, Score)
}
