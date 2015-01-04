pub type Bag = Vec<Piece>;
pub type Piece = uint;
/**

Colours are 1-6
Shapes are 10-60
(00) is empty


e.g. a Red Square is 11
*/


pub fn make_bag() -> Vec<Piece> {
  // TODO: maybe use std::collections::enum_set?
  range(0, 108).map(|i| 1 + (i % 6) + (10 + ((i / 6) * 10) % 60)).collect()
}


// pub use self::Colour::{R, O, Y, G, B, P};
// pub use self::Shape::{H,I,J,K,L,M};

// #[derive(Show, Clone)]
// pub struct Piece(Colour, Shape);


// #[derive(Show, Clone)]
// pub enum Colour {
//   R,O,Y,G,B,P
// // }

// impl Colour {
//   fn all() -> Vec<Colour> {
//     vec![R,O,Y,G,B,P]
//   }
// }

// // TODO: define default types or something http://doc.rust-lang.org/std/default/
// // TODO: equality of members?
// // TODO: make these enums not involve pointers.

// #[derive(Show, Clone)]
// pub enum Shape {
//   H=10,I=20,J=30,K=40,L=50,M=60,
// }

// impl Shape {
//   fn all() -> Vec<Shape> {
//     vec![H,I,J,K,L,M]
//   }
// }
