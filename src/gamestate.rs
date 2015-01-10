use board::Board;
use mv::Move;
use partial::Partial;
use piece::{Bag, Piece};
use piece;
use player::{PlayerState, Score};
use std::collections::RingBuf;
use std::mem;

pub struct GameState {
  pub board: Board,
  pub players: Vec<PlayerState>,
  bag: Bag,
  pub turn: usize,
}


impl GameState {

  pub fn new(num_players: isize) -> GameState {
    let mut initial_bag = piece::make_bag();

    let players = (0..num_players).map( |_| {
      let mut ps = PlayerState::new();
      piece::resupply_player_mutate(&mut ps.bag, &mut initial_bag);
      ps
    }).collect();

    GameState {
      board: Board::new(),
      players: players,
      bag: initial_bag,
      turn: 0,
    }
  }

  pub fn total_score(&self) -> Score {
    let mut sum = 0;
    for player in self.players.iter() {
      sum = sum + player.score;
    }
    return sum;
  }

  // TODO re-use code between generate_best_move and generate_moves
  pub fn generate_best_move(&self) -> Option<Move> {
    let mut best_score = 0;
    let mut best_move = Move::SwapPieces;

    // We use a RingBuf as a queue to test increasingly long sequences of pieces
    // without repeating any validation or scoring work we did testing the prefixes.
    let mut queue:RingBuf<Partial> = RingBuf::new();

    // Invariants for every partial in the queue:
    //  * every prefix of partial.pieces has already been validated (so we just need to check the last piece)
    //  * we have already computed the score for the n-1 prefix
    //  * partial.last_square is the square that the last piece would fall on
    //  * partial.main_validator is the result of validating everything before the start of the line,
    //    and everything except the last element in the `pieces` vector (ie, it's None for singletons)
    //  * partial.perp_scores stores the poisizes that would be gained from any perpendicular lines that
    //    this play would form.

    // figure out possible start squares (and directions).
    for &(square, ref direction) in self.board.get_start_squares().iter() {
      // initialize queue with singletons
      for &piece in self.players[self.turn].bag.iter() {
        queue.push_back(Partial::new(square, direction, piece));
      }
      // figure out any possible moves starting at this start square and direction, add to `moves`
      loop {
        match queue.pop_front().as_mut() {
          None => break,
          Some(partial) => {
            if self.board.allows(partial, &self.players[self.turn].bag) {
              // put new partials back in
              for &p in self.players[self.turn].bag.iter() {
                match partial.try_extend(p) {
                  None => {},
                  Some(extended) => queue.push_back(extended),
                }
              }

              // calculate full score and return move
              if partial.total_score() > best_score {
                best_score = partial.total_score();
                best_move = partial.save_as_move();
              }
            }
          }
        }
      }
    }
    if self.bag.len() == 0 && best_score == 0 {
      return None
    } else {
      return Some(best_move)
    }
  }

  pub fn apply_move(&mut self, chosen_move: &Move)  {
    match chosen_move {
      &Move::PlacePieces(sq, ref dir, ref pieces_to_place, score) => {
        self.board.put(sq, dir, pieces_to_place);

        let mut depleted_player_bag:Vec<Piece> = Vec::new();
        'outer: for existing_piece in self.players[self.turn].bag.iter() {
          for piece in pieces_to_place.iter() {
            if *existing_piece == *piece {
              continue 'outer;
            }
          }
          depleted_player_bag.push(*existing_piece);
        }
        self.players[self.turn].bag = depleted_player_bag;
        piece::resupply_player_mutate(&mut self.players[self.turn].bag, &mut self.bag);

        self.players[self.turn].score += score;
      },
      &Move::SwapPieces => {
        let oldbag = mem::replace(&mut self.players[self.turn].bag, vec![]);
        // return pieces to bag
        self.bag.push_all(oldbag.as_slice());
        // do shuffle and re-draw 6 (if possible)
        piece::resupply_player_mutate(&mut self.players[self.turn].bag, &mut self.bag);
      },
    }
    self.turn = (self.turn + 1) % self.players.len();
  }
}