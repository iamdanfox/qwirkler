use components::{Board, Move, PlayerState};
use components::Move::{SwapPieces, PlacePieces};
use piece::{Bag, Piece};
use std::collections::RingBuf;
// use std::io;


mod piece;
mod components;



fn main() {
  let mut game_state = GameState::new(2);
  let mut i = 0u;

  loop {
    i = i + 1;
    println!("\n\n{}\n", game_state.board);
    println!("{}: player {} turn (score = {})", i, game_state.turn, game_state.players[game_state.turn].score);

    let mut moves = game_state.generate_moves();
    match moves.pop() {
      None => break,
      Some(chosen_move) => {
        game_state = game_state.apply_move(chosen_move);
      },
    }
  }

  // compute total score
  println!("\n\n\n\nGame finished.");
  let mut sum = 0;
  let mut i = 0;
  for player in game_state.players.iter() {
    println!("  player {} score={}", i, player.score);
    sum = sum + player.score;
    i += 1;
  }
  println!("\n  total = {}", sum);
}



struct GameState {
  board: Board,
  players: Vec<PlayerState>,
  bag: Bag,
  turn: uint,
}

impl GameState {

  // factory method
  fn new(num_players: int) -> GameState {
    let mut initial_bag = piece::make_bag();
    let mut players = vec![];

    for _ in range(0, num_players) {
      let empty:Vec<uint> = vec![];
      let (player_bag, main_bag) = piece::resupply_player(empty, initial_bag);
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

  // instance method
  fn generate_moves(&self) -> Vec<Move> {

    let mut moves:Vec<Move> = Vec::new();
    if self.bag.len() > 0 {
      moves.push(Move::SwapPieces)
    }

    let current_player_bag:Bag = self.players[self.turn].clone().bag; // TODO: do this without cloning
    // TODO: turn this into a SET!! (no need to include repetitions here!)

    // figure out possible start squares (and directions).
    // FOR EACH POSSIBLE START CONFIG:
    for &(square, ref direction) in self.board.get_start_squares().iter() {
      // initialize queue with singletons
      let mut pieces_queue:RingBuf<Vec<Piece>> = RingBuf::new();
      for piece in current_player_bag.iter() {
        let singleton: Vec<Piece> = vec![*piece];
        pieces_queue.push_back(singleton);
      }
      // figure out any possible moves starting at this start square and direction, add to `moves`
      loop {
        match pieces_queue.pop_front() {
          None => break,
          Some(ref piece_vector) => {
            let place_pieces = Move::PlacePieces(square, (*direction).clone(), (*piece_vector).clone());

            if self.board.allows_move(&place_pieces) {
              moves.push(place_pieces);
              // print!("success");

              // put longer sequences back in the queue (no duplicates allowed!)
              'outer: for next_piece in current_player_bag.iter() {
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

    println!("    ({} possible moves)", moves.len());

    return moves
  }

  fn apply_move(&self, chosen_move: Move) -> GameState {
    match chosen_move {
      Move::PlacePieces(sq, dir, pieces_to_place) => {

        let (new_board, score_increment) = self.board.put(sq, &dir, &pieces_to_place);

        let mut new_players:Vec<PlayerState> = Vec::new();
        let mut final_bag:Vec<Piece> = vec![];
        for (player, i) in self.players.iter().zip(range(0, self.players.len())) {
          if self.turn == i {

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

            // TODO: place the pieces on the board and increment the player's score!
            // let new_board = board.put(sq, dir, pieces);

            new_players.push(PlayerState { score: player.score + score_increment, bag: player_bag2 });
          } else {
            new_players.push(player.clone());
          }
        }


        GameState {
          board: new_board,
          players: new_players,
          bag: final_bag,
          turn: (self.turn + 1) % self.players.len()
        }
      },
      Move::SwapPieces => {
        self.apply_swap_pieces()
      },
    }
  }

  fn apply_swap_pieces(&self) -> GameState {
    let mut new_players:Vec<PlayerState> = Vec::new();
    let mut final_bag = vec![];
    for (player, i) in self.players.iter().zip(range(0, self.players.len())) {
      if self.turn == i {
        let mut main_bag2 = Vec::new();
        main_bag2.push_all(player.bag.as_slice());
        main_bag2.push_all(self.bag.as_slice());
        let empty:Bag = vec![];
        let (player_bag2, main_bag3) = piece::resupply_player(empty, main_bag2);
        final_bag = main_bag3;
        new_players.push(PlayerState { score: player.score, bag: player_bag2 });
      } else {
        new_players.push(player.clone());
      }
    }

    final_bag.pop(); // DELETE THIS FAKE CODE ONCE PlacePieces WORKS

    GameState {
      board: self.board.clone(),
      players: new_players,
      bag: final_bag,
      turn: (self.turn + 1) % self.players.len()
    }
  }
}