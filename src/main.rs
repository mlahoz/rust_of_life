extern crate rust_of_life;

use rust_of_life::game::board;
use rust_of_life::game::command;
use std::io;
use std::io::Write;
use std::io::stdout;

fn main() {
    println!("Welcome to Rust of Live!");

    let mut board = board::Board::new(10, 10);
    board.set(0, 0, true).is_ok();
    board.set(9, 0, true).is_ok();
    board.set(10, 0, true).is_err(); // shouldn't work
    println!("[9 0]: {}", board.get(9, 0).ok().unwrap());

    println!("Board [{}, {}]\n{}", board.height, board.width, board);

    loop {
        // Prompt
        print!("game> ");
        stdout().flush().is_ok();

        // Read command
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read command");

        let result = command::parse_command(&input);

        if result.is_ok() {
            let command = result.ok().unwrap();

            println!("Your command: {}", command);

            if let command::Command::Quit = command {
                break;
            }
        } else {
            println!("{}", result.err().unwrap());
        }
    }

    println!("Thanks for playing to Rust of Live!");
}
