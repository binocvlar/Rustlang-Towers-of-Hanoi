extern crate hanoi_simple;
use hanoi_simple::{move_tower,solve_game,Board};

fn main() {

    let MAX_DISC: u8 = 10;

    let solved_board = solve_game(MAX_DISC, &Board::new(MAX_DISC));

    println!("Your solved board is: {:?}", solved_board);
}
