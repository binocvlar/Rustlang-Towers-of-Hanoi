/* Ideas */
/*
 * Given that we know that completion time is on the order of (2**n - 1),
 * I could take a user issued sleep time, and provide an estimated time to
 * completion. Perhaps even a count-down timer and/or progress bar?
 *
 */

/* Crates */
#[macro_use] extern crate itertools;
extern crate termion;

/* Imports */
use std::{fmt,thread,time};
use std::cmp::Ordering;
use itertools::Itertools;
use std::process::exit;
use termion::cursor;

/* Types */
// Add an enum for OptionalDisc:
//     The `u8` carried by the `Disc::Empty` variant signifies the capacity of the `Peg` (A.K.A the
//     largest `OptionalDisc` on the `Board`).
#[derive(Debug, Clone)]
pub enum OptionalDisc {
    Some(Disc),
    None(u8),
}

// Concrete `Disc` type
#[derive(Debug, Clone)]
pub struct Disc {
    size: u8,
    max: u8,
}

// Peg represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
pub struct Peg {
    label: PegLabel,
    capacity: u8,
    stack: Vec<OptionalDisc>
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

// `Board` represents a fixed configuration of three pegs
#[derive(Debug, Clone)]
pub struct Board {
    pub left: Peg,
    pub middle: Peg,
    pub right: Peg,
}

/* Implementations */
impl Disc {
    // Associated function which constructs a new `Disc`
    fn new(size: u8, max: u8) -> Self {
        Disc {
            size,
            max,
        }
    }
}

impl OptionalDisc {
    fn get_padding(&self, pad_length: f64) -> (String, String) {
        // This closure simply returns a string commprised of the requested number of spaces
        let make_padding = |x| (0..x).map(|_| " ").collect::<String>();
        let half_pad_len = pad_length / 2.0_f64;
        // Getting the `ceil` of the left value, and the `floor` of the right value is responsible
        // for right-aliging the disc number within each disc representation, when displaying a disc
        (make_padding(half_pad_len.ceil() as u32), make_padding(half_pad_len.floor() as u32))
    }
}

impl Peg {
    // Associated function which constructs a `Peg` loaded with `OptionalDisc::Some`s
    pub fn new(label: PegLabel, capacity: u8) -> Self {
        Peg {
            label,
            capacity,
            stack: (0..capacity).map(|x| OptionalDisc::Some(Disc::new(x, capacity)))
                                .rev()
                                .collect::<Vec<_>>(),
        }
    }

    // Associated function which constructs a `Peg` loaded with `OptionalDisc::None`s
    fn new_empty(label: PegLabel, capacity: u8) -> Self {
        Peg {
			label,
            capacity,
            stack: Vec::with_capacity(capacity as usize),
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

// This trait implementation is unused, and will likely be removed soon. It is unused, as printing a
// horizontal representation of each `Peg` has not been very useful so far. If I was going to keep
// this around, I'd likely refactor it to construct an exact-sized iterator of padding, to avoid
// unnecessarity allocating space, and then having to `take()` the required amount.
impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get an iterator over a stack of `String`ified `u8`s
        let discs = self.stack.iter()
            .map(|x| format!("({})", x.to_string()));
        // Get a padding iterator, from 0 to Peg.capacity
        let padding = (0..self.capacity)
            .map(|_| "-".to_string());
        // Chain the two iterators together - only take Peg.capacity's worth of elements
        let loaded_peg = discs.chain(padding)
            .take(self.capacity as usize)
            .join("");

        write!(f, "|{:?}: {}", self.label, loaded_peg)
    }
}

impl Ord for PegLabel {
    fn cmp(&self, other: &PegLabel) -> Ordering {
        self.cmp(other)
    }
}

impl Board {
    // Associated function (which constructs a `Board`)
    fn new(capacity: u8) -> Self {
        Board {
            left: Peg::new(PegLabel::Left, capacity),
            middle: Peg::new_empty(PegLabel::Middle, capacity),
            right: Peg::new_empty(PegLabel::Right, capacity),
        }
    }
}

impl fmt::Display for OptionalDisc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionalDisc::Some(disc) => {
                let dashes = (0..disc.size).map(|_| "-").collect::<String>();
                let max_width = disc.max * 2 + disc.max.to_string().len() as u8;
                let total_pad_len = max_width - 2 * dashes.len() as u8 - disc.size.to_string().len() as u8;
                // WRITE A TEST FOR THIS FUNCTION
                // Given 6, it should return ("---", "---")
                // Given 7, it should return ("----", "---")
                let (left_pad, right_pad) = self.get_padding(total_pad_len as f64);
                write!(f, "{}{}{}{}{}", left_pad, dashes, disc.size.to_string(), dashes, right_pad)
            },
            OptionalDisc::None(i) => {
                let padding = (0..i * 2 + i.to_string().len() as u8).map(|_| " ").collect::<String>();
                write!(f, "{}", padding)
            },
        }
    }
}

