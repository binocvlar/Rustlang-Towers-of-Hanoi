extern crate hanoi_simple;
use hanoi_simple::{move_tower,Board};

fn main() {

    let MAX_DISC: u8 = 10;

    // let b = Board::new(2);
    let Board {
        mut left,
        mut middle,
        mut right,
    } = Board::new(MAX_DISC);

    move_tower(MAX_DISC, &mut left, &mut middle, &mut right);
}
