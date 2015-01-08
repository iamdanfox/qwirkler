use std::rand::{thread_rng, Rng};
use std::fmt;

pub type Bag = Vec<Piece>;


#[derive(Copy, Clone)]
pub struct Piece {
  internal: u8,
}

impl Piece {
  fn new(colour: u8, shape: u8) -> Piece {
    return Piece {
      internal: (colour << 4) ^ shape
    }
  }

  fn blank() -> Piece {
    return Piece {
      internal: 0
    }
  }

  fn colour(&self) -> u8 {
    return self.internal >> 4;
  }

  fn shape(&self) -> u8 {
    return self.internal & 0b0000_1111;
  }

  fn is_blank(&self) -> bool {
    return self.internal == 0;
  }

  pub fn to_string(&self) -> String {
    return ((self.colour() * 10) + self.shape()).to_string();
  }
}


impl fmt::Show for Piece {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    self.to_string().fmt(formatter)
  }
}

impl PartialEq for Piece {
  fn eq(&self, other: &Piece) -> bool {
    return self.internal == other.internal
  }
}




pub fn make_bag() -> Bag {
  // this generates three copies of ij for i <- [1..6] and j <- [1..6]
  return range(0, 108).map(|i| Piece::new(1 + (i % 6), 1 + ((i / 6) % 6))).collect();
}

// TODO: try eliminating randomness for repeatable benchmarks

#[inline(always)]
pub fn resupply_player_mutate(player_bag: Bag, main_bag: &mut Bag) -> Bag {
  {
    let mut rng = thread_rng();
    let slice = main_bag.as_mut_slice();
    rng.shuffle(slice);
  }

  let num_to_take = 6 - player_bag.len();
  let mut player_bag2 = player_bag;

  for _ in range(0, num_to_take) {
    main_bag.pop().map(|piece| player_bag2.push(piece));
  }

  return player_bag2
}

#[inline(always)]
pub fn is_blank(p: Piece) -> bool {
  return p.is_blank()
}

#[inline(always)]
pub fn blank() -> Piece {
  return Piece::blank()
}

fn index(piece: Piece) -> uint {
  return ((piece.colour() * 10) + piece.shape()) as uint
}

#[inline(always)]
pub fn valid_line(line: &Vec<Piece>) -> bool {
  if line.len() == 1 {
    return true;
  }
  if line.len() > 6 {
    return false;
  }
  if !all_unique(line) {
    return false;
  }
  if !all_same_colour(line) && !all_same_shape(line) {
    return false;
  }
  return true;
}

#[inline(always)]
fn all_unique(line: &Vec<Piece>) -> bool {
  let mut seen_already = [false; 67];
  for piece in line.iter() {
    if seen_already[index(*piece)] {
      return false;
    } else {
      seen_already[index(*piece)] = true;
    }
  }
  return true
}

#[inline(always)]
fn all_same_colour(line: &Vec<Piece>) -> bool {
  let first = line[0].colour();
  for piece in line.iter() {
    if (*piece).colour() != first {
      return false
    }
  }
  return true
}

#[inline(always)]
fn all_same_shape(line: &Vec<Piece>) -> bool {
  let first = line[0].shape();
  for piece in line.iter() {
    if (*piece).shape() != first {
      return false
    }
  }
  return true
}