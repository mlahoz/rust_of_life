use std::fmt;

#[derive(Copy,Clone)]
pub enum Command {
    New { width: usize, height: usize},
    Toggle { x: usize, y: usize},
    Play,
    Step,
    Quit
}

pub fn parse_command(input: &str) -> Result<Command, &str> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 0 {

        match tokens[0] {
            "quit" | "q" => return Ok(Command::Quit),
            "play" | "p" => return Ok(Command::Play),
            "step" | "s" => return Ok(Command::Step),
            c @ "new" | c @ "n" | c @ "toggle" | c @ "t" => {
                if tokens.len() == 3 {
                    let first: usize = match tokens[1].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid first argument value: not a number"),
                    };
                    let second: usize = match tokens[2].parse() {
                        Ok(num) => num,
                        Err(_) => return Err("Invalid second argument value: not a number"),
                    };

                    if first == 0 || second == 0 {
                        return Err("Invalid argument value");
                    }

                    match c {
                        "new" | "n" => return Ok(Command::New{ width: first, height: second}),
                        "toggle" | "t" => return Ok(Command::Toggle{ x: first, y: second}),
                        _ => return Err("Invalid syntax for command"),
                    }
                } else {
                    return Err("Invalid syntax for command");
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
            Command::Step => write!(f, "Step"),
            Command::New { width, height } => write!(f, "New [width: {} height: {}]", width, height),
            Command::Toggle { x, y } => write!(f, "Toggle [x: {} y: {}]", x, y),
        }

    }
}

#[test]
fn command_new() {
    let c = Command::New { width: 30, height: 20 };
    assert_eq!(c.to_string(), "New [width: 30 height: 20]");

    // parse
    let pc = parse_command("new 30 20").ok().unwrap();
    if let Command::New { width: 30, height: 20} = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("new 20 80").ok().unwrap();
    if let Command::New { width: 20, height: 80} = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("n 30 20").ok().unwrap();
    if let Command::New { width: 30, height: 20} = pc { assert!(true); } else { assert!(false); }

    let error = parse_command("new thirty twenty").err().unwrap();
    assert_eq!(error, "Invalid first argument value: not a number");

    let error = parse_command("new 30 twenty").err().unwrap();
    assert_eq!(error, "Invalid second argument value: not a number");

    let error = parse_command("new 30").err().unwrap();
    assert_eq!(error, "Invalid syntax for command");

    let error = parse_command("new").err().unwrap();
    assert_eq!(error, "Invalid syntax for command");

    let error = parse_command("ne 30 20").err().unwrap();
    assert_eq!(error, "Unknown command");

    let error = parse_command("new 0 10").err().unwrap();
    assert_eq!(error, "Invalid argument value");

    let error = parse_command("new 10 0").err().unwrap();
    assert_eq!(error, "Invalid argument value");
}

#[test]
fn command_toggle() {
    let c = Command::Toggle { x: 4, y: 7 };
    assert_eq!(c.to_string(), "Toggle [x: 4 y: 7]");

    // parse
    let pc = parse_command("toggle 4 7").ok().unwrap();
    if let Command::Toggle { x: 4, y: 7} = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("toggle 2 8").ok().unwrap();
    if let Command::Toggle { x: 2, y: 8} = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("t 4 7").ok().unwrap();
    if let Command::Toggle { x: 4, y: 7} = pc { assert!(true); } else { assert!(false); }

    let error = parse_command("toggle four seven").err().unwrap();
    assert_eq!(error, "Invalid first argument value: not a number");

    let error = parse_command("toggle 4 seven").err().unwrap();
    assert_eq!(error, "Invalid second argument value: not a number");

    let error = parse_command("toggle 4").err().unwrap();
    assert_eq!(error, "Invalid syntax for command");

    let error = parse_command("toggle").err().unwrap();
    assert_eq!(error, "Invalid syntax for command");

    let error = parse_command("tog 4 7").err().unwrap();
    assert_eq!(error, "Unknown command");

    let error = parse_command("toggle 0 10").err().unwrap();
    assert_eq!(error, "Invalid argument value");

    let error = parse_command("toggle 10 0").err().unwrap();
    assert_eq!(error, "Invalid argument value");
}

#[test]
fn command_play() {
    let c = Command::Play;
    assert_eq!(c.to_string(), "Play");

    let pc = parse_command("play").ok().unwrap();
    if let Command::Play = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("p").ok().unwrap();
    if let Command::Play = pc { assert!(true); } else { assert!(false); }

    let error = parse_command("pla").err().unwrap();
    assert_eq!(error, "Unknown command");

}

#[test]
fn command_quit() {
    let c = Command::Quit;
    assert_eq!(c.to_string(), "Quit");

    let pc = parse_command("quit").ok().unwrap();
    if let Command::Quit = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("q").ok().unwrap();
    if let Command::Quit = pc { assert!(true); } else { assert!(false); }

    let error = parse_command("qui").err().unwrap();
    assert_eq!(error, "Unknown command");
}

#[test]
fn command_step() {
    let c = Command::Step;
    assert_eq!(c.to_string(), "Step");

    let pc = parse_command("step").ok().unwrap();
    if let Command::Step = pc { assert!(true); } else { assert!(false); }

    let pc = parse_command("s").ok().unwrap();
    if let Command::Step = pc { assert!(true); } else { assert!(false); }

    let error = parse_command("st").err().unwrap();
    assert_eq!(error, "Unknown command");
}
