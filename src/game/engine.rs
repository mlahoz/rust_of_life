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
            self.board.as_mut().unwrap().toggle(x - 1, y - 1)
        } else {
            Err("Game not created yet")
        }
    }

    fn play(&mut self) -> Result<(), &str> {
        Err("TODO!")
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
