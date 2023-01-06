mod board;
mod player;

use board::*;
use player::*;
use rand::prelude::*;

// Before
// One board, two "players", but they didn't have state
// The board had the state only, enum to switch which cell go put into board

// After
// Two players
// Each player has a board in it

fn main() {
    // 1. Make 10 X 10 board
    // 2. Mark a ship of size 2
    // 3. Make a guess
    // 4. Mark hit or miss
    // 5. Check winner
    // 6. Repeat 3 - 5

    // NOTE: DO NOT MODIFY THE ORDER OF THIS ARRAY
    let mut players = [
        Player::new(PlayerType::Human),
        Player::new(PlayerType::Computer),
    ];

    // Pick your pieces
    let mut current_player = PlayerType::Human;
    for _ in 0..2 {
        current_player = current_player.inverse();
        let player = &mut players[current_player as usize];
        for ship_size in 2..3 {
            println!(
                "{:?} Place your {} piece ship, enter R,C,D: ",
                player, ship_size
            );
            // let (row, col, dir) = current_player.collect_input_location(true);
            // for i in 0..ship_size {
            //     if i == 0 {
            //         board.cells[row][col] = Cell::PartialShip(current_player);
            //     } else if dir == Direction::Horizontal {
            //         board.cells[row][col + i] = Cell::PartialShip(current_player);
            //     } else if dir == Direction::Vertical {
            //         board.cells[row + i][col] = Cell::PartialShip(current_player);
            //     }
            // }
        }
    }

    // board.print();

    // // Guess where they are
    // loop {
    //     let guess_list = current_player.collect_input_location(false);
    //     board.make_guess(guess_list.0, guess_list.1, current_player);
    //     board.print();
    //     if board.has_winner(current_player) {
    //         println!(
    //             "Winner! {:?} : {}",
    //             current_player,
    //             board.has_winner(current_player)
    //         );
    //         break;
    //     }
    //     current_player = current_player.inverse();
    // }
    // board.print();
}
