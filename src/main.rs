/* Crates */
extern crate towers_of_hanoi;
extern crate argparse;

/* Imports */
use towers_of_hanoi::solve_game;
use argparse::{ArgumentParser, StoreOption};

fn main() {
    /* Handle user arguments */
    let mut game_size: Option<u8> = None;
    let mut refresh_interval: Option<u64> = None;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Towers of Hanoi");
        ap.refer(&mut game_size)
            .add_option(&["-s", "--size"], StoreOption, "Specifies the number of Discs in a Game");
        ap.refer(&mut refresh_interval)
            .add_option(&["-r", "--refresh"], StoreOption, "Specifies the number of milliseconds to sleep between calls");
        ap.parse_args_or_exit();
    }

    let game_size = match game_size {
        Some(i) => i,
        None => 10,
    };

    let refresh_interval = match refresh_interval {
        Some(i) => i,
        None => 0,
    };

    /* Solve the game */
    let _solved_board = solve_game(game_size, game_size, refresh_interval);
}
