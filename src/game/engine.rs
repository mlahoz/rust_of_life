use ::game::board;
use ::game::command;

pub struct Engine {
    board: Option<board::Board>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            board: None
        }
    }

    pub fn board(&self) -> Option<board::Board> {
        self.board.clone()
    }

    fn toggle(&mut self, x: usize, y: usize) -> Result<(), &str> {
        if self.board.is_some() {
            let board = self.board.as_mut().unwrap();
            if x == 0 || x > board.width || y == 0 || y > board.height {
                Err("Index out of bounds")
            } else {
                board.toggle(x - 1, y - 1)
            }
        } else {
            Err("Game not created yet")
        }
    }

    fn play(&mut self) -> Result<(), &str> {
        if self.board.is_some() {
            Err("TODO!")
        } else {
            Err("Game not created yet")
        }
    }

    pub fn process_command(&mut self, command: command::Command) -> Result<(), &str> {
        println!("Command: {}", command);

        match command {
            command::Command::Quit => {
                self.board = None;
                Ok(())
            },
            command::Command::Play => self.play(),
            command::Command::New { width, height } => {
                self.board = Some(board::Board::new(width, height));
                Ok(())
            },
            command::Command::Toggle { x, y } => self.toggle(x, y),
        }
    }
}

#[test]
fn engine_constructor() {
    let e = Engine::new();
    assert!(e.board.is_none());
}

#[test]
fn engine_board() {
    let e = Engine::new();
    let b = e.board();
    assert!(b.is_none());
}

#[test]
fn engine_cmd_new() {
    let mut e = Engine::new();
    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());

    let b = e.board();
    assert!(b.is_some());
    let b = e.board().unwrap();
    assert_eq!(b.width, 10);
    assert_eq!(b.height, 10);

    assert!(e.process_command(command::Command::New { width: 30, height: 20 } ).is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.width, 30);
    assert_eq!(b.height, 20);
}

#[test]
fn engine_cmd_quit() {
    let mut e = Engine::new();
    assert!(e.process_command(command::Command::Quit).is_ok());

    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());
    assert!(e.process_command(command::Command::Quit).is_ok());

    assert!(e.board.is_none());
}

#[test]
fn engine_toggle() {
    let mut e = Engine::new();
    assert!(e.toggle(3, 7).is_err());
    assert_eq!(e.toggle(3, 7).err().unwrap(), "Game not created yet");

    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.get(2, 6).ok().unwrap(), false);
    assert!(e.toggle(3, 7).is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.get(2, 6).ok().unwrap(), true);

    assert!(e.toggle(0, 0).is_err());
    assert!(e.toggle(1, 0).is_err());
    assert!(e.toggle(0, 1).is_err());
    assert!(e.toggle(11, 1).is_err());
    assert!(e.toggle(11, 11).is_err());
    assert!(e.toggle(1, 11).is_err());
    assert_eq!(e.toggle(0, 0).err().unwrap(), "Index out of bounds");
}

#[test]
fn engine_play() {
    let mut e = Engine::new();
    assert!(e.play().is_err());
    assert_eq!(e.play().err().unwrap(), "Game not created yet");

    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());
    assert!(e.play().is_err());
}

#[test]
fn engine_cmd_toggle() {
    let mut e = Engine::new();
    assert!(e.process_command(command::Command::Toggle { x: 3, y: 7 } ).is_err());
    assert_eq!(e.process_command(command::Command::Toggle { x: 3, y: 7 } ).err().unwrap(), "Game not created yet");

    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.get(2, 6).ok().unwrap(), false);
    assert!(e.process_command(command::Command::Toggle { x: 3, y: 7 } ).is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.get(2, 6).ok().unwrap(), true);
}

#[test]
fn engine_cmd_play() {
    let mut e = Engine::new();
    assert!(e.process_command(command::Command::Play).is_err());
    assert_eq!(e.process_command(command::Command::Play).err().unwrap(), "Game not created yet");

    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());
    assert!(e.process_command(command::Command::Play).is_err());
}
