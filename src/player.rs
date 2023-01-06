use crate::board::{Board, Direction};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PlayerType {
    Human = 0,
    Computer = 1,
}

impl PlayerType {
    pub fn inverse(&self) -> PlayerType {
        if *self == PlayerType::Human {
            PlayerType::Computer
        } else {
            PlayerType::Human
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub t: PlayerType,
    pub board: Board,
}

impl Player {
    pub fn default() -> Self {
        Self {
            t: PlayerType::Human,
            board: Board::default(),
        }
    }
    pub fn new(player_type: PlayerType) -> Self {
        Self {
            t: player_type,
            board: Board::default(),
        }
    }
    // pub fn collect_input_location(&self, need_direction: bool) -> (usize, usize, Direction) {
    //     if *self.t == Player::Computer {
    //         let guess_row: usize = rand::random::<usize>() % 10;
    //         let guess_col: usize = rand::random::<usize>() % 10;
    //         let direction = if rand::random() {
    //             Direction::Horizontal
    //         } else {
    //             Direction::Vertical
    //         };
    //         println!("{:?}, {:?}, {:?}", guess_row, guess_col, direction);
    //         (guess_row, guess_col, direction)
    //     } else {
    //         let mut guess = String::new();
    //         std::io::stdin().read_line(&mut guess).unwrap();
    //         let guess_list: Vec<usize> = guess
    //             .trim()
    //             .split(",")
    //             .map(|f| f.parse().unwrap())
    //             .collect();
    //         if need_direction {
    //             let direction = if guess_list[2] == 1 {
    //                 Direction::Horizontal
    //             } else {
    //                 Direction::Vertical
    //             };
    //             (guess_list[0], guess_list[1], direction)
    //         } else {
    //             (guess_list[0], guess_list[1], Direction::None)
    //         }
    //     }
    // }
}
