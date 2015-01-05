use std::rand::{thread_rng, Rng};

pub type Bag = Vec<Piece>;
pub type Piece = uint;


pub fn make_bag() -> Bag {
  // TODO: maybe use std::collections::enum_set?
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
