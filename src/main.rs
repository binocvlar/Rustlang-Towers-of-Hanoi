extern crate hanoi_simple;
use hanoi_simple::{move_tower,Peg,Board};

/*
fn move_tower(disc: u8, source: &mut Vec<u8>, dest: &mut Vec<u8>, spare: &mut Vec<u8>) {
    if disc == 0 {
        println!("DEBUG BASE CASE: Source {:?}, Dest {:?}, Spare {:?}", source, dest, spare);
        if let Some(i) = source.pop() {
            dest.push(i);
            println!("DEBUGa: _DISC_: {}, Source {:?}, Dest {:?}, Spare {:?}", disc, source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(disc - 1, source, spare, dest);
        if let Some(i) = source.pop() {
            dest.push(i);
            println!("DEBUGc: _DISC_: {}, Source {:?}, Dest {:?}, Spare {:?}", disc, source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(disc - 1, spare, dest, source);
        println!("DEBUGe: _DISC_: {}, Source {:?}, Dest {:?}, Spare {:?}", disc, source, dest, spare);
    }
    // println!("You've asked me to move disc {}", disc);
}
*/

fn main() {
    /*
    let output = "Left {}, Middle {}, Right {}";
    let mut left: Vec<u8> = vec![2, 1, 0];
    let mut middle: Vec<u8> = vec![];
    let mut right: Vec<u8> = vec![];

    println!("BEGIN: Left {:?}, Middle {:?}, Right {:?}", left, middle, right);
    move_tower(2, &mut left, &mut middle, &mut right);
    println!("END: Left {:?}, Middle {:?}, Right {:?}", left, middle, right);
    */

    /*
    let left = Peg::new(Some(10_u8));
    let middle = Peg::new(None);
    println!("Peg looks like: {:?}", left);
    println!("Peg looks like: {:?}", middle);
    */

    let b = Board::new(10);
    println!("{:?}", b);
    move_tower(2, &mut left, &mut middle, &mut right);
}
