use std::fmt;

pub struct Board {
    pub width: usize,
    pub height: usize,
    board: Vec<Vec<bool>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width: width,
            height: height,
            board: vec![vec![false; width]; height]
        }
    }

    pub fn set(&mut self, width: usize, height: usize, value: bool) -> Result<(), &str> {
        if width < self.width && height < self.height {
            self.board[height][width] = value;
            Ok(())
        } else {
            Err("Invalid indexes")
        }
    }

    pub fn get(&self, width: usize, height: usize) -> Result<bool, &str> {
        if width < self.width && height < self.height {
            Ok(self.board[height][width])
        } else {
            Err("Invalid indexes")
        }
    }

    pub fn toggle(&mut self, width: usize, height: usize) -> Result<(), &str> {
        if width < self.width && height < self.height {
            self.board[height][width] = !self.board[height][width];
            Ok(())
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

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut cloned = Board::new(self.width, self.height);
        cloned.board = self.board.clone();
        cloned
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

#[test]
fn board_toggle() {
    const W: usize = 6;
    const H: usize = 4;

    let mut b = Board::new(W, H);

    assert_eq!(b.get(0, 0).ok(), Some(false));
    assert!(b.toggle(0, 0).is_ok());
    assert_eq!(b.get(0, 0).ok(), Some(true));
    assert!(b.toggle(0, 0).is_ok());
    assert_eq!(b.get(0, 0).ok(), Some(false));

    assert_eq!(b.get(4, 1).ok(), Some(false));
    assert!(b.toggle(4, 1).is_ok());
    assert_eq!(b.get(4, 1).ok(), Some(true));
    assert!(b.toggle(4, 1).is_ok());
    assert_eq!(b.get(4, 1).ok(), Some(false));

    assert!(b.toggle(W, 0).is_err());
    assert!(b.toggle(0, H).is_err());
    assert_eq!(b.toggle(W, H).err(), Some("Invalid indexes"));
}

#[test]
fn board_clone() {
    const W: usize = 4;
    const H: usize = 4;

    let mut b = Board::new(W, H);

    assert!(b.toggle(0, 0).is_ok());
    assert!(b.toggle(1, 1).is_ok());
    assert!(b.toggle(2, 2).is_ok());
    assert!(b.toggle(3, 3).is_ok());

    let cloned = b.clone();

    assert_eq!(b.get(0, 0).ok(), Some(true));
    assert_eq!(b.get(1, 1).ok(), Some(true));
    assert_eq!(b.get(2, 2).ok(), Some(true));
    assert_eq!(b.get(3, 3).ok(), Some(true));

    for x in 0..b.width {
        for y in 0..b.height {
            assert_eq!(cloned.get(x, y), b.get(x, y));
        }
    }

    let expected = "X___\n\
                    _X__\n\
                    __X_\n\
                    ___X\n";

    assert_eq!(cloned.to_string(), expected);
}
