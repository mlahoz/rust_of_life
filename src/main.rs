use std::fmt;

struct Board {
    height: u8,
    width: u8,
    board: Vec<Vec<bool>>
}

impl Board {
    fn new(height: u8, width: u8) -> Board {
        Board {
            height: height,
            width: width,
            board: vec![vec![false; width as usize]; height as usize]
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = self.board.iter().fold("".to_string(), |acc, x| {
            let line = x.iter().fold("".to_string(), |acc, x| acc + if *x { "X" } else { "_" });
            acc + &line + "\n"
        });
        write!(f, "{}", display)
    }
}

fn main() {
    println!("Welcome to Rust of Live!");
    let board = Board::new(10, 10);

    println!("Board [{}, {}]\n{}", board.height, board.width, board);
}
