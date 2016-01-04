use std::fmt;

pub enum Command {
    New { width: usize, height: usize},
    Set { x: usize, y: usize},
    Play,
    Print,
    Quit
}

pub fn parse_command(input: &str) -> Result<Command, &str> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 0 {

        // TODO: refactor parsing of arguments

        match tokens[0] {
            "quit" => return Ok(Command::Quit),
            "play" => return Ok(Command::Play),
            "print" => return Ok(Command::Print),
            "new" => {
                if tokens.len() == 3 {
                    let width: usize = match tokens[1].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid width value"),
                    };
                    let height: usize = match tokens[2].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid height value"),
                    };
                    return Ok(Command::New{ width: width, height: height});
                } else {
                    return Err("Invalid syntax for new command");
                }
            },
            "set" => {
                if tokens.len() == 3 {
                    let x: usize = match tokens[1].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid x value"),
                    };
                    let y: usize = match tokens[2].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid y value"),
                    };
                    return Ok(Command::Set{ x: x, y: y});
                } else {
                    return Err("Invalid syntax for set command");
                }
            },
            _ => return Err("Unknown command"),
        }

    }

    Err("Unable to parse command")
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {
            Command::Quit => write!(f, "Quit"),
            Command::Play => write!(f, "Play"),
            Command::Print => write!(f, "Print"),
            Command::New { width, height } => write!(f, "New [width: {} height: {}]", width, height),
            Command::Set { x, y } => write!(f, "Set [x: {} y: {}]", x, y),
        }

    }
}

// TODO: unit tests
