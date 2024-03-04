use std::collections::HashSet;

use wasm_bindgen::prelude::*;

use crate::board::Board;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct EmptyCells {
    size: u32,
    start: u32,
    // end: u32, // ?
    board: u128,
    cells: Vec<(u32, u32)>,
    mapping: HashSet<usize>
}

impl EmptyCells {
  pub fn new(board: &Board) -> Self {
    let mut val = 0 as u128;
    let mut i = 0;
    let mut start = 0;
    let mut cells = Vec::new();
    for c in board.clone_cells() {
      if c.owner == 0 {
        if start == 0 && i > 0 {
          start = i;
        }
        val = val << 1;
        cells.push((c.x, c.y));
      } else {
        val = val << 0;
      }
      i += 1;
    }
    Self {
      size: i / 2,
      start,
      cells,
      mapping: HashSet::new(),
      board: val
    }
  }

  pub fn copy_with_cell_selected(&self, x: u32, y: u32, empty: bool) -> Vec<usize> {
    let mut created = Vec::with_capacity(self.mapping.len());
    for key in self.mapping.into_iter() {
      if empty || x + y * self.size != key as u32 {
        created.push(key);
      }
    }
    created
  }

  pub fn create_empty_vec(&self) -> Vec<usize> {
    let mut created = Vec::with_capacity(self.mapping.len());
    for key in self.mapping.into_iter() {
      created.push(key);
    }
    created
  }

  pub fn set_empty(&mut self, x: u32, y: u32, player: u32) {
    let idx = (x + y * self.size) as usize;
    if player == 0 {
      self.mapping.insert(idx);
    } else {
      self.mapping.remove(&idx);
    }
  }
}
