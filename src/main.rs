struct Board {
    cells: [[Cell; 10]; 10],
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Empty,
    PartialShip(Player),
    Hit,
    Miss,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
    Human,
    Computer,
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
    fn make_guess(&mut self, row: usize, col: usize) {
        if self.cells[row][col] == Cell::PartialShip(Player::Computer) {
            // its a hit
            self.cells[row][col] = Cell::Hit;
        }
        else {
            self.cells[row][col] = Cell::Miss;
        }
    }
    fn has_winner(&self) -> bool {
        let mut count = 0;
        for x in &self.cells {
            for y in x {
                if *y == Cell::Hit {
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
    loop {
        let mut guess = String::new(); 
        std::io::stdin().read_line(&mut guess).unwrap();
        let guess_list: Vec<usize> = guess.trim().split(",").map(|f|f.parse().unwrap()).collect();
        board.make_guess(guess_list[0], guess_list[1]);
        board.print();
        if board.has_winner() {
            println!("Winner! {}", board.has_winner());
            break;
        }
    }
    board.print();
}