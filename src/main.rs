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
                let mut command_ok = false;
                {
                    let result = engine.process_command(command);

                    if result.is_ok() {
                        command_ok = true;
                    } else {
                        println!("Error processing command: {}", result.err().unwrap());
                    }
                }
                if command_ok {
                    print!("{}", engine.board().unwrap());
                    stdout().flush().is_ok();
                }
            }
        } else {
            println!("{}", result.err().unwrap());
        }
    }

    println!("Thanks for playing to Rust of Live!");
}
