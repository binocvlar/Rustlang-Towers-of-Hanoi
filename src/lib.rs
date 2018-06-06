/* Ideas */
/*
 * Given that we know that completion time is on the order of (2**n - 1),
 * I could take a user issued sleep time, and provide an estimated time to
 * completion. Perhaps even a count-down timer and/or progress bar?
 *
 */

/* Crates */
extern crate itertools;

/* Imports */
use std::fmt;
use itertools::Itertools;

/* Types */
// Add a type-synonym for Disc
type Disc = u8;

// Peg represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
pub struct Peg {
    capacity: usize,
    stack: Vec<Disc>
}

impl Peg {
    pub fn new(capacity: usize, largest_disc: Option<Disc>) -> Self {
        // FIXME: I'm adding 1 to a user supplied int. If this int is maliciously chosen, this
        // could panic. Add a bounds check?
        match largest_disc {
            Some(i) => Peg {
                capacity,
                stack: (0..i + 1).rev().collect::<Vec<Disc>>(),
            },
            None => Peg {
                capacity,
                stack: Vec::with_capacity(capacity + 1)
            },
        }
    }
}

impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let empty_peg: Vec<_> = (0..self.capacity)
            .map(|_| "-".to_string())
            .collect();
        let discs = self.stack.iter()
            .map(|x| format!("({})", x.to_string()))
            .collect::<Vec<_>>();
        let loaded_peg = discs.iter()
            .chain(empty_peg.iter())
            .take(10)
            .join("");

        write!(f, "||{}", loaded_peg)
    }
}


// Board represents a fixed configuration of three pegs
#[derive(Debug, Clone)]
pub struct Board {
    pub left: Peg,
    pub middle: Peg,
    pub right: Peg,
}

impl Board {
    pub fn new(largest_disc: Disc) -> Self {
        Board {
            left: Peg::new(largest_disc as usize, Some(largest_disc)),
            middle: Peg::new(largest_disc as usize, None),
            right: Peg::new(largest_disc as usize, None),
        }
    }
}

/* Functions */
pub fn solve_game(disc: Disc, board: &Board) -> Board {
    let board2 = board.clone();
    let Board {
        mut left,
        mut middle,
        mut right,
    } = board2;

    move_tower(disc, &mut left.stack, &mut middle.stack, &mut right.stack);
    Board { left, middle, right }
}

fn move_tower(disc: Disc, source: &mut Vec<Disc>, dest: &mut Vec<Disc>, spare: &mut Vec<Disc>) {
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
}
