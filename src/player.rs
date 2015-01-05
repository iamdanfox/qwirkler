use piece::Bag;

#[derive(Show, Clone)]
pub struct PlayerState {
  pub bag: Bag,
  pub score: int,
}

impl PlayerState {
  pub fn new(bag: Bag) -> PlayerState {
    PlayerState { bag: bag, score: 0 }
  }
}

pub fn mutate_current(turn: uint, players:&Vec<PlayerState>, mutate: |&PlayerState|->PlayerState) -> Vec<PlayerState> {
  let mut new_players:Vec<PlayerState> = Vec::new();
  for (player, i) in players.iter().zip(range(0, players.len())) {
    if i == turn {
      let mutated = mutate(player);
      new_players.push(mutated);
    } else {
      new_players.push(player.clone());
    }
  }
  return new_players
}
