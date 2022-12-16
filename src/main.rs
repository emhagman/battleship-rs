use rand::prelude::*;

struct Board {
    cells: [[Cell; 10]; 10],
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    PartialShip(Player),
    Hit(Player),
    Miss(Player),
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
    Human,
    Computer,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Horizontal,
    Vertical,
    None
}
impl Player {
    fn inverse(&self) -> Player {
        if *self == Player::Computer {
            Player::Human
        } else {
            Player::Computer
        }
    }
    fn collect_input_location(&self, need_direction: bool) -> (usize, usize, Direction) {
        if *self == Player::Computer {
            let guess_row: usize = rand::random::<usize>() % 10;
            let guess_col: usize = rand::random::<usize>() % 10;
            let direction = if rand::random() { Direction::Horizontal } else { Direction::Vertical };
            println!("{:?}, {:?}, {:?}", guess_row, guess_col, direction);
            (guess_row, guess_col, direction)
        } else {
            let mut guess = String::new(); 
            std::io::stdin().read_line(&mut guess).unwrap();
            let guess_list: Vec<usize> = guess.trim().split(",").map(|f|f.parse().unwrap()).collect();
            if need_direction {
                let direction = if guess_list[2] == 1 { Direction::Horizontal } else { Direction::Vertical };
                (guess_list[0], guess_list[1], direction)
            } else {
                (guess_list[0], guess_list[1], Direction::None)
            }
        }
    }
}

impl Board {
    fn print(&self){
        for x in self.cells {
            for y in x {
                print!("{:?} ", y);
            }
            println!();
        }
    }
    fn make_guess(&mut self, row: usize, col: usize, current_player: Player) {
        if self.cells[row][col] == Cell::PartialShip(current_player.inverse()) {
            self.cells[row][col] = Cell::Hit(current_player);
        }
        else {
            self.cells[row][col] = Cell::Miss(current_player);
        }
    }
    fn has_winner(&self, current_player: Player) -> bool {
        let mut count = 0;
        for x in &self.cells {
            for y in x {
                if *y == Cell::Hit(current_player) {
                    count += 1;
                }
            }
        }
        count == 2
    }
}

fn main() {
    println!("Hello, world!");
    // 1. Make 10 X 10 board
    // 2. Mark a ship of size 2
    // 3. Make a guess
    // 4. Mark hit or miss
    // 5. Check winner
    // 6. Repeat 3 - 5
    let mut board = Board{ cells: [[Cell::Empty; 10]; 10] };
    board.print();

    // board.cells[0][0] = Cell::PartialShip(Player::Computer);
    // board.cells[0][1] = Cell::PartialShip(Player::Computer);
    // board.cells[3][1] = Cell::PartialShip(Player::Human);
    // board.cells[4][1] = Cell::PartialShip(Player::Human);
    let mut current_player = Player::Human;
    let (row, col, dir) = current_player.collect_input_location(true);
    for i in 0..2 {
        if i == 0 {
            board.cells[row][col] = Cell::PartialShip(current_player);
        } else if dir == Direction::Horizontal {
            board.cells[row][col + i] = Cell::PartialShip(current_player);
        } else if dir == Direction::Vertical {
            board.cells[row + i][col] = Cell::PartialShip(current_player);
        }
    } 
    board.print();
    // board.cells[location_inputs[0], location_inputs[1]] = 
    loop {
        let guess_list  = current_player.collect_input_location(false);
        board.make_guess(guess_list.0, guess_list.1, current_player);
        board.print();
        if board.has_winner(current_player) {
            println!("Winner! {:?} : {}", current_player, board.has_winner(current_player));
            break;
        }
        current_player = current_player.inverse();
    }
    board.print();
}
