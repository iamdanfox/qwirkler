
pub type Square = (int,int);

#[derive(Show, Clone)]
pub enum Direction {
  U, D, L, R
}

impl Direction {
  pub fn apply(&self, sq: Square) -> Square {
    let (x,y) = sq;
    match *self {
      Direction::U => (x, y+1),
      Direction::D => (x, y-1),
      Direction::L => (x-1, y),
      Direction::R => (x+1, y),
    }
  }

  pub fn opposite(&self) -> Direction {
    match *self {
      Direction::U => Direction::D,
      Direction::D => Direction::U,
      Direction::L => Direction::R,
      Direction::R => Direction::L,
    }
  }

  pub fn perpendiculars(&self) -> (Direction,Direction) {
    match *self {
      Direction::U | Direction::D => (Direction::L, Direction::R),
      Direction::L | Direction::R => (Direction::U, Direction::D),
    }
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
    return vec![Direction::U, Direction::D, Direction::L, Direction::R]
  }

  pub fn initial() -> Direction {
    return Direction::R
  }
}
