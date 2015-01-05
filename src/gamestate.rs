use components::{Board, Move};
use piece::{Bag, Piece};
use piece;
use player::PlayerState;
use player;
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
    let mut players = vec![];

    for _ in range(0, num_players) {
      let (player_bag, main_bag) = piece::resupply_player(vec![], initial_bag);
      initial_bag = main_bag;
      players.push(PlayerState::new(player_bag));
    }

    GameState {
      board: Board::new(),
      players: players,
      bag: initial_bag,
      turn: 0,
    }
  }

  pub fn generate_moves(&self) -> Vec<Move> {
    let mut moves:Vec<Move> = Vec::new();
    if self.bag.len() > 0 {
      moves.push(Move::SwapPieces)
    }

    let mut initial_queue = RingBuf::new();
    for piece in self.players[self.turn].bag.iter() {
      let singleton: Vec<Piece> = vec![*piece];
      initial_queue.push_back(singleton);
    }

    // figure out possible start squares (and directions).
    for &(square, ref direction) in self.board.get_start_squares().iter() {
      // initialize queue with singletons
      let mut pieces_queue = initial_queue.clone();
      // figure out any possible moves starting at this start square and direction, add to `moves`
      loop {
        match pieces_queue.pop_front() {
          None => break,
          Some(ref piece_vector) => {
            let place_pieces = Move::PlacePieces(square, (*direction).clone(), (*piece_vector).clone());

            if self.board.allows_move(&place_pieces) {
              moves.push(place_pieces);

              // put longer sequences back in the queue (no duplicates allowed!)
              'outer: for next_piece in self.players[self.turn].bag.iter() {
                for already in piece_vector.iter() {
                  if *next_piece == *already {
                    continue 'outer
                  }
                }
                let mut appended = piece_vector.clone();
                appended.push(*next_piece);
                pieces_queue.push_back(appended);
              }
            }
          },
        }
      }
    }
    return moves
  }

  pub fn apply_move(&mut self, chosen_move: &Move)  {
    match chosen_move {
      &Move::PlacePieces(sq, ref dir, ref pieces_to_place) => {
        let score_increment = self.board.put(sq, dir, pieces_to_place);
        let mut final_bag:Vec<Piece> = vec![];

        let new_players = player::mutate_current(self.turn, &self.players, |player| {
          let mut depleted_player_bag:Vec<Piece> = Vec::new();
          'outer: for existing_piece in player.bag.iter() {
            for piece in pieces_to_place.iter() {
              if *existing_piece == *piece {
                continue 'outer;
              }
            }
            depleted_player_bag.push(*existing_piece);
          }

          let (player_bag2, main_bag2) = piece::resupply_player(depleted_player_bag, self.bag.clone());
          final_bag = main_bag2;

          return PlayerState {
            score: player.score + score_increment,
            bag: player_bag2
          }
        });

        self.players = new_players;
        self.bag = final_bag;
        self.turn = (self.turn + 1) % self.players.len();

        // GameState {
        //   board: self.board,
        //   players: new_players,
        //   bag: final_bag,
        //   turn: (self.turn + 1) % self.players.len()
        // }
      },
      &Move::SwapPieces => {
        let mut final_bag = vec![];

        let new_players = player::mutate_current(self.turn, &self.players, |player| {
          let mut main_bag2 = Vec::new();
          main_bag2.push_all(player.bag.as_slice());
          main_bag2.push_all(self.bag.as_slice());
          let empty:Bag = vec![];
          let (player_bag2, main_bag3) = piece::resupply_player(empty, main_bag2);
          final_bag = main_bag3;

          return PlayerState {
            score: player.score,
            bag: player_bag2
          }
        });

        self.players = new_players;
        self.bag = final_bag;
        self.turn = (self.turn + 1) % self.players.len();
        // GameState {
        //   board: self.board.clone(),
        //   players: new_players,
        //   bag: final_bag,
        //   turn: (self.turn + 1) % self.players.len()
        // }
      },
    }
  }
}