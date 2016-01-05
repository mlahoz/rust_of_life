extern crate rust_of_life;

use rust_of_life::game::command;
use rust_of_life::game::engine;
use std::io;
use std::io::Write;
use std::io::stdout;

fn main() {
    println!("Welcome to Rust of Live!");

    let mut engine = engine::Engine::new();

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

            if let command::Command::Quit = command {
                break;
            } else {
                match engine.process_command(command) {
                    Ok(None) => {},
                    Ok(Some(board)) => println!("{}", board),
                    Err(error) => println!("Error processing command: {}", error),
                }
            }
        } else {
            println!("{}", result.err().unwrap());
        }
    }

    println!("Thanks for playing to Rust of Live!");
}
