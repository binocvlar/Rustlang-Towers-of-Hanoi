/* Crates */
extern crate hanoi_simple;

/* Imports */
use hanoi_simple::{solve_game};
use std::env;
use std::process::exit;

fn main() {
    let key = "HANOI_GAME_SIZE";
    let game_size = match env::var(key) {
        Ok(val) => val,
        Err(_) => "10".to_string(),
    };

    let game_size = match game_size.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Could not convert your HANOI_GAME_SIZE to a u8");
            exit(1);
        }
    };

    let _solved_board = solve_game(game_size);
}
