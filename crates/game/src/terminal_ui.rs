use crate::board::*;
use crate::events::*;
use crate::game::*;
use crate::game_state::*;

use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn run_term_ui() {
    println!("### TicTac5 ###");
    let local_player_id = 1;
    let mut game = Game::new(None, None);
    game.handle_player_join(&PlayerJoin {
        game_id: game.id.to_string(),
        player_id: local_player_id,
        name: "Bob".to_string(),
    });
    game.handle_player_join(&PlayerJoin {
        game_id: game.id.to_string(),
        player_id: 2,
        name: "Urho".to_string(),
    });
    game.start_game();
    loop {
        if !game.is_running() {
            break;
        }
        print_board(&game.state.board, &game.state.players);
        // if game.state.status
        let input = prompt_move();
        if input.is_some() {
            let payload = PlayerMove {
                game_id: game.id.to_string(),
                player_number: game.state.player_in_turn,
                x: input.unwrap()[0],
                y: input.unwrap()[1],
            };
            game.handle_player_move(&payload);
        }
    }
    println!("### GAME ENDED ###");
}

fn print_board(board: &Board, players: &HashMap<u32, Player>) {
    for x in 0..board.size {
        for y in 0..board.size {
            let cell = board.get_cell_at(x, y);
            if cell.owner == 0 {
                print!(" |")
            } else {
                print!("{}|", players.get(&cell.owner).unwrap().symbol);
            }
        }
        println!();
    }
}

fn prompt_move() -> Option<[u32; 2]> {
    println!("Enter x,y coordinates separated by space eg: 0 1");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let lines: Vec<u32> = line
        .split(" ")
        .map(|val| val.trim().parse::<u32>())
        .filter(|val| val.is_ok())
        .map(|val| val.unwrap())
        .collect();
    if lines.len() < 2 {
        println!("Please provide two integeres separated by space");
        return None;
    }
    Some([*lines.get(0).unwrap(), *lines.get(1).unwrap()])
}
