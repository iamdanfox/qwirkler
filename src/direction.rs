use std::fmt;

pub type Square = (int,int);

#[derive(Copy,PartialEq,Clone)]
pub struct Direction {
  internal: u8
}

impl fmt::Show for Direction {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    (match *self {
      U => "U",
      D => "D",
      L => "L",
      R => "R",
      _ => "X"
    }).fmt(formatter)
  }
}

const U:Direction = Direction { internal: 0b0000_0011 };
const D:Direction = Direction { internal: 0b0000_0000 };
const L:Direction = Direction { internal: 0b0000_0010 };
const R:Direction = Direction { internal: 0b0000_0001 };

impl Direction {
  pub fn apply(&self, (x,y): Square) -> Square {
    match *self {
      U => (x, y+1),
      D => (x, y-1),
      L => (x-1, y),
      R => (x+1, y),
      _ => (x,y)
    }
  }

  pub fn opposite(&self) -> Direction {
    return Direction {
      internal: (0b1111_1111 ^ self.internal) & 0b0000_0011
    };
  }

  pub fn perpendiculars(&self) -> (Direction,Direction) {
    let rot90 = Direction {
      internal: self.internal ^ 0b0000_0001
    };
    return (rot90, rot90.opposite());
  }

  pub fn apply_all(&self, sq: Square, len: uint) -> Vec<Square> {
    let mut squares = vec![];
    let mut last = sq;
    for _ in range(0, len) {
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
