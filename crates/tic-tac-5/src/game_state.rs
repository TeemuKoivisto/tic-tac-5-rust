use rand::{rngs::OsRng, rngs::StdRng, Rng, SeedableRng};
use std::collections::{HashMap, HashSet};

use crate::board::{Adjacency, Adjancies, Board};
use crate::proto::client_events::*;
use crate::proto::game::*;

// impl std::fmt::Display for PlayerSymbol {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             PlayerSymbol::X => write!(f, "X"),
//             PlayerSymbol::O => write!(f, "O"),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct GameState {
    pub status: GameStatus,
    pub options: GameOptions,
    pub players: Vec<Player>,
    pub player_in_turn: u32,
    pub board: Board,
    pub rng: StdRng,
}

impl GameState {
    pub fn new(opts: &GameOptions, rng_seed: Option<[u8; 32]>) -> Self {
        let rng;
        if rng_seed.is_some() {
            rng = StdRng::from_seed(rng_seed.unwrap());
        } else {
            rng = StdRng::from_seed(OsRng.gen());
        }
        let options = GameOptions {
            size: if opts.size != 0 { opts.size } else { 25 },
            players: if opts.players != 0 {
                opts.players.min(8)
            } else {
                2
            },
        };
        let size = options.size;
        Self {
            status: GameStatus::WAITING,
            options,
            players: Vec::new(),
            board: Board::new(size),
            player_in_turn: 1,
            rng,
        }
    }
    pub fn add_player(&mut self, player_id: &u32, name: String, socket_id: Option<u32>) {
        let player_number = self.players.len() as u32 + 1;
        let symbol = if player_number == 1 {
            "X".to_string()
        } else {
            "O".to_string()
        };
        let player = Player {
            id: *player_id,
            socket_id: socket_id.unwrap_or(0),
            player_number,
            symbol,
            name,
            dead: false,
            ai: false,
        };
        self.players.push(player);
    }
    pub fn get_player(&self, player_id: u32) -> &Player {
        self.players.iter().find(|p| p.id == player_id).unwrap()
    }
    pub fn remove_player(&mut self, player_number: u32) {
        self.players[(player_number - 1) as usize].dead = true;
    }
    pub fn player_move(&mut self, x: u32, y: u32, player: u32) {
        self.board.update_cell_owner(x, y, player);
    }
    pub fn check_win(&self, x: u32, y: u32) -> bool {
        let cell = self.board.get_cell_at(x, y);
        let mut won = false;
        for dir in Adjacency::iterator() {
            won = won || cell.adjacency[*dir] == 5;
        }
        won
    }
    pub fn get_cells(&self) -> Vec<Cell> {
        self.board
            .cells
            .iter()
            .map(|c| Cell {
                x: c.x,
                y: c.y,
                cell_type: if c.owner == 0 {
                    CellType::EMPTY
                } else {
                    CellType::PLAYER_CELL
                },
                player: c.owner,
            })
            .collect::<Vec<Cell>>()
    }
}
