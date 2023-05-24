use log::debug;
use uuid::Uuid;

use crate::game::game::Game;

use std::collections::HashMap;
use std::sync::Arc;
use tic_tac_5::proto::proto_all::*;
use tokio::sync::Mutex;

pub struct GameManager {
    pub games: HashMap<Uuid, Arc<Mutex<Game>>>,
}

impl GameManager {
    pub fn new() -> GameManager {
        Self {
            games: HashMap::new(),
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
