use tic_tac_5::proto::client_events::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JoinedPlayer {
    pub player_id: u32,
    pub name: String,
    pub socket_id: Option<u32>, // None if AI
    pub connected: bool,
    pub last_seen: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ListedGame {
    pub id: Uuid,
    pub joined_players: Vec<JoinedPlayer>,
    pub running: bool,
    pub options: GameOptions,
}

impl ListedGame {
    pub fn new(options: GameOptions) -> Self {
        Self {
            id: Uuid::new_v4(),
            joined_players: Vec::new(),
            running: false,
            options,
        }
    }
    pub fn allows_joining(&self) -> bool {
        !self.running && self.joined_players.len() != self.options.players as usize
    }
    pub fn matches_player_options(&self, user_options: &GameOptions) -> bool {
        true
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn is_empty(&self) -> bool {
        self.joined_players.len() == 0
    }
    pub fn handle_player_join(&mut self, payload: &PlayerJoinGame, socket_id: u32) -> bool {
        self.joined_players.insert(
            0,
            JoinedPlayer {
                player_id: payload.player_id,
                name: payload.name.clone(),
                socket_id: Some(socket_id),
                connected: true,
                last_seen: None,
            },
        );
        self.joined_players.len() == self.options.players as usize
    }

    pub fn handle_player_leave(&mut self, player_id: &u32) {
        // TODO set disconnected & last connected time, remove later in game_loop if not reconnected before eg 15s timeout
        self.joined_players.retain(|p| &p.player_id != player_id);
    }
}
