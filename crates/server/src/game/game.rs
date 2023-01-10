// use tic_tac_5::{events::*, game_state::*};
use tic_tac_5::{events::ServerEvent, game_state::*, proto::proto_all::*};
use uuid::Uuid;

pub struct JoinedPlayer {
    pub player_id: u32,
    pub name: String,
    pub socket_id: u32,
}

pub struct Game {
    pub id: Uuid,
    pub state: GameState,
    pub joined_players: Vec<JoinedPlayer>,
}

impl Game {
    pub fn new(options: &GameOptions, rng_seed: Option<[u8; 32]>) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: GameState::new(options, rng_seed),
            joined_players: Vec::new(),
        }
    }
    pub fn allows_joining(&self) -> bool {
        self.state.status == GameStatus::WAITING
            && self.state.players.len() as u32 != self.state.options.players
    }
    pub fn matches_player_options(&self, user_options: &GameOptions) -> bool {
        true
    }
    pub fn is_running(&self) -> bool {
        self.state.status == GameStatus::X_TURN || self.state.status == GameStatus::O_TURN
    }
    pub fn is_waiting_and_empty(&self) -> bool {
        self.state.status == GameStatus::WAITING && self.joined_players.len() == 0
    }
    pub fn get_player_in_turn(&self) -> &Player {
        &self.state.players[self.state.player_in_turn as usize]
        // self.state.players.get(&self.state.player_in_turn).unwrap()
    }
    pub fn start_game(&mut self) {
        self.state.status = GameStatus::X_TURN;
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

    pub fn handle_player_join(&mut self, payload: &PlayerJoinGame, socket_id: u32) -> bool {
        self.joined_players.insert(
            0,
            JoinedPlayer {
                player_id: payload.player_id,
                name: payload.name.clone(),
                socket_id,
            },
        );
        if self.joined_players.len() as u32 == self.state.options.players {
            self.handle_game_start();
            return true;
        }
        false
    }
    pub fn handle_game_start(&mut self) {
        for player in &self.joined_players {
            self.state
                .add_player(&player.player_id, player.name.clone(), &player.socket_id);
        }
        self.state.status = GameStatus::X_TURN;
    }

    pub fn handle_player_disconnect(&mut self, player_id: &u32) {
        // TODO set disconnected & last connected time, remove later in game_loop if not reconnected before eg 15s timeout
        self.joined_players.retain(|p| p.player_id != *player_id);
        if self.state.status != GameStatus::WAITING {
            let player = self.state.get_player(*player_id);
            self.state.remove_player(player.player_number);
            let remaining = self.state.players.iter().filter(|p| !p.dead).count();
            if remaining == 1 {
                // TODO right player
                self.state.status = GameStatus::X_WON
                // TODO send winner maybe
            } else if remaining == 0 {
                self.state.status = GameStatus::TIE;
            }
        }
    }
}
