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
    pub turns_elapsed: u32,
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
            turns_elapsed: 0,
            rng,
        }
    }
    pub fn add_player(&mut self, player_id: &u32, name: String, socket_id: Option<u32>) -> bool {
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
        let full = self.players.len() == self.options.players as usize;
        if full {
            self.status = GameStatus::X_TURN;
        }
        full
    }
    pub fn get_player(&self, player_id: u32) -> &Player {
        self.players.iter().find(|p| p.id == player_id).unwrap()
    }
    pub fn remove_player(&mut self, player_number: u32) {
        self.players[(player_number - 1) as usize].dead = true;
    }
    pub fn is_valid_move(&self, x: u32, y: u32, player_number: u32) -> Option<String> {
        if self.status != GameStatus::O_TURN && self.status != GameStatus::X_TURN {
            return Some(format!("Game is not running: {:?}", self.status));
        } else if player_number != self.player_in_turn {
            return Some(format!(
                "Player {} tried to move when it was {} turn",
                player_number, self.player_in_turn
            ));
        } else if !self.board.is_within_board(x as i32, y as i32) {
            return Some("Move's x, y weren't inside the board".to_string());
        }
        let cell = self.board.get_cell_at(x, y);
        if cell.owner != 0 {
            return Some(format!(
                "Cell at {} {} was already selected",
                cell.x, cell.y
            ));
        }
        None
    }

    pub fn handle_player_move(
        &mut self,
        x: u32,
        y: u32,
        player_number: u32,
    ) -> Result<(bool, u32), Box<dyn std::error::Error>> {
        let is_valid_err = self.is_valid_move(x, y, player_number);
        if is_valid_err.is_some() {
            return Err(is_valid_err.unwrap().into());
        }
        self.board.update_cell_owner(x, y, player_number);
        self.turns_elapsed += 1;
        let did_win = self.check_win(x, y);
        if player_number == 2 {
            self.player_in_turn = 1;
            self.status = if did_win {
                GameStatus::O_WON
            } else {
                GameStatus::X_TURN
            };
        } else {
            self.player_in_turn = player_number + 1;
            self.status = if did_win {
                GameStatus::X_WON
            } else {
                GameStatus::O_TURN
            };
        }
        Ok((did_win, self.player_in_turn))
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

#[cfg(test)]
mod tests {
    use crate::{
        game_state::GameState,
        proto::{
            client_events::GameOptions,
            game::{GameStatus, Player},
        },
    };
    use rand::{rngs::OsRng, Rng};

    #[test]
    fn test_basic_game() {
        let opts = GameOptions {
            size: 25,
            players: 2,
        };
        let rng_seed = OsRng.gen();
        let mut game = GameState::new(&opts, rng_seed);
        let player1 = Player {
            id: 100,
            socket_id: 0,
            player_number: 1,
            symbol: "X".to_string(),
            name: "Player 1".to_string(),
            dead: false,
            ai: false,
        };
        let player2 = Player {
            id: 200,
            socket_id: 1,
            player_number: 2,
            symbol: "O".to_string(),
            name: "Player 2".to_string(),
            dead: false,
            ai: false,
        };
        game.add_player(&player1.id, player1.name.clone(), Some(player1.socket_id));
        assert_eq!(game.status, GameStatus::WAITING);
        let full = game.add_player(&player2.id, player2.name.clone(), Some(player2.socket_id));
        assert_eq!(full, true);
        assert_eq!(game.status, GameStatus::X_TURN);
        assert_eq!(game.get_player(player1.id), &player1);
        assert_eq!(game.get_player(player2.id), &player2);
        let moves: Vec<(u32, u32, u32)> = vec![
            (0, 0, 1),
            (10, 0, 2),
            (11, 0, 2),
            (0, 0, 1),
            (11, 0, 2),
            (2, 2, 1),
            (12, 0, 2),
            (1, 1, 1),
            (13, 0, 2),
            (4, 4, 1),
            (0, 0, 2),
            (14, 0, 2),
            (3, 3, 1),
            (11, 0, 2),
        ];
        for (i, (x, y, player_number)) in moves.iter().enumerate() {
            let _ = game.handle_player_move(*x, *y, *player_number);
            match i {
                1 | 2 | 3 | 4 | 6 | 8 | 11 => assert_eq!(
                    game.status,
                    GameStatus::X_TURN,
                    "Move (i {}) wasn't X_TURN but {:?}",
                    i,
                    game.status
                ),
                0 | 5 | 7 | 9 | 10 => assert_eq!(
                    game.status,
                    GameStatus::O_TURN,
                    "Move (i {}) wasn't O_TURN but {:?}",
                    i,
                    game.status
                ),
                12 | 13 => assert_eq!(
                    game.status,
                    GameStatus::X_WON,
                    "Move (i {}) wasn't X_WON but {:?}",
                    i,
                    game.status
                ),
                _ => {}
            }
        }
        assert_eq!(game.status, GameStatus::X_WON);
        assert_eq!(game.get_player(player1.id), &player1);
        assert_eq!(game.get_player(player2.id), &player2);
        assert_eq!(game.turns_elapsed, 9);
    }
}
