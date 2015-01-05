use std::rand::{thread_rng, Rng};

pub type Bag = Vec<Piece>;
pub type Piece = uint;


pub fn make_bag() -> Bag {
  // TODO: maybe use std::collections::enum_set?
  // this generates three copies of ij for i <- [1..6] and j <- [1..6]
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

pub fn is_blank(p: Piece) -> bool {
  return p == 0
}

pub fn blank() -> Piece {
  return 0
}


pub fn all_unique(line: &Vec<Piece>) -> bool {
  let mut seen_already = [false; 67];
  for piece in line.iter() {
    if seen_already[*piece] {
      return false;
    } else {
      seen_already[*piece] = true;
    }
  }
  return true
}

pub fn all_same_colour(line: &Vec<Piece>) -> bool {
  let first = line[0] / 10;
  for piece in line.iter() {
    if (*piece) / 10 != first {
      return false
    }
  }
  return true
}

pub fn all_same_shape(line: &Vec<Piece>) -> bool {
  let first = line[0] % 10;
  for piece in line.iter() {
    if (*piece) % 10 != first {
      return false
    }
  }
  return true
}