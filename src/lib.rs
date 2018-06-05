/* Ideas */
/*
 * Given that we know that completion time is on the order of (2**n - 1),
 * I could take a user issued sleep time, and provide an estimated time to
 * completion. Perhaps even a count-down timer and/or progress bar?
 *
 */

/* Imports */
use std::fmt;

/* Types */
// Add a type-synonym for Disc
type Disc = u8;

// Peg represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
pub struct Peg(Vec<Disc>);

impl Peg {
    pub fn new(capacity: usize, largest_disc: Option<Disc>) -> Self {
        // FIXME: I'm adding 1 to a user supplied int. If this int is maliciously chosen, this
        // could panic. Add a bounds check?
        match largest_disc {
            Some(i) => {
                Peg((0..i + 1).rev().collect::<Vec<Disc>>())
            },
            None => Peg(Vec::with_capacity(capacity + 1)),
        }
    }
}

impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Peg(peg) = self;
        let peg_string = peg.iter()
            .map(|x| format!("({})", x.to_string()))
            .collect::<String>();
        write!(f, "||{}", peg_string)
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

    move_tower(disc, &mut left, &mut middle, &mut right);
    Board { left, middle, right }
}

pub fn move_tower(disc: Disc, source: &mut Peg, dest: &mut Peg, spare: &mut Peg) {
    if disc == 0 {
        println!("DEBUG BASE CASE: Source {:?}, Dest {:?}, Spare {:?}", source, dest, spare);
        if let Some(i) = source.0.pop() {
            dest.0.push(i);
            println!("DEBUGa: _DISC_: {}, Source {:?}, Dest {:?}, Spare {:?}", disc, source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(disc - 1, source, spare, dest);
        if let Some(i) = source.0.pop() {
            dest.0.push(i);
            println!("DEBUGc: _DISC_: {}, Source {:?}, Dest {:?}, Spare {:?}", disc, source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(disc - 1, spare, dest, source);
        println!("DEBUGe: _DISC_: {}, Source {:?}, Dest {:?}, Spare {:?}", disc, source, dest, spare);
    }
}
