use board::{Square, Direction};
use piece::{Piece};

#[derive(Show)]
pub enum Move {
  SwapPieces,
  PlacePieces(Square, Direction, Vec<Piece>)
}
