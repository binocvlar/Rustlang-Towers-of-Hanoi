/* Crates */
extern crate hanoi_simple;

/* Imports */
use hanoi_simple::{Disc,solve_game};

fn main() {

    let solved_board = solve_game(3);

    // println!("{}", solved_board.left);
    // println!("{}", solved_board.middle);
    // println!("{}", solved_board.right);
}
