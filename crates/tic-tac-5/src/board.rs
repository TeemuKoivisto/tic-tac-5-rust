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
    pub adjacency: Adjancies,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub size: u32,
    pub cells: Vec<BoardCell>,
}

#[derive(Debug, Clone)]
pub struct Adjancies {
    hor: u32,
    ver: u32,
    left_diag: u32,
    right_diag: u32,
}

impl Index<Adjacency> for Adjancies {
    type Output = u32;

    fn index(&self, dir: Adjacency) -> &Self::Output {
        match dir {
            Adjacency::Horizontal => &self.hor,
            Adjacency::Vertical => &self.ver,
            Adjacency::LeftToRightDiagonal => &self.left_diag,
            Adjacency::RightToLeftDiagonal => &self.right_diag,
        }
    }
}

impl IndexMut<Adjacency> for Adjancies {
    fn index_mut(&mut self, dir: Adjacency) -> &mut Self::Output {
        match dir {
            Adjacency::Horizontal => &mut self.hor,
            Adjacency::Vertical => &mut self.ver,
            Adjacency::LeftToRightDiagonal => &mut self.left_diag,
            Adjacency::RightToLeftDiagonal => &mut self.right_diag,
        }
    }
}

impl Board {
    pub fn new(size: u32) -> Self {
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
        Self { size, cells }
    }

    pub fn is_within_board(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.size as i32 && y < self.size as i32
    }

    pub fn get_cell_at(&self, x: u32, y: u32) -> &BoardCell {
        &self.cells[(x + y * self.size) as usize]
    }

    pub fn set_cell_owner(&mut self, x: u32, y: u32, player: u32) {
        self.cells[(x + y * self.size) as usize].owner = player;
    }

    pub fn set_cell_adjacency(&mut self, x: u32, y: u32, dir: Adjacency, count: u32) {
        self.cells[(x + y * self.size) as usize].adjacency[dir] = count;
    }

    fn get_adjacent_in_direction(
        &self,
        x: u32,
        y: u32,
        dir: Adjacency,
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
        Some(self.get_cell_at(xx as u32, yy as u32))
    }

    pub fn get_adjacent_cells(
        &self,
        x: u32,
        y: u32,
        player: u32,
        dir: Adjacency,
    ) -> Vec<&BoardCell> {
        let mut adjacent = Vec::new();
        let mut topside = true;
        let mut now_x = x;
        let mut now_y = y;
        let mut iters = 0;
        loop {
            let mut cell = self.get_adjacent_in_direction(now_x, now_y, dir, topside);
            if iters > 20 {
                println!("hey cell {:?} in dir {:?}", cell, dir);
                println!("player {}", player);
                println!("topSide {}", topside);
                panic!("infinite loop");
            }
            if cell.is_some() && cell.unwrap().owner == player {
                adjacent.push(cell.unwrap());
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

    pub fn clone_adjacent_cells(
        &self,
        x: u32,
        y: u32,
        player: u32,
        dir: Adjacency,
    ) -> Vec<BoardCell> {
        self.get_adjacent_cells(x, y, player, dir)
            .into_iter()
            .map(|c| BoardCell {
                x: c.x,
                y: c.y,
                owner: c.owner,
                adjacency: c.adjacency.clone(),
            })
            .collect::<Vec<BoardCell>>()
    }

    pub fn update_cell_owner(&mut self, x: u32, y: u32, player: u32) {
        self.set_cell_owner(x, y, player);
        let mut adjancies = Adjancies {
            hor: 0,
            ver: 0,
            left_diag: 0,
            right_diag: 0,
        };
        for dir in Adjacency::iterator() {
            let cells = self.clone_adjacent_cells(x, y, player, *dir);
            let adjacent_count = (cells.len() + 1) as u32;
            for c in cells {
                self.cells[(c.x + c.y * self.size) as usize].adjacency[*dir] = adjacent_count;
            }
            adjancies[*dir] = adjacent_count;
        }
        self.cells[(x + y * self.size) as usize].adjacency = adjancies;
    }
}
