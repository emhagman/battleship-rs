use std::fmt;

use crate::player::Player;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    PartialShip,
    Hit,
    Miss,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match *self {
            Cell::Empty => "◻️",
            Cell::PartialShip => "⚓",
            Cell::Hit => "💣",
            Cell::Miss => "😞",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
    None,
}

#[derive(Debug)]
pub struct Board {
    pub cells: [[Cell; 10]; 10],
}

impl Board {
    pub fn default() -> Self {
        Self {
            cells: [[Cell::Empty; 10]; 10],
        }
    }
    pub fn print(&self) {
        for x in self.cells {
            for y in x {
                print!("{} ", y);
            }
            println!();
        }
    }
    // pub fn make_guess(&mut self, row: usize, col: usize, current_player: Player) {
    //     if self.cells[row][col] == Cell::PartialShip(current_player.inverse()) {
    //         self.cells[row][col] = Cell::Hit(current_player);
    //     } else {
    //         self.cells[row][col] = Cell::Miss(current_player);
    //     }
    // }
    // pub fn has_winner(&self, current_player: Player) -> bool {
    //     let mut count = 0;
    //     for x in &self.cells {
    //         for y in x {
    //             if *y == Cell::Hit(current_player) {
    //                 count += 1;
    //             }
    //         }
    //     }
    //     count == 2
    // }
}
