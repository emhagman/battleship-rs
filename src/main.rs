struct Board {
    cells: [[i32; 10]; 10],
}

impl Board {
    fn print(&self){
        for x in self.cells {
            for y in x {
                print!("{}", y);
            }
            println!();
        }
    }
    fn make_guess(&mut self, row: usize, col: usize) {
        if self.cells[row][col] == 1 {
            // its a hit
            self.cells[row][col] = 2;
        }
    }
    fn has_winner(&self) -> bool {
        let mut count = 0;
        for x in &self.cells {
            for y in x {
                if *y == 2 {
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
    let mut board = Board{ cells: [[0; 10]; 10] };
    board.print();
    board.cells[0][0] = 1;
    board.cells[0][1] = 1;
    board.make_guess(0, 0);
    println!("{}", board.has_winner());
    board.print();
    board.make_guess(0, 1);
    println!("{}", board.has_winner());
    board.print();
}