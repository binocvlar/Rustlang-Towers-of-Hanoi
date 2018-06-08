/* Crates */
extern crate hanoi_simple;

/* Imports */
use hanoi_simple::{solve_game,Board};

fn main() {

    let MAX_DISC = 3;

    let solved_board = solve_game(MAX_DISC, &Board::new(MAX_DISC));

    // println!("Your solved board is: {:?}", solved_board);

    println!("{}", solved_board.left);
    println!("{}", solved_board.middle);
    println!("{}", solved_board.right);
}
