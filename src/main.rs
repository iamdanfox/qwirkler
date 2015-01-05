use components::{Board, Move, PlayerState};
use components::Move::{SwapPieces, PlacePieces};
use piece::{Bag, Piece};
use std::collections::RingBuf;

// imports all the `pub` stuff from piece.rs
mod piece;
mod components;


fn main() {

  let mut game_state = GameState::new(2);

  loop {
    let mut moves = game_state.generate_moves();
    match moves.pop() {
      None => break,
      Some(chosen_move) => {
        println!("player {} turn", game_state.turn);
        game_state = game_state.apply_move(chosen_move);
      },
    }
  }

  println!("Game finished.")

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

    let current_player_bag:Bag = self.players[self.turn].clone().bag; // TODO: do this without cloning
    // TODO: turn this into a SET!! (no need to include repetitions here!)

    // figure out possible start squares (and directions).
    // FOR EACH POSSIBLE START CONFIG:
    for &(square, ref direction) in self.board.get_start_squares().iter() {
      // initialize queue with singletons
      let mut queue:RingBuf<Vec<Piece>> = RingBuf::new();
      for piece in current_player_bag.iter() {
        queue.push_back(vec![*piece])
      }
      // figure out any possible moves starting at this start square and direction, add to `moves`
      loop {
        match queue.pop_front() {
          None => break,
          Some(piece_vector) => {
            let place_pieces = Move::PlacePieces(square, (*direction).clone(), piece_vector);
            println!("{}", place_pieces);
            if self.board.allows_move() {
              moves.push(place_pieces);
              // TODO: also put longer lists into our queue [p1] works try [p1,px], [p1,py], [p1,pz]...
            }
          },
        }
      }
    }


    if self.bag.len() > 0 {
      moves.push(Move::SwapPieces)
    }

    return moves
  }

  fn apply_move(&self, chosen_move: Move) -> GameState {
    // fake code
    let mut new_bag = self.bag.clone();
    new_bag.pop();

    match chosen_move {
      Move::SwapPieces => {
        self.apply_swap_pieces()
      },
      Move::PlacePieces(_sq, _dir, _pieces) => {
        // remove pieces from the player's bag.
        // resupply players bag from the main bag
        // let new_board = board.put(sq, dir, pieces);
        println!("placespieces");

        GameState {
          board: self.board.clone(),
          players: self.players.clone(),
          bag: new_bag,
          turn: (self.turn + 1) % self.players.len()
        }
      }
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
    // TODO: I think this is reversing the list of players...

    final_bag.pop(); // DELETE THIS FAKE CODE ONCE PlacePieces WORKS

    GameState {
      board: self.board.clone(),
      players: new_players,
      bag: final_bag,
      turn: (self.turn + 1) % self.players.len()
    }
  }
}