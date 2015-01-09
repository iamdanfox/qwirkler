use piece::Bag;

pub type Score = usize;

#[derive(Show, Clone)]
pub struct PlayerState {
  pub bag: Bag,
  pub score: Score,
}

impl PlayerState {
  pub fn new(bag: Bag) -> PlayerState {
    PlayerState { bag: bag, score: 0 }
  }
}
