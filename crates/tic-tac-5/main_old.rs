pub mod board;
pub mod events;
pub mod game;
pub mod game_state;
mod terminal_ui;

use crate::terminal_ui::*;
// use terminal_ui::*;

fn main() {
    run_term_ui();
}
