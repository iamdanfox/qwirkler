use components::{Board, Move};
use partial::PartialScored;
use piece::{Bag, Piece};
use piece;
use player::{PlayerState, Score};
use std::collections::RingBuf;


pub struct GameState {
  pub board: Board,
  pub players: Vec<PlayerState>,
  bag: Bag,
  pub turn: uint,
}


impl GameState {

  pub fn new(num_players: int) -> GameState {
    let mut initial_bag = piece::make_bag();

    let players = range(0, num_players).map( |_| {
      let player_bag = piece::resupply_player_mutate(vec![], &mut initial_bag);
      PlayerState::new(player_bag)
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
  pub fn generate_best_move(&self) -> Option<(Score,Move)> {
    let mut best_score = 0;
    let mut best_move = Move::SwapPieces;

    // We use a RingBuf so that we can remove short ideas from the front
    // and test them first, adding longer ones to the end. (Vec<...> doesn't allow this)
    // Invariants for every partial in the queue:
    //  * the prefix of partial.pieces has already been validated (so we just need to check the last piece)
    //  * partial.last_square is the square that the last piece would fall on
    //  * partial.main_validator is the result of validating everything before the start of the line,
    //    and everything except the last element in the `pieces` vector (ie, it's None for singletons)
    //  * partial.perp_scores stores the points that would be gained from any perpendicular lines that
    //    this play would form.
    let mut queue:RingBuf<PartialScored> = RingBuf::new();

    // The PartialStruct data structure allows us to check increasingly long sequences of pieces
    // without repeating any validation or scoring work when testing the long ones.

    // figure out possible start squares (and directions).
    for &(square, ref direction) in self.board.get_start_squares().iter() {
      // initialize queue with singletons
      for &piece in self.players[self.turn].bag.iter() {
        queue.push_back(PartialScored::new(piece, square));
      }
      // figure out any possible moves starting at this start square and direction, add to `moves`
      loop {
        match queue.pop_front().as_mut() {
          None => break,
          Some(partial) => {

            match self.board.allows(square, direction, partial) {
              None => {}
              Some((mainline_score, perp_score)) => {
                // calculate full score and return move
                let total_score = mainline_score + perp_score + partial.perp_scores;
                if total_score > best_score {
                  best_score = total_score;
                  best_move = Move::PlacePieces(square, (*direction).clone(), partial.pieces.clone());
                }

                // put new partials back in
                'outer: for &next_piece in self.players[self.turn].bag.iter() {
                  for &already in partial.pieces.iter() {
                    if next_piece == already {
                      continue 'outer
                    }
                  }
                  let extended_partial = partial.extend(mainline_score, perp_score, direction, next_piece);
                  queue.push_back(extended_partial);
                }
              }
            }
          },
        }
      }
    }
    if self.bag.len() == 0 && best_score == 0 {
      return None
    } else {
      return Some((best_score,best_move))
    }
  }

  pub fn apply_move(&mut self, chosen_move: &Move, score: Score)  {
    match chosen_move {
      &Move::PlacePieces(sq, ref dir, ref pieces_to_place) => {
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

        self.players[self.turn] = PlayerState {
          score: self.players[self.turn].score + score,
          bag: piece::resupply_player_mutate(depleted_player_bag, &mut self.bag),
        };
      },
      &Move::SwapPieces => {
        // return pieces to bag
        self.bag.push_all(self.players[self.turn].bag.as_slice());
        // do shuffle and re-draw 6 (if possible)
        self.players[self.turn] = PlayerState {
          score: self.players[self.turn].score,
          bag: piece::resupply_player_mutate(vec![], &mut self.bag),
        };
      },
    }
    self.turn = (self.turn + 1) % self.players.len();
  }
}