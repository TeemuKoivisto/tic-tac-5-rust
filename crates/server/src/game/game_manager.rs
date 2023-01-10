use log::{debug, error};
use uuid::Uuid;

use crate::game::game::Game;

use std::collections::HashMap;
use std::sync::Arc;
use tic_tac_5::{
    events::{GameEvent, ServerEvent},
    proto::proto_all::*,
};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{self, Receiver, Sender, UnboundedSender};
use tokio::sync::Mutex;

pub struct GameManager {
    pub broadcast: UnboundedSender<ServerEvent>,
    pub games: HashMap<Uuid, Arc<Mutex<Game>>>,
    pub lobby_players: Vec<LobbyPlayer>,
    pub lobby_chat: Vec<String>,
}

impl GameManager {
    pub fn new(broadcast: UnboundedSender<ServerEvent>) -> GameManager {
        Self {
            broadcast,
            games: HashMap::new(),
            lobby_players: Vec::new(),
            lobby_chat: Vec::new(),
        }
    }

    pub async fn lobby_state(&self) -> Vec<LobbyGame> {
        let mut games = Vec::new();
        for g in self.games.values() {
            let game = g.lock().await;
            games.insert(
                0,
                LobbyGame {
                    game_id: game.id.to_string(),
                    players: game.joined_players.len() as u32,
                    max_players: game.state.options.players,
                },
            );
        }
        games
    }

    pub fn player_join_lobby(&mut self, data: PlayerJoinLobby) {
        self.lobby_players.insert(
            0,
            LobbyPlayer {
                player_id: data.player_id,
                name: data.name,
            },
        );
    }

    pub fn player_leave_lobby(&mut self, player_id: u32) {
        self.lobby_players.retain(|p| p.player_id != player_id);
    }

    pub fn remove_game(&mut self, game_id: Uuid) {
        self.games.remove(&game_id);
    }

    pub fn get_game(&self, game_id: &Uuid) -> Option<&Arc<Mutex<Game>>> {
        self.games.get(game_id)
    }

    pub async fn find_game(&self, user_options: &GameOptions) -> Option<Arc<Mutex<Game>>> {
        for g in self.games.values() {
            let game = g.lock().await;
            if game.allows_joining() && game.matches_player_options(user_options) {
                debug!("Joining existing game {:?}", game.id);
                return Some(g.clone());
            }
        }
        None
    }

    pub async fn create_game(&mut self, user_options: &GameOptions) -> Arc<Mutex<Game>> {
        let game = Arc::new(Mutex::new(Game::new(&user_options, None)));
        let game_id = game.lock().await.id;
        self.games.insert(game_id, game.clone());
        debug!("Create new game id: {:?}", game_id);
        game
    }

    pub async fn find_or_create_game(&mut self, user_options: &GameOptions) -> Arc<Mutex<Game>> {
        let found_game = self.find_game(user_options).await;
        if found_game.is_none() {
            self.create_game(user_options).await
        } else {
            found_game.unwrap()
        }
    }
}
