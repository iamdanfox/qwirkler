use std::rand::{thread_rng, Rng};
use std::fmt;

pub type Bag = Vec<Piece>;

#[derive(Copy, Clone, PartialEq, Show, Eq)]
pub enum Colour {
  R,O,Y,G,B,P
}

impl Colour {
  pub fn index(&self) -> usize {
    match *self {
      Colour::R => 0,
      Colour::O => 1,
      Colour::Y => 2,
      Colour::G => 3,
      Colour::B => 4,
      Colour::P => 5,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Show, Eq)]
pub enum Shape {
  A,B,C,D,E,F
}

impl Shape {
  pub fn index(&self) -> usize {
    match *self {
      Shape::A => 0,
      Shape::B => 1,
      Shape::C => 2,
      Shape::D => 3,
      Shape::E => 4,
      Shape::F => 5,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Show, Eq)]
pub struct Piece {
  pub colour: Colour,
  pub shape: Shape,
}

impl Piece {
  pub fn new(colour: Colour, shape: Shape) -> Piece {
    return Piece {
      colour: colour,
      shape: shape,
    }
  }

  pub fn colour(&self) -> Colour {
    return self.colour;
  }

  pub fn shape(&self) -> Shape {
    return self.shape;
  }

  pub fn index(&self) -> usize {
    return 6*self.colour.index() + self.shape.index();
  }

  pub fn to_string(&self) -> String {
    let mut s = String::new();
    s.push_str((1+self.colour.index()).to_string().as_slice());
    s.push_str((1+self.shape.index()).to_string().as_slice());
    return s;
  }

  pub fn compatible_with(&self, piece2: Piece) -> bool {
    if self.colour() == piece2.colour() {
      return true
    } else {
      if self.shape() == piece2.shape() {
        return true
      } else {
        return false
      }
    }
  }

  pub fn compatible3(&self, piece2: Piece, piece3: Piece) -> bool {
    if self.colour() == piece2.colour() && self.colour() == piece3.colour() {
      return true
    } else {
      if self.shape() == piece2.shape() && self.shape() == piece3.shape() {
        return true
      } else {
        return false
      }
    }
  }
}

pub fn make_bag() -> Bag {
  // this generates three copies of ij for i <- [1..6] and j <- [1..6]
  let mut res = vec![];
  for &c in vec![Colour::R, Colour::O, Colour::Y, Colour::G, Colour::B, Colour::P].iter() {
    for &s in vec![Shape::A, Shape::B, Shape::C, Shape::D, Shape::E, Shape::F].iter() {
      for _ in (0..3) {
        res.push(Piece::new(c,s));
      }
    }
  }
  return res;
}

pub fn resupply_player_mutate(player_bag: Bag, main_bag: &mut Bag) -> Bag {
  {
    let mut rng = thread_rng();
    let slice = main_bag.as_mut_slice();
    rng.shuffle(slice);
  }

  let num_to_take = 6 - player_bag.len();
  let mut player_bag2 = player_bag;

  for _ in (0..num_to_take) {
    main_bag.pop().map(|piece| player_bag2.push(piece));
  }

  return player_bag2
}
