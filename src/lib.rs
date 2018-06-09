/* Ideas */
/*
 * Given that we know that completion time is on the order of (2**n - 1),
 * I could take a user issued sleep time, and provide an estimated time to
 * completion. Perhaps even a count-down timer and/or progress bar?
 *
 */

/* Crates */
#[macro_use] extern crate itertools;

/* Imports */
use std::{fmt,thread,time};
use std::cmp::Ordering;
use itertools::Itertools;

/* Types */
// Add a type-synonym for Disc
type Disc = u8;
// This exists just to pretty up my return type, but is declaring it
// here even uglier?
type PegTriad<'a> = (&'a Peg, &'a Peg, &'a Peg);


// Peg represents one of three vertical pegs in a game board
#[derive(Debug, Clone, Eq)]
pub struct Peg {
    label: PegLabel,
    capacity: usize,
    stack: Vec<Disc>
}

impl Peg {
    // Associated function (which constructs a Peg)
    pub fn new(label: PegLabel, capacity: usize, largest_disc: Option<Disc>) -> Self {
        // FIXME: I'm adding 1 to a user supplied int. If this int is maliciously chosen, this
        // could panic. Add a bounds check?
        match largest_disc {
            Some(i) => Peg {
                label,
                capacity,
                stack: (0..i + 1).rev().collect::<Vec<Disc>>(),
            },
            None => Peg {
                label,
                capacity,
                stack: Vec::with_capacity(capacity + 1)
            },
        }
    }
}

impl PartialOrd for Peg {
    fn partial_cmp(&self, other: &Peg) -> Option<Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

impl PartialEq for Peg {
    fn eq(&self, other: &Peg) -> bool {
        self.label == other.label
    }
}

impl Ord for Peg {
    fn cmp(&self, other: &Peg) -> Ordering {
        self.label.cmp(&other.label)
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
            // FIXME: This is hardcoded... Tsk tsk!
            .take(10)
            .join("");

        write!(f, "|{:?}: {}", self.label, loaded_peg)
    }
}

// PegLabel: The ordering of each variant in the definition below is responsible for the ordering
// of `Peg`'s within a `Board`:
//
// "When derived on enums, variants are ordered by their top-to-bottom declaration order."*
// * From https://doc.rust-lang.org/std/cmp/trait.Ord.html
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum PegLabel {
    Left,
    Middle,
    Right,
}

impl Ord for PegLabel {
    fn cmp(&self, other: &PegLabel) -> Ordering {
        self.cmp(&other)
    }
}

// `Board` represents a fixed configuration of three pegs
#[derive(Debug, Clone)]
pub struct Board {
    pub left: Peg,
    pub middle: Peg,
    pub right: Peg,
}

impl Board {
    // Associated function (which constructs a `Board`)
    pub fn new(largest_disc: Disc) -> Self {
        Board {
            left: Peg::new(PegLabel::Left, largest_disc as usize, Some(largest_disc)),
            middle: Peg::new(PegLabel::Middle, largest_disc as usize, None),
            right: Peg::new(PegLabel::Right, largest_disc as usize, None),
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

// Implements approximation of the famous algorithm which solves the "Towers of Hanoi"
// game using recursion. I've yet to determine the original author of this bad boy.
fn move_tower(disc: Disc, source: &mut Peg, dest: &mut Peg, spare: &mut Peg) {
    if disc == 0 {
        if let Some(i) = source.stack.pop() {
            dest.stack.push(i);
            // thread::sleep(time::Duration::from_millis(1000));
            display_board(source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(disc - 1, source, spare, dest);
        if let Some(i) = source.stack.pop() {
            dest.stack.push(i);
            display_board(source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(disc - 1, spare, dest, source);
    }
}

/*
// Peg display function
fn get_peg_iterator<'a>(peg: &Peg) -> Chain<Iter<'a, String>, String> {
    // empty_peg should be an iterator
    let empty_peg = (0..peg.capacity).map(|_| "-".to_string());

    // I AM UP TO HERE :)
    peg.stack.iter().map(|x| x.to_string())
                    .chain(&empty_peg)
                    .take(peg.capacity)
}
*/

// Simple display function
fn display_board(source: &Peg, dest: &Peg, spare: &Peg) {
    // FIXME!
    let padding1: Vec<_> = (0..source.capacity).map(|_| "-".to_string()).collect();
    let padding2: Vec<_> = (0..source.capacity).map(|_| "-".to_string()).collect();
    let padding3: Vec<_> = (0..source.capacity).map(|_| "-".to_string()).collect();

    let mut pegs = [source, dest, spare];
    pegs.sort();
    let (source, dest, spare) = (pegs[0], pegs[1], pegs[2]);

    // let _ = disc_peg.stack.iter()
    //               .map(|x| x.to_string())
    //               .chain(padding)
    //               .take(disc_peg.capacity);

    // FIXME! This should be a function
    let source = source.stack.iter()
                             .map(|x| x.to_string())
                             .chain(padding1)
                             .take(source.capacity).collect::<Vec<_>>();
    let source = source.iter().rev();

    let dest = dest.stack.iter()
                         .map(|x| x.to_string())
                         .chain(padding2)
                         .take(dest.capacity).collect::<Vec<_>>();
    let dest = dest.iter().rev();

    let spare = spare.stack.iter()
                           .map(|x| x.to_string())
                           .chain(padding3)
                           .take(spare.capacity).collect::<Vec<_>>();;
    let spare = spare.iter().rev();

    for (a, b, c) in izip!(source, dest, spare) {
        println!("{}     {}     {}", a, b, c);
    }
    println!("--------------------");

    // This is daggy, but it works
    // println!("{}", source);
    // println!("{}", dest);
    // println!("{}", spare);
    // println!("");
}

fn magic(disc_peg: &Peg, padding: Vec<String>) -> () {
// fn magic(disc_peg: &Peg, padding: Vec<String>) -> iter::Take<iter::Chain<iter::Map<slice::Iter<u8>, [closure@src/lib.rs:202:24: 202:41]>, vec::IntoIter<String>>> {
    // Using a "let" statement and ';' here to ensure we return unit
    // The return type would be something like this!
    // std::iter::Take<std::iter::Chain<std::iter::Map<std::slice::Iter<'_, u8>, [closure@src/lib.rs:202:24: 202:41]>, std::vec::IntoIter<std::string::String>>>
    let _ = disc_peg.stack.iter()
                  .map(|x| x.to_string())
                  .chain(padding)
                  .take(disc_peg.capacity);
}
