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
            let board = self.board.as_mut().unwrap();
            let cloned = board.clone();

            for i in 0..board.width {
                for j in 0..board.height {
                    let is_alive = cloned.get(i, j).ok().unwrap();
                    let neighbours = cloned.neighbours(i, j);
                    if is_alive {
                        match neighbours {
                                0 | 1 => { board.set(i, j, false).is_ok(); },  // dies, by under-population
                                2 | 3 => {},                                   // lives
                                _ => { board.set(i, j, false).is_ok(); },      // dies, by over-population
                        }
                    } else {
                        match neighbours {
                                3 => { board.set(i, j, true).is_ok(); },       // borns, by reproduction
                                _ => {},                                       // remains death
                        }
                    }
                }
            }
            Ok(())
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
            command::Command::Step => self.play(),
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
    assert!(e.process_command(command::Command::Play).is_ok());
}

#[test]
fn engine_cmd_step() {
    let mut e = Engine::new();
    assert!(e.process_command(command::Command::Step).is_err());
    assert_eq!(e.process_command(command::Command::Step).err().unwrap(), "Game not created yet");

    assert!(e.process_command(command::Command::New { width: 10, height: 10 } ).is_ok());
    assert!(e.process_command(command::Command::Step).is_ok());
}

#[test]
fn engine_play() {
    let mut e = Engine::new();
    assert!(e.play().is_err());
    assert_eq!(e.play().err().unwrap(), "Game not created yet");

    assert!(e.process_command(command::Command::New { width: 6, height: 6 } ).is_ok());
    assert!(e.play().is_ok());
    let b = e.board().unwrap();

    let expected = "______\n\
                    ______\n\
                    ______\n\
                    ______\n\
                    ______\n\
                    ______\n";

    assert_eq!(b.to_string(), expected);
}

#[test]
fn engine_play_starve_underpopulation() {
    let mut e = Engine::new();

    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());

    // starve 0 neighbours
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(0,0).unwrap(), false);

    // starve 1 neighbour
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(0,0).unwrap(), false);
}

#[test]
fn engine_play_survive() {
    let mut e = Engine::new();

    // survive 2 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,0).unwrap(), true);

    // survive 3 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), true);
}

#[test]
fn engine_play_starve_overpopulation() {
    let mut e = Engine::new();

    // starve 4 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,0).unwrap(), false);

    // starve 5 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,0).unwrap(), false);

    // starve 6 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(1, 3).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,0).unwrap(), false);

    // starve 7 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(1, 3).is_ok());
    assert!(e.toggle(2, 3).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,0).unwrap(), false);

    // starve 8 neighbours
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(1, 3).is_ok());
    assert!(e.toggle(2, 3).is_ok());
    assert!(e.toggle(3, 3).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,0).unwrap(), false);
}

#[test]
fn engine_play_born() {
    let mut e = Engine::new();

    // 1 neighbour no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);

    // 2 neighbours no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);

    // 3 neighbours new born!
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), true);

    // 4 neighbours no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);

    // 5 neighbours no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);

    // 6 neighbours no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(1, 3).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);

    // 7 neighbours no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(1, 3).is_ok());
    assert!(e.toggle(2, 3).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);

    // 8 neighbours no born
    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 1).is_ok());
    assert!(e.toggle(2, 1).is_ok());
    assert!(e.toggle(3, 1).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(1, 3).is_ok());
    assert!(e.toggle(2, 3).is_ok());
    assert!(e.toggle(3, 3).is_ok());
    assert!(e.play().is_ok());

    let b = e.board().unwrap();
    assert_eq!(b.get(1,1).unwrap(), false);
}

#[test]
fn engine_play_blinker() {
    let mut e = Engine::new();

    assert!(e.process_command(command::Command::New { width: 3, height: 3 } ).is_ok());
    assert!(e.toggle(1, 2).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.toggle(3, 2).is_ok());

    let expected_1 = "_X_\n\
                      _X_\n\
                      _X_\n";
    let expected_2 = "___\n\
                      XXX\n\
                      ___\n";

    assert!(e.play().is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.to_string(), expected_1);

    assert!(e.play().is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.to_string(), expected_2);
}

#[test]
fn engine_play_block() {
    let mut e = Engine::new();

    assert!(e.process_command(command::Command::New { width: 4, height: 4 } ).is_ok());
    assert!(e.toggle(2, 2).is_ok());
    assert!(e.toggle(2, 3).is_ok());
    assert!(e.toggle(3, 2).is_ok());
    assert!(e.toggle(3, 3).is_ok());

    let expected = "____\n\
                    _XX_\n\
                    _XX_\n\
                    ____\n";

    assert!(e.play().is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.to_string(), expected);

    assert!(e.play().is_ok());
    let b = e.board().unwrap();
    assert_eq!(b.to_string(), expected);
}
