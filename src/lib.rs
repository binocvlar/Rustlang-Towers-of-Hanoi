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
// Add a newtype for Disc
#[derive(Debug, Clone)]
pub struct Disc (u8);

impl Disc {
    pub fn new(disc: u8) -> Self {
        Disc(disc)
    }
}

// Peg represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
pub struct Peg {
    label: PegLabel,
    capacity: usize,
    stack: Vec<Disc>
}

impl Peg {
    // Associated function (which constructs a Peg)
    pub fn new(label: PegLabel, capacity: usize, largest_disc: Option<&Disc>) -> Self {
        // FIXME: I'm adding 1 to a user supplied int. If this int is maliciously chosen, this
        // could panic. Add a bounds check?
        match largest_disc {
            Some(i) => Peg {
                label,
                capacity,
                stack: (0..i.0 + 1).rev().map(|x| Disc::new(x)).collect::<Vec<Disc>>(),
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

// Note that I have implemented the `Eq` trait here, rather than derived it, as deriving it would
// necessitate that all `Peg` struct members would need to implement this trait as well.
// From the docs:
//
// "This trait can be used with #[derive]. When derived, because Eq has no extra methods, it is only
// informing the compiler that this is an equivalence relation rather than a partial equivalence
// relation. Note that the derive strategy requires all fields are Eq, which isn't always desired."
//
// From the [Rust docs](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
impl Eq for Peg {}

impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let discs = self.stack.iter()
            .map(|x| format!("({})", x.0.to_string()));
        let padding = (0..self.capacity)
            .map(|_| "-".to_string());
        let loaded_peg = discs.chain(padding)
            .take(self.capacity)
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
        self.cmp(other)
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
    fn new(largest_disc: &Disc) -> Self {
        Board {
            left: Peg::new(PegLabel::Left, (largest_disc.0 + 1) as usize, Some(&largest_disc)),
            middle: Peg::new(PegLabel::Middle, (largest_disc.0 + 1) as usize, None),
            right: Peg::new(PegLabel::Right, (largest_disc.0 + 1) as usize, None),
        }
    }
}

/* Functions */
pub fn solve_game(disc: &Disc) -> Board {
    // Clear the terminal
    print!("{}[2J", 27 as char);
    let Board {
        mut left,
        mut middle,
        mut right,
    } = Board::new(disc);

    display_board(&left, &middle, &right);

    move_tower(disc, &mut left, &mut middle, &mut right);
    Board { left, middle, right }
}

// Implements an approximation of the famous algorithm which solves the
// "Towers of Hanoi" game using recursion. I've yet to determine the original
// author of this bad boy.
fn move_tower(disc: &Disc, source: &mut Peg, dest: &mut Peg, spare: &mut Peg) {
    if disc.0 == 0 {
        if let Some(i) = source.stack.pop() {
            dest.stack.push(i);
            display_board(source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(&Disc(disc.0 - 1), source, spare, dest);
        if let Some(i) = source.stack.pop() {
            dest.stack.push(i);
            display_board(source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(&Disc::new(disc.0 - 1), spare, dest, source);
    }
}

// `Board` display function
fn display_board(source: &Peg, dest: &Peg, spare: &Peg) {

    let mut pegs = [source, dest, spare];
    pegs.sort();
    let (source, dest, spare) = (pegs[0], pegs[1], pegs[2]);

    // (l, m, r) means (left, middle, right)
    for (l, m, r) in izip!(get_peg_display(&source).iter(),
                           get_peg_display(&dest).iter(),
                           get_peg_display(&spare).iter()) {
        println!("{}     {}     {}", l, m, r);
    }
    thread::sleep(time::Duration::from_millis(1000));
    // Clear the terminal
    print!("{}[2J", 27 as char);
}

fn get_peg_display(peg: &Peg) -> Vec<String> {
    // Convert Vec<u8> to Iterator of Strings
    let discs = peg.stack.iter()
                   .map(|x| format!("{}", x.0.to_string()))
                   .rev();
    // Create the required amount of padding, and chain the `discs` iterator
    // of strings onto the end of this padding.
    // Note that I'm collecting into a Vec<String>, as attempting to return
    // the iterator yields a terribly long return type... FIXME
    (0..(peg.capacity - peg.stack.len()))
        .map(|_| String::from("-"))
        .chain(discs)
        .collect::<Vec<_>>()
}
