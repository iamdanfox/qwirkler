use piece::Bag;

pub type Score = usize;

#[derive(Debug, Clone)]
pub struct PlayerState {
  pub bag: Bag,
  pub score: Score,
}

impl PlayerState {
  pub fn new() -> PlayerState {
    PlayerState { bag: vec![], score: 0 }
  }
}
