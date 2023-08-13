use std::cmp::{max, min};
use wasm_bindgen::prelude::*;

use crate::board::Board;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn compute_ai(board: &mut Board, ai_number: u32, search_depth: i32) -> bool {
    let mut chosen = false;
    let mut x = 0;
    let mut y = 0;
    let mut best_value = -10000000;
    let human_player = if ai_number == 1 { 2 } else { 1 };
    let perf = web_sys::window().unwrap().performance().unwrap();

    let t0 = perf.now();
    let empty = board.get_empty_indices();
    for (cx, cy) in empty.clone() {
        let value = minimax(
            cx,
            cy,
            board,
            empty.clone(),
            search_depth,
            false,
            -10000000,
            10000000,
            ai_number,
            human_player,
        );
        board.set_cell_owner(&cx, &cy, 0);
        if value > best_value {
            chosen = true;
            x = cx;
            y = cy;
            best_value = value;
        }
    }
    let t1 = perf.now();
    log(&format!("ai took: {} ms", (t1 - t0).floor()));
    if !chosen {
        panic!("no ai move found");
    }
    log(&format!("best: {} {} {}", x, y, best_value));
    board.set_cell_owner(&x, &y, ai_number);
    board.update_cell_adjancies(x, y, ai_number)
}

pub fn minimax(
    x: u32,
    y: u32,
    board: &mut Board,
    empty_cells: Vec<(u32, u32)>,
    depth: i32,
    is_maximizing: bool,
    alpha: i32,
    beta: i32,
    player: u32,
    human_player: u32,
) -> i32 {
    board.set_cell_owner(&x, &y, player);
    let won = board.update_cell_adjancies(x, y, player);
    let mut value: i32 = 0;
    let mut ended = true;
    if won {
        value = if human_player == player {
            -1000 - depth
        } else {
            1000 + depth
        };
    } else if board.is_full() {
        value = if human_player == player {
            -100 - depth
        } else {
            100 + depth
        };
    } else if depth == 0 {
        value = 0;
    } else {
        ended = false;
    }
    if ended {
        return value;
    }
    if is_maximizing {
        value = -10000000;
        let mut alph = alpha;
        let pl = if player == 2 { 1 } else { 2 };
        let empty: Vec<(u32, u32)> = empty_cells
            .into_iter()
            .filter(|(cx, cy)| cx != &x || cy != &y)
            .collect();
        for (cx, cy) in &empty {
            value = max(
                value,
                minimax(
                    *cx,
                    *cy,
                    board,
                    empty.clone(),
                    depth - 1,
                    false,
                    alph,
                    beta,
                    pl,
                    human_player,
                ),
            );
            alph = max(alph, value);
            board.set_cell_owner(cx, cy, 0);
            if beta <= alpha {
                break;
            }
        }
    } else {
        value = 10000000;
        let mut bet = beta;
        let pl = if player == 2 { 1 } else { 2 };
        let empty: Vec<(u32, u32)> = empty_cells
            .into_iter()
            .filter(|(cx, cy)| cx != &x || cy != &y)
            .collect();
        for (cx, cy) in &empty {
            value = min(
                value,
                minimax(
                    *cx,
                    *cy,
                    board,
                    empty.clone(),
                    depth - 1,
                    true,
                    alpha,
                    bet,
                    pl,
                    human_player,
                ),
            );
            bet = min(bet, value);
            board.set_cell_owner(cx, cy, 0);
            if beta <= alpha {
                break;
            }
        }
    }
    value
}
