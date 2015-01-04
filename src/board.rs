// use piece::Piece;


pub type Square = (uint,uint);

#[derive(Show)]
pub enum Direction {
  U, D, L, R
}

// impl Direction {
//   fn apply(direction: Direction, square: Square) -> Square {
//     let (x,y) = square;
//     match direction {
//       Direction::U => (x,y+1),
//       Direction::D => (x,y-1),
//       Direction::L => (x-1,y),
//       Direction::R => (x+1,y),
//     }
//   }
// }


pub struct Board {
  board: [[uint; 50]; 50] // compatible with uint Piece
}

impl Board {
  pub fn new() -> Board {
    let new_board = [[0; 50]; 50];
    Board { board: new_board}
  }

  // fn put(&self, square: Square, direction: Direction, pieces: Vec<Piece>) -> Board {
  //   let (x,y) = square;
  //   let mut new_board = self.board;

  //   new_board[x][y] = 99;

  //   Board { board: new_board }
  // }
}

impl Clone for Board {
  fn clone(&self) -> Board {
    Board { board: self.board }
  }
}
