use std::fmt;

pub type Square = (isize,isize);

#[derive(Copy,PartialEq,Clone)]
pub enum Direction {
  U,D,L,R
}

impl fmt::Show for Direction {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    (match *self {
      Direction::U => "U",
      Direction::D => "D",
      Direction::L => "L",
      Direction::R => "R",
    }).fmt(formatter)
  }
}

const U:Direction = Direction::U;
const D:Direction = Direction::D;
const L:Direction = Direction::L;
const R:Direction = Direction::R;

impl Direction {
  pub fn apply(&self, (x,y): Square) -> Square {
    match *self {
      U => (x, y+1),
      D => (x, y-1),
      L => (x-1, y),
      R => (x+1, y),
    }
  }

  pub fn opposite(&self) -> Direction {
    match *self {
      U => D,
      D => U,
      L => R,
      R => L,
    }
  }

  pub fn perpendiculars(&self) -> (Direction,Direction) {
    match *self {
      U | D => (L,R),
      L | R => (U,D),
    }
  }

  pub fn apply_all(&self, sq: Square, len: usize) -> Vec<Square> {
    let mut squares = vec![];
    let mut last = sq;
    for _ in (0..len) {
      squares.push(last);
      last = self.apply(last);
    }
    return squares
  }

  pub fn all() -> Vec<Direction> {
    return vec![U,D,L,R]
  }

  pub fn initial() -> Direction {
    return R
  }
}
