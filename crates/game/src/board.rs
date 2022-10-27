use rand::{rngs::OsRng, rngs::StdRng, Rng, SeedableRng};
use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
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

#[derive(Debug, Clone)]
pub struct BoardCell {
    pub x: u32,
    pub y: u32,
    pub owner: u32,
    adjacency: Adjancies,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub size: u32,
    pub cells: HashMap<u32, BoardCell>,
}

#[derive(Debug, Clone)]
struct Adjancies {
    hor: u32,
    ver: u32,
    leftDiag: u32,
    rightDiag: u32,
}

impl Index<Adjacency> for Adjancies {
    type Output = u32;

    fn index(&self, dir: Adjacency) -> &Self::Output {
        match dir {
            Adjacency::Horizontal => &self.hor,
            Adjacency::Vertical => &self.ver,
            Adjacency::LeftToRightDiagonal => &self.leftDiag,
            Adjacency::RightToLeftDiagonal => &self.rightDiag,
        }
    }
}

impl IndexMut<Adjacency> for Adjancies {
    fn index_mut(&mut self, dir: Adjacency) -> &mut Self::Output {
        match dir {
            Adjacency::Horizontal => &mut self.hor,
            Adjacency::Vertical => &mut self.ver,
            Adjacency::LeftToRightDiagonal => &mut self.leftDiag,
            Adjacency::RightToLeftDiagonal => &mut self.rightDiag,
        }
    }
}

impl Board {
    pub fn new(size: u32) -> Self {
        let mut cells = HashMap::new();
        for x in 0..size {
            for y in 0..size {
                cells.insert(
                    x + y * size,
                    BoardCell {
                        x,
                        y,
                        owner: 0,
                        adjacency: Adjancies {
                            hor: 0,
                            ver: 0,
                            leftDiag: 0,
                            rightDiag: 0,
                        },
                    },
                );
            }
        }
        Self { size, cells }
    }

    pub fn is_within_board(&self, x: u32, y: u32) -> bool {
        x >= 0 && y >= 0 && x < self.size && y < self.size
    }

    pub fn get_cell_at(&self, x: u32, y: u32) -> &BoardCell {
        self.cells.get(&(y * self.size + x)).unwrap()
    }

    fn get_adjacent_in_direction(
        &self,
        x: u32,
        y: u32,
        dir: Adjacency,
        topside: bool,
    ) -> Option<&BoardCell> {
        let mut xx = x;
        let mut yy = y;
        match dir {
            Adjacency::Horizontal => {
                if topside {
                    xx = x + 1;
                } else {
                    xx = x - 1;
                }
            }
            Adjacency::Vertical => {
                if topside {
                    yy = y + 1;
                } else {
                    yy = y - 1;
                }
            }
            Adjacency::LeftToRightDiagonal => {
                if topside {
                    xx = x - 1;
                    yy = y + 1;
                } else {
                    xx = x + 1;
                    yy = y - 1;
                }
            }
            Adjacency::RightToLeftDiagonal => {
                if topside {
                    xx = x + 1;
                    yy = y + 1;
                } else {
                    xx = x - 1;
                    yy = y - 1;
                }
            }
        }
        if !self.is_within_board(xx, yy) {
            return None;
        }
        Some(self.get_cell_at(xx, yy))
    }

    fn get_adjacent_cells(&self, x: u32, y: u32, player: u32, dir: Adjacency) -> Vec<&BoardCell> {
        let mut adjacent = Vec::new();
        let mut topside = true;
        let mut now_x = x;
        let mut now_y = y;
        let mut iters = 0;
        loop {
            let mut cell = self.get_adjacent_in_direction(now_x, now_y, dir, topside);
            if iters > 20 {
                // println!("cell is {}", cell.unwrap_or("none".to_string()));
                println!("player {}", player);
                println!("topSide {}", topside);
                panic!("infinite loop");
            }
            if cell.is_some() && cell.unwrap().owner == player {
                adjacent.insert(0, cell.unwrap());
                now_x = cell.unwrap().x;
                now_y = cell.unwrap().y;
            } else if topside {
                cell = Some(self.get_cell_at(x, y));
                topside = false;
                now_x = x;
                now_y = y;
            } else {
                break;
            }
            iters += 1;
        }
        adjacent
    }

    fn update_cells_in_direction(&mut self, x: u32, y: u32, player: u32, dir: Adjacency) -> u32 {
        let cells = self.get_adjacent_cells(x, y, player, dir);
        let adjacent_count = cells.len() + 1;
        let cells2 = cells
            .iter()
            .map(|c| {
                let mut cc = (*c).clone();
                cc.adjacency[dir] = adjacent_count as u32;
                cc
            })
            .collect::<Vec<BoardCell>>();
        for cell in cells2 {
            self.cells.insert(x + y * self.size, cell);
        }
        adjacent_count as u32
    }

    pub fn update_cell_owner(&mut self, x: u32, y: u32, player: u32) {
        {
            let mut cell = self.cells.get_mut(&(y * self.size + x)).unwrap();
            cell.owner = player;
        }
        let mut adjancies = Adjancies {
            hor: 0,
            ver: 0,
            leftDiag: 0,
            rightDiag: 0,
        };
        for dir in Adjacency::iterator() {
            adjancies[*dir] = self.update_cells_in_direction(x, y, player, dir.clone());
        }
        let mut cell = self.cells.get_mut(&(y * self.size + x)).unwrap();
        cell.adjacency = adjancies;
    }
}
