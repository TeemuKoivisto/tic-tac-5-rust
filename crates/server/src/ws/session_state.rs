use tic_tac_5::proto::{client_events::PlayerJoinLobby, server_events::*};
use tokio::sync::{broadcast, mpsc};

use crate::state::{
    client::Client,
    events::{
        ClientToGameEvent, ClientToLobbyEvent, GameToClientEvent, LobbyToClientEvent, PlayerMove,
    },
};

pub struct SubscribedGame {
    game_id: String,
    pub sender: broadcast::Sender<ClientToGameEvent>,
}

pub struct SessionState {
    pub player_id: u32,
    pub name: String,
    pub socket_id: u32,
    pub app_state: PlayerAppState,
    pub prev_app_state: PlayerAppState,
    pub game_state: PlayerInGameState,
    pub prev_game_state: PlayerInGameState,
    pub subscribed_lobby: Option<broadcast::Sender<ClientToLobbyEvent>>,
    pub subscribed_games: Vec<SubscribedGame>,
}

fn is_valid_app_transition(from: &PlayerAppState, to: &PlayerAppState) -> bool {
    match from {
        PlayerAppState::initializing => {
            [PlayerAppState::disconnected, PlayerAppState::lobby].contains(to)
        }
        PlayerAppState::disconnected => [
            PlayerAppState::initializing,
            PlayerAppState::lobby,
            PlayerAppState::waiting_game_start,
            PlayerAppState::in_game,
            PlayerAppState::errored,
        ]
        .contains(to),
        PlayerAppState::lobby => [
            PlayerAppState::disconnected,
            PlayerAppState::lobby,
            PlayerAppState::waiting_game_start,
            PlayerAppState::in_game,
        ]
        .contains(to),
        PlayerAppState::waiting_game_start => [
            PlayerAppState::disconnected,
            PlayerAppState::lobby,
            PlayerAppState::in_game,
        ]
        .contains(to),
        PlayerAppState::in_game => {
            [PlayerAppState::disconnected, PlayerAppState::lobby].contains(to)
        }
        PlayerAppState::errored => [].contains(to),
    }
}

fn is_valid_game_transition(from: &PlayerInGameState, to: &PlayerInGameState) -> bool {
    match from {
        PlayerInGameState::not_started => [
            PlayerInGameState::x_turn,
            PlayerInGameState::o_turn,
            PlayerInGameState::waiting_player,
            PlayerInGameState::paused,
            PlayerInGameState::ended,
        ]
        .contains(to),
        PlayerInGameState::x_turn => [
            PlayerInGameState::waiting_player,
            PlayerInGameState::o_turn,
            PlayerInGameState::paused,
            PlayerInGameState::ended,
        ]
        .contains(to),
        PlayerInGameState::o_turn => [
            PlayerInGameState::waiting_player,
            PlayerInGameState::x_turn,
            PlayerInGameState::paused,
            PlayerInGameState::ended,
        ]
        .contains(to),
        PlayerInGameState::waiting_player => [
            PlayerInGameState::not_started,
            PlayerInGameState::x_turn,
            PlayerInGameState::o_turn,
            PlayerInGameState::paused,
            PlayerInGameState::ended,
        ]
        .contains(to),
        PlayerInGameState::paused => [
            PlayerInGameState::waiting_player,
            PlayerInGameState::not_started,
            PlayerInGameState::x_turn,
            PlayerInGameState::o_turn,
            PlayerInGameState::ended,
        ]
        .contains(to),
        PlayerInGameState::ended => [].contains(to),
    }
}

impl SessionState {
    pub fn new(socket_id: u32) -> Self {
        Self {
            name: "".to_string(),
            player_id: 0,
            socket_id,
            app_state: PlayerAppState::initializing,
            prev_app_state: PlayerAppState::initializing,
            game_state: PlayerInGameState::not_started,
            prev_game_state: PlayerInGameState::not_started,
            subscribed_lobby: None,
            subscribed_games: Vec::new(),
        }
    }

    pub fn get_client(&self) -> Client {
        Client {
            name: self.name.clone(),
            player_id: self.player_id,
            socket_id: self.socket_id,
        }
    }

    pub fn get_player_state(&self) -> PlayerState {
        PlayerState {
            app_state: self.app_state,
            game_state: self.game_state,
            waiting_games: self.get_game_ids(),
            ended_games: Vec::new(),
        }
    }

    pub fn get_game_ids(&self) -> Vec<String> {
        self.subscribed_games
            .iter()
            .map(|g| g.game_id.clone())
            .collect()
    }

    pub fn set_player(&mut self, player_join: &PlayerJoinLobby) {
        self.name = player_join.name.clone();
        self.player_id = player_join.player_id;
    }

    pub fn set_lobby(&mut self, sender: broadcast::Sender<ClientToLobbyEvent>) {
        self.subscribed_lobby = Some(sender);
    }

    pub fn push_game(
        &mut self,
        game_id: String,
        client_sender: broadcast::Sender<ClientToGameEvent>,
    ) {
        self.subscribed_games.push(SubscribedGame {
            game_id,
            sender: client_sender,
        });
    }

    pub fn revert_disconnected(&mut self) {
        if self.app_state != PlayerAppState::disconnected {
            panic!(
                "Session was not in disconnected state to revert it to {:?}",
                self.prev_app_state
            );
        }
        self.app_state = self.prev_app_state;
    }

    pub fn transit(&mut self, to: PlayerAppState) {
        if !is_valid_app_transition(&self.app_state, &to) {
            panic!(
                "Not valid app transition: from {:?} to {:?}",
                self.app_state, to
            );
        }
        self.prev_app_state = self.app_state;
        self.app_state = to;
    }

    pub fn transit_game(&mut self, to: PlayerInGameState) {
        // if !is_valid_game_transition(&self.game_state, &to) {
        //     panic!(
        //         "Not valid game transition: from {:?} to {:?}",
        //         self.game_state, to
        //     );
        // }
        self.prev_game_state = self.game_state;
        self.game_state = to;
    }
}
