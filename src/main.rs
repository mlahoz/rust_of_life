extern crate rust_of_life;

use rust_of_life::game::command;
use rust_of_life::game::engine;
use std::io;
use std::io::Write;
use std::io::stdout;
use std::thread;

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
                loop {
                    // This flag and the following {} block are needed as a workaround
                    // for the strict borrowing rules of Rust:
                    // the first mutable borrow when calling process_command was preventing the second
                    // borrow when getting the board to print it
                    // the {} block is needed to ensure the first borrow is released, the flag is needed
                    // to save the result of the process_command to decide if the board should be printed
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

                    if let command::Command::Play = command {
                        thread::sleep_ms(1000);
                    } else {
                        break;
                    }
                }
            }
        } else {
            println!("{}", result.err().unwrap());
        }
    }

    println!("Thanks for playing to Rust of Live!");
}
