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
impl Player {
    fn inverse(&self) -> Player {
        if *self == Player::Computer {
            Player::Human
        } else {
            Player::Computer
        }
    }
    fn guess(&self) -> (usize, usize) {
        if *self == Player::Computer {
            let guess_row: usize = rand::random::<usize>() % 10;
            let guess_col: usize = rand::random::<usize>() % 10;
            println!("{:?}, {:?}", guess_row, guess_col);
            (guess_row, guess_col)
        } else {
            let mut guess = String::new(); 
            std::io::stdin().read_line(&mut guess).unwrap();
            let guess_list: Vec<usize> = guess.trim().split(",").map(|f|f.parse().unwrap()).collect();
            (guess_list[0], guess_list[1])
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
    board.cells[0][0] = Cell::PartialShip(Player::Computer);
    board.cells[0][1] = Cell::PartialShip(Player::Computer);
    board.cells[3][1] = Cell::PartialShip(Player::Human);
    board.cells[4][1] = Cell::PartialShip(Player::Human);
    let mut current_player = Player::Human;
    loop {
        let guess_list  = current_player.guess();
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
