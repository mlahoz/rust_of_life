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

    pub fn process_command(&mut self, command: command::Command) -> Result<Option<board::Board>, &str> {
        println!("Command: {}", command);

        match command {
            command::Command::Quit => self.board = None,
            command::Command::Play => println!("TODO!"),
            command::Command::Show => {
                if let None = self.board {
                    return Err("Game not created yet");
                }
                // TODO remove this when returning properly the board
                else {
                    println!("{}", self.board.as_mut().unwrap());
                }
            },
            command::Command::New { width, height } => self.board = Some(board::Board::new(width, height)),
            command::Command::Toggle { x, y } => {
                if self.board.is_some() {
                    if let Err(error) = self.board.as_mut().unwrap().toggle(x - 1, y - 1) {
                        return Err(error);
                    }
                } else {
                    return Err("Game not created yet");
                }
            },
        }

        // TODO clone board?
        Ok(None)
    }
}
