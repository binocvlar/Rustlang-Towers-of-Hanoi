/* Crates */
extern crate towers_of_hanoi;
extern crate argparse;

/* Imports */
use towers_of_hanoi::{solve_game, Config};
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

    // Constrain the size of user input
    if game_size < 1 || game_size > 32 {
        eprintln!("Maximum number of Discs must be in the range of 1 - 32 inclusive.");
        std::process::exit(1);
    }

    // Get a Config type
    let config = Config::new(game_size, refresh_interval);

    /* Solve the game */
    solve_game(&config);
}
