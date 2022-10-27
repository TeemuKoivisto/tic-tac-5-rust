use rand::{rngs::OsRng, rngs::StdRng, Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

use crate::{board::Board, events::PlayerJoin};

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerSymbol {
    X,
    O,
}

impl std::fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlayerSymbol::X => write!(f, "X"),
            PlayerSymbol::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub id: u32,
    pub player_number: u32,
    pub symbol: PlayerSymbol,
    pub name: String,
}

#[derive(Debug, Clone, Copy)]
pub struct GameOptions {
    pub size: u32,
    pub players: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Waiting,
    XTurn,
    OTurn,
    XWon,
    OWon,
    Tie,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub status: GameStatus,
    pub options: GameOptions,
    pub players: HashMap<u32, Player>, // TODO: use PlayerNumber type
    pub player_in_turn: u32,
    // pub symbols: HashMap<u32, PlayerSymbol>,
    pub board: Board,
    pub rng: StdRng,
}

impl GameState {
    pub fn new(opts: Option<GameOptions>, rng_seed: Option<[u8; 32]>) -> Self {
        let rng;
        if rng_seed.is_some() {
            rng = StdRng::from_seed(rng_seed.unwrap());
        } else {
            rng = StdRng::from_seed(OsRng.gen());
        }
        let options;
        if opts.is_none() {
            options = GameOptions {
                players: 2,
                size: 25,
            };
        } else {
            options = opts.unwrap();
        }
        Self {
            status: GameStatus::Waiting,
            options,
            players: HashMap::new(),
            board: Board::new(options.clone().size),
            player_in_turn: 1,
            rng,
        }
    }
    pub fn add_player(&mut self, payload: &PlayerJoin) {
        let player_number = self.players.len() as u32 + 1;
        let symbol = if player_number == 1 {
            PlayerSymbol::X
        } else {
            PlayerSymbol::O
        };
        let player = Player {
            id: payload.player_id,
            player_number,
            symbol,
            name: payload.name.clone(),
        };
        self.players.insert(player.player_number, player);
    }
}
