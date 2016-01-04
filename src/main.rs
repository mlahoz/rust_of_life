extern crate rust_of_life;

use rust_of_life::game::board;

fn main() {
    println!("Welcome to Rust of Live!");

    let mut board = board::Board::new(10, 10);
    board.set(0, 0, true).is_ok();
    board.set(9, 0, true).is_ok();
    board.set(10, 0, true).is_err(); // shouldn't work
    println!("[9 0]: {}", board.get(9, 0).ok().unwrap());

    println!("Board [{}, {}]\n{}", board.height, board.width, board);
}
