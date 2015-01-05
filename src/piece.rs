use std::rand::{thread_rng, Rng};

pub type Bag = Vec<Piece>;
pub type Piece = uint;


pub fn make_bag() -> Bag {
  // this generates three copies of ij for i <- [1..6] and j <- [1..6]
  range(0, 108).map(|i| 1 + (i % 6) + (10 + ((i / 6) * 10) % 60)).collect()
}

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
  return p == 0
}

#[inline(always)]
pub fn blank() -> Piece {
  return 0
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
    if seen_already[*piece] {
      return false;
    } else {
      seen_already[*piece] = true;
    }
  }
  return true
}

#[inline(always)]
fn all_same_colour(line: &Vec<Piece>) -> bool {
  let first = line[0] / 10;
  for piece in line.iter() {
    if (*piece) / 10 != first {
      return false
    }
  }
  return true
}

#[inline(always)]
fn all_same_shape(line: &Vec<Piece>) -> bool {
  let first = line[0] % 10;
  for piece in line.iter() {
    if (*piece) % 10 != first {
      return false
    }
  }
  return true
}