use crate::{events::*, game_state::*};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub state: GameState,
}

impl Game {
    pub fn new(options: Option<GameOptions>, rng_seed: Option<[u8; 32]>) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: GameState::new(options, rng_seed),
        }
    }
    pub fn allows_joining(&self) -> bool {
        self.state.status == GameStatus::Waiting
            && self.state.players.len() as u32 != self.state.options.players
    }
    pub fn is_running(&self) -> bool {
        self.state.status == GameStatus::XTurn || self.state.status == GameStatus::OTurn
    }
    pub fn get_player_in_turn(&self) -> &Player {
        self.state.players.get(&self.state.player_in_turn).unwrap()
    }
    pub fn start_game(&mut self) {
        self.state.status = GameStatus::XTurn;
    }
    pub fn end_game() {}
    pub fn handle_player_turn() {}

    pub fn is_valid_move(&mut self, payload: &PlayerMove) -> Option<String> {
        let in_turn = self.get_player_in_turn();
        if payload.player_number != in_turn.player_number {
            return Some(format!(
                "Player {} tried to move when it was {} turn",
                payload.player_number, in_turn.player_number
            ));
        } else if !self.is_running() {
            return Some("Game has already ended".to_string());
        } else if !self.state.board.is_within_board(payload.x, payload.y) {
            return Some("Move's x, y weren't inside the board".to_string());
        }
        let cell = self.state.board.get_cell_at(payload.x, payload.y);
        if cell.owner != 0 {
            return Some(format!(
                "Cell at {} {} was already selected",
                cell.x, cell.y
            ));
        }
        None
    }

    pub fn handle_player_move(&mut self, payload: &PlayerMove) -> Option<String> {
        let err = self.is_valid_move(payload);
        if err.is_some() {
            return err;
        }
        self.state
            .board
            .update_cell_owner(payload.x, payload.y, payload.player_number);
        if self.state.player_in_turn == self.state.players.len() as u32 {
            self.state.player_in_turn = 1;
        } else {
            self.state.player_in_turn = self.state.player_in_turn + 1;
        }
        None
    }

    pub fn handle_player_join(&mut self, payload: &PlayerJoin) {
        self.state.add_player(payload)
    }
}
