use rand::{thread_rng, Rng};


pub type Bag = Vec<Piece>;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
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

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
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

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
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

  pub fn to_string(&self) -> String {
    let mut s = String::new();
    s.push_str((1+self.colour.index()).to_string().as_ref());
    s.push_str((1+self.shape.index()).to_string().as_ref());
    return s;
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

pub fn resupply_player_mutate(player_bag: &mut Bag, main_bag: &mut Bag) {
  {
    let mut rng = thread_rng();
    let slice = main_bag.as_mut_slice();
    rng.shuffle(slice);
  }
  let num_to_take = 6 - player_bag.len();
  for _ in (0..num_to_take) {
    main_bag.pop().map(|piece| player_bag.push(piece));
  }
}
