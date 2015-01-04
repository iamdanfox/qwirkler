#[derive(Show, Clone)]
pub struct Piece(Colour, Shape);

#[derive(Show, Clone)]
pub enum Colour {
  R, O, Y, G, B, P,
}

#[derive(Show, Clone)]
pub enum Shape {
  S, // square
  D, // diamond
  C, // circle
  B, // clubs
  E, // explosion
  X, // cross
}