/* Functions */
pub fn solve_game(disc_tally: u8) -> Board {
    // Clear the terminal
    print!("{}[2J", 27 as char);
    let Board {
        mut left,
        mut middle,
        mut right,
    } = Board::new(disc_tally);

    display_board(&left, &middle, &right);

    // Note that we must subtract `1` from disc_tally, to convert between the number of discs, and
    // the size of the largest disc (example: 10 discs, 9 is the largest (0-indexed)).
    // If you _don't_ subtract 1, you'll panic thanks to an out-by-one error.
    move_tower(disc_tally - 1, &mut left, &mut middle, &mut right);

    display_board(&left, &middle, &right);

    Board { left, middle, right }
}

// Implements an approximation of the famous algorithm which solves the
// "Towers of Hanoi" game using recursion. I've yet to determine the original
// author of this bad boy.
fn move_tower(disc_size: u8, source: &mut Peg, dest: &mut Peg, spare: &mut Peg) {
    if disc_size == 0 {
        if let Some(i) = source.stack.pop() {
            display_board(source, dest, spare);
            dest.stack.push(i);
            display_board(source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(disc_size - 1, source, spare, dest);
        if let Some(i) = source.stack.pop() {
            display_board(source, dest, spare);
            dest.stack.push(i);
            display_board(source, dest, spare);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(disc_size - 1, spare, dest, source);
        display_board(source, dest, spare);
    }
}

// `Board` display function
fn display_board(source: &Peg, dest: &Peg, spare: &Peg) {
    // Clear the terminal
    // print!("{}[2J", 27 as char);

    // Get the `x` and `y` coordinates of the terminal. This is used in order to "jump back" and
    // redraw the board. This should provide a better appearance than continually redrawing the
    // board. Only the `y` coordinate is important in this case.
    let x_y_coords = termion::terminal_size();

    let (_, y) = match x_y_coords {
        Ok((x, y)) => (x, y),
        Err(e) => {
            eprintln!("Unable to get terminal size: '{}'", e);
            exit(1);
        },
    };

    // Sort the borrowed `Peg`s
    let mut pegs = [source, dest, spare];
    pegs.sort();
    let (source, dest, spare) = (pegs[0], pegs[1], pegs[2]);

    // Jump-back to the top of the board
    print!("{}", cursor::Goto(1, y - source.capacity as u16));

    // (l, m, r) means (left, middle, right)
    for (l, m, r) in izip!(get_peg_representation(&source).iter(),
                           get_peg_representation(&dest).iter(),
                           get_peg_representation(&spare).iter()) {
        println!("|{}|{}|{}|", l, m, r);
    }
    // Sleep to ensure the board isn't redrawn too quickly
    thread::sleep(time::Duration::from_millis(100));
}

fn get_peg_representation(peg: &Peg) -> Vec<String> {
    // Convert Vec<Disc> to Iterator of Strings
    let discs = peg.stack.iter()
                         .map(|x| format!("{}", x))
                         .rev();
    // Create the required amount of padding, and chain the `discs` iterator
    // of strings onto the end of this padding.
    //
    // FIXME:
    // Note that I'm collecting into a Vec<String>, as attempting to return
    // the iterator directly yields a terribly long return type...
    (0..(peg.capacity as usize - peg.stack.len()))
        .map(|_| format!("{}", OptionalDisc::None(peg.capacity)))
        .chain(discs)
        .collect::<Vec<_>>()
}
