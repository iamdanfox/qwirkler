use std::rand::{thread_rng, Rng};

pub type Bag = Vec<Piece>;
pub type Piece = uint;

/**

Colours are 1-6
Shapes are 10-60
(00) is empty


e.g. a Red Square is 11
*/


pub fn make_bag() -> Bag {
  // TODO: maybe use std::collections::enum_set?
  range(0, 108).map(|i| 1 + (i % 6) + (10 + ((i / 6) * 10) % 60)).collect()
}

pub fn resupply_player(player_bag: Bag, main_bag: Bag) -> (Bag, Bag) {
  // shuffle main_bag -> main_bag2
  let mut main_bag2 = Vec::new();
  let mut rng = thread_rng();
  let mut mutable_bag = main_bag;
  main_bag2.push_all({
    let slice = mutable_bag.as_mut_slice();
    rng.shuffle(slice);
    slice
  });

  let num_to_take = 6 - player_bag.len();
  let mut player_bag2 = player_bag;

  for _ in range(0, num_to_take) {
    main_bag2.pop().map(|piece| player_bag2.push(piece));
  }

  return (player_bag2, main_bag2)
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
