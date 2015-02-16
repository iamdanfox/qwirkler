use piece::{Piece};
use direction::{Square, Direction};
use player::Score;

#[derive(Debug)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>, Score)
}
