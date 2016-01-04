use std::fmt;

struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            width: width,
            height: height,
            board: vec![vec![false; width]; height]
        }
    }

    fn set(&mut self, width: usize, height: usize, value: bool) -> Result<(), &str> {
        if width < self.width && height < self.height {
            self.board[height][width] = value;
            Ok(())
        } else {
            Err("Invalid indexes")
        }
    }

    fn get(&mut self, width: usize, height: usize) -> Result<bool, &str> {
        if width < self.width && height < self.height {
            Ok(self.board[height][width])
        } else {
            Err("Invalid indexes")
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

#[test]
fn board_constructor() {
    const W: usize = 30;
    const H: usize = 20;

    let b = Board::new(W, H);

    assert_eq!(b.width, W);
    assert_eq!(b.height, H);

    assert_eq!(b.board.len(), H);
    for i in 0..H {
        assert_eq!(b.board[i].len(), W);
    }

    for i in 0..H {
        for j in 0..W {
            assert_eq!(b.board[i][j], false);
        }
    }
}

#[test]
fn board_print() {
    const W: usize = 6;
    const H: usize = 4;

    let b = Board::new(W, H);

    let expected = "______\n\
                    ______\n\
                    ______\n\
                    ______\n";

    assert_eq!(b.to_string(), expected);
}

#[test]
fn board_set() {
    const W: usize = 6;
    const H: usize = 4;

    let mut b = Board::new(W, H);

    assert!(b.set(0, 0, true).is_ok());
    assert_eq!(b.board[0][0], true);
    assert!(b.set(0, 0, false).is_ok());
    assert_eq!(b.board[0][0], false);

    // Invert width and height when accessing directly to the vector
    assert!(b.set(W - 1, 0, true).is_ok());
    assert_eq!(b.board[0][W - 1], true);

    assert!(b.set(0, H - 1, true).is_ok());
    assert_eq!(b.board[H - 1][0], true);

    assert!(b.set(W - 1, H - 1, true).is_ok());
    assert_eq!(b.board[H - 1][W - 1], true);

    assert!(b.set(4, 1, true).is_ok());
    assert_eq!(b.board[1][4], true);

    assert!(b.set(W, 0, true).is_err());
    assert!(b.set(0, H, true).is_err());
    assert_eq!(b.set(W, H, true).err(), Some("Invalid indexes"));

    let expected = "_____X\n\
                    ____X_\n\
                    ______\n\
                    X____X\n";

    assert_eq!(b.to_string(), expected);
}

#[test]
fn board_get() {
    const W: usize = 6;
    const H: usize = 4;

    let mut b = Board::new(W, H);

    assert_eq!(b.get(0, 0).ok(), Some(false));
    assert!(b.set(0, 0, true).is_ok());
    assert_eq!(b.get(0, 0).ok(), Some(true));

    assert_eq!(b.get(4, 1).ok(), Some(false));
    assert!(b.set(4, 1, true).is_ok());
    assert_eq!(b.get(4, 1).ok(), Some(true));

    assert!(b.get(W, 0).is_err());
    assert!(b.get(0, H).is_err());
    assert_eq!(b.get(W, H).err(), Some("Invalid indexes"));
}

fn main() {
    println!("Welcome to Rust of Live!");

    let mut board = Board::new(10, 10);
    board.set(0, 0, true).is_ok();
    board.set(9, 0, true).is_ok();
    board.set(10, 0, true).is_err(); // shouldn't work
    println!("[9 0]: {}", board.get(9, 0).ok().unwrap());

    println!("Board [{}, {}]\n{}", board.height, board.width, board);
}
