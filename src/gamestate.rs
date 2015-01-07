use components::{Board, Move, PartialScored};
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

  pub fn generate_moves(&self) -> Vec<(Score,Move)> {
    let mut moves:Vec<(Score,Move)> = Vec::new();
    if self.bag.len() > 0 {
      moves.push((0,Move::SwapPieces))
    }


    // figure out possible start squares (and directions).
    for &(square, ref direction) in self.board.get_start_squares().iter() {
      // initialize queue with singletons

      let mut pieces_queue = RingBuf::new();
      for piece in self.players[self.turn].bag.iter() {
        let initial_singleton = PartialScored {
          pieces: vec![*piece],
          mainline_score: 0,
          perp_scores: 0,
          last_square: square,
        };
        pieces_queue.push_back(initial_singleton);
      }
      // figure out any possible moves starting at this start square and direction, add to `moves`
      loop {
        match pieces_queue.pop_front() {
          None => break,
          // Some(ref piece_vector) => {
          Some(ref partial) => {

            match self.board.allows(square, direction, partial) {
              None => {}
              Some((mainline_score, perp_score)) => {
                // store official to return
                let place_pieces = Move::PlacePieces(square, (*direction).clone(), partial.pieces.clone());
                // calculate full score
                moves.push((mainline_score + perp_score + partial.perp_scores, place_pieces));

                // put new partials
                'outer: for next_piece in self.players[self.turn].bag.iter() {
                  for already in partial.pieces.iter() {
                    if *next_piece == *already {
                      continue 'outer
                    }
                  }
                  let extended_partial = partial.extend(mainline_score, perp_score, direction, *next_piece);
                  pieces_queue.push_back(extended_partial);
                }
              }
            }
          },
        }
      }
    }
    return moves
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