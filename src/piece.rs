use std::rand::{thread_rng, Rng};
use std::fmt;
use linevalidator::LineValidator;

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

  pub fn colour(&self) -> u8 {
    return self.internal >> 4;
  }

  pub fn shape(&self) -> u8 {
    return self.internal & 0b0000_1111;
  }

  pub fn is_blank(&self) -> bool {
    return self.internal == 0;
  }

  pub fn index(&self) -> uint {
    return ((self.colour() * 10) + self.shape()) as uint
  }

  pub fn to_string(&self) -> String {
    return ((self.colour() * 10) + self.shape()).to_string();
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
pub fn blank() -> Piece {
  return Piece::blank()
}

#[inline(always)]
pub fn valid_line(line: &Vec<Piece>) -> bool {
  let lv = LineValidator::accept_all(line);
  return !lv.is_none()
}
