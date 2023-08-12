use js_sys::Array;
use std::ops::{Index, IndexMut};
use std::slice::Iter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adjacency {
    Horizontal = 0,
    Vertical = 1,
    LeftToRightDiagonal = 2,
    RightToLeftDiagonal = 3,
}

impl Adjacency {
    pub fn iterator() -> Iter<'static, Adjacency> {
        static ADJANCIES: [Adjacency; 4] = [
            Adjacency::Horizontal,
            Adjacency::Vertical,
            Adjacency::LeftToRightDiagonal,
            Adjacency::RightToLeftDiagonal,
        ];
        ADJANCIES.iter()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardCell {
    pub x: u32,
    pub y: u32,
    pub owner: u32,
    adjacency: Adjancies,
}

#[wasm_bindgen]
pub struct Board {
    size: u32,
    available: u32,
    in_row: u32,
    cells: Vec<BoardCell>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Adjancies {
    hor: u32,
    ver: u32,
    left_diag: u32,
    right_diag: u32,
}

impl Index<&Adjacency> for Adjancies {
    type Output = u32;

    fn index(&self, dir: &Adjacency) -> &Self::Output {
        match dir {
            Adjacency::Horizontal => &self.hor,
            Adjacency::Vertical => &self.ver,
            Adjacency::LeftToRightDiagonal => &self.left_diag,
            Adjacency::RightToLeftDiagonal => &self.right_diag,
        }
    }
}

impl IndexMut<&Adjacency> for Adjancies {
    fn index_mut(&mut self, dir: &Adjacency) -> &mut Self::Output {
        match dir {
            Adjacency::Horizontal => &mut self.hor,
            Adjacency::Vertical => &mut self.ver,
            Adjacency::LeftToRightDiagonal => &mut self.left_diag,
            Adjacency::RightToLeftDiagonal => &mut self.right_diag,
        }
    }
}

impl Board {
    pub fn new(size: u32, in_row: u32) -> Self {
        let mut cells = Vec::new();
        for y in 0..size {
            for x in 0..size {
                cells.push(BoardCell {
                    x,
                    y,
                    owner: 0,
                    adjacency: Adjancies {
                        hor: 0,
                        ver: 0,
                        left_diag: 0,
                        right_diag: 0,
                    },
                });
            }
        }
        Self {
            size,
            available: size * size,
            in_row,
            cells,
        }
    }

    fn is_within_board(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.size as i32 && y < self.size as i32
    }

    // fn get_cell_at(&self, x: u32, y: u32) -> &BoardCell {
    //     &self.cells[(x + y * self.size) as usize]
    // }

    fn set_cell_adjacency(&mut self, x: u32, y: u32, dir: &Adjacency, count: u32) {
        self.cells[(x + y * self.size) as usize].adjacency[dir] = count;
    }

    // pub fn get_next_empty_cell(&self) -> Option<BoardCell> {
    //     if self.available == 0 {
    //       return None
    //     }
    //     let idx = 0;
    //     let len = self.cells.len();
    //     let cell = self.cells[idx];

    //     while cell.owner != 0 || len == idx {
    //       idx += 1;
    //       cell = self.cells[idx]
    //     }
    //     return cell
    //   }

    fn get_adjacent_in_direction(
        &self,
        x: u32,
        y: u32,
        dir: &Adjacency,
        topside: bool,
    ) -> Option<&BoardCell> {
        let mut xx = x as i32;
        let mut yy = y as i32;
        match dir {
            Adjacency::Horizontal => {
                if topside {
                    xx += 1;
                } else {
                    xx -= 1;
                }
            }
            Adjacency::Vertical => {
                if topside {
                    yy += 1;
                } else {
                    yy -= 1;
                }
            }
            Adjacency::LeftToRightDiagonal => {
                if topside {
                    xx += 1;
                    yy -= 1;
                } else {
                    xx -= 1;
                    yy += 1;
                }
            }
            Adjacency::RightToLeftDiagonal => {
                if topside {
                    xx -= 1;
                    yy -= 1;
                } else {
                    xx += 1;
                    yy += 1;
                }
            }
        }
        if !self.is_within_board(xx, yy) {
            return None;
        }
        Some(&self.cells[(xx as usize + yy as usize * self.size as usize)])
    }

    fn get_adjacent_cells(&self, x: u32, y: u32, player: u32, dir: &Adjacency) -> Vec<&BoardCell> {
        let mut adjacent = Vec::new();
        let mut topside = true;
        let mut now_x = x;
        let mut now_y = y;
        let mut iters = 0;
        loop {
            let mut cell = self.get_adjacent_in_direction(now_x, now_y, dir, topside);
            if cell.is_some() && cell.unwrap().owner == player {
                let c = cell.unwrap();
                adjacent.push(c);
                now_x = c.x;
                now_y = c.y;
                // log(&format!("cell: {:?}", c));
            } else if topside {
                cell = Some(&self.cells[(x + y * self.size) as usize]);
                // cell = Some(self.get_cell_at(x, y));
                topside = false;
                now_x = x;
                now_y = y;
            } else {
                break;
            }
            if iters > 20 {
                alert(&format!(
                    "Cell {:?} dir {:?} iters {iters} player {player} topside {topside} adj.len {}",
                    cell,
                    dir,
                    adjacent.len()
                ));
                panic!("infinite loop");
            }
            iters += 1;
        }
        adjacent
    }

    pub fn clone_adjacent_cells(
        &self,
        x: u32,
        y: u32,
        player: u32,
        dir: &Adjacency,
    ) -> Vec<BoardCell> {
        let ar = self.get_adjacent_cells(x, y, player, dir);
        ar.into_iter()
            .map(|c| {
                return BoardCell {
                    x: c.x,
                    y: c.y,
                    owner: c.owner,
                    adjacency: c.adjacency.clone(),
                };
            })
            .collect::<Vec<BoardCell>>()
    }
}

#[wasm_bindgen]
impl Board {
    pub fn is_full(&self) -> bool {
        self.available == 0
    }

    pub fn get_available_moves(&self) -> Array {
        self.cells
            .iter()
            .filter(|c| c.owner == 0)
            .map(|c| JsValue::from(c.clone()))
            .collect()
    }

    pub fn get_cell_at(&self, x: u32, y: u32) -> BoardCell {
        self.cells[(x + y * self.size) as usize].clone()
    }

    pub fn set_cell_owner(&mut self, x: u32, y: u32, player: u32) {
        self.cells[(x + y * self.size) as usize].owner = player;
        if player != 0 {
            self.available -= 1;
        } else {
            self.available += 1;
        }
    }

    pub fn update_cell_adjancies(&mut self, x: u32, y: u32, player: u32) -> bool {
        let mut best_in_row = 0;
        for dir in Adjacency::iterator() {
            let cells = self.clone_adjacent_cells(x, y, player, dir);
            let adjacent_count = (cells.len() + 1) as u32;
            for c in cells {
                self.cells[(c.x + c.y * self.size) as usize].adjacency[dir] = adjacent_count;
            }
            if adjacent_count > best_in_row {
                best_in_row = adjacent_count;
            }
            self.cells[(x + y * self.size) as usize].adjacency[dir] = adjacent_count;
        }
        self.in_row == best_in_row
    }
}

#[wasm_bindgen]
pub fn create_board(size: u32, in_row: u32) -> Board {
    Board::new(size, in_row)
}
