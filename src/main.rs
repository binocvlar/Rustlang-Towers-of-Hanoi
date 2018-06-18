/* Crates */
extern crate towers_of_hanoi;
extern crate argparse;

/* Imports */
use towers_of_hanoi::{solve_game, Config};
use std::process::exit;
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

    if game_size < 1 || game_size > 32 {
        eprintln!("Maximum number of Discs must be in the range of 1 - 32 inclusive.");
        exit(1);
    }

    /* Get an Rc<Config> */
    let config = Config::get_config();

    /* Update our Config struct instance */
    config.set_game_size(game_size);
    config.set_refresh_interval(refresh_interval);

    println!("The game size is {:?}", config.get_game_size());
    println!("The refresh interval is {:?}", config.get_refresh_interval());

    /* Solve the game */
    let _solved_board = solve_game(game_size);
}
