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
use std::process::exit;
use termion::{cursor, clear};

/* Types */
// Add an enum for OptionalDisc:
//     The `u8` carried by the `Disc::Empty` variant signifies the capacity of the `Peg` (A.K.A the
//     largest `Disc` on the `Board`).
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
    repr: String,
}

// Peg represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
pub struct Peg {
    label: PegLabel,
    capacity: u8,
    stack: Vec<OptionalDisc>,
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
        // Contains a `String` of dashes, e.g. "--------"
        let dashes = (0..size).map(|_| "▬").collect::<String>();
        // FIXME: Display this if terminal does NOT suppot UTF8
        // let dashes = (0..size).map(|_| "-").collect::<String>();
        // Contains the width of the representation of the largest `Disc`
        let max_width = max * 2 + max.to_string().len() as u8;
        // How much total whitespace padding is required, in order for this `Disc` to line up properly
        let total_pad_len = max_width - 2 * dashes.chars().count() as u8 - size.to_string().len() as u8;
        // Contains the whitespace on either side of the `Disc` (can differ by 1).
        let (left_pad, right_pad) = Disc::get_padding(total_pad_len as f64);
        // Contains the total textual representation of the new `Disc`
        let repr = format!("{}{}{}{}{}", left_pad, dashes, size.to_string(), dashes, right_pad);
        Disc {
            size,
            max,
            repr,
        }
    }

    // Return example: ("   ", "  ")
    fn get_padding(pad_length: f64) -> (String, String) {
        // This closure simply returns a string comprised of the requested number of spaces
        let make_padding = |x: u32| (0..x).map(|_| " ").collect::<String>();
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

    // This method returns a `Vec<String>` representing each `OptionalDisc` on the peg
    fn get_peg_repr(&self) -> Vec<String> {
        // Convert Vec<Disc> to Iterator of Strings
        let discs = self.stack.iter()
                              .map(|x| format!("{}", x))
                              .rev();
        // Create the required amount of padding, and chain the `discs` iterator
        // of strings onto the end of this padding.
        //
        // Note that I'm collecting into a Vec<String>, as attempting to return
        // the iterator directly yields a terribly long return type...
        (0..(self.capacity as usize - self.stack.len()))
            .map(|_| format!("{}", OptionalDisc::None(self.capacity)))
            .chain(discs)
            .collect::<Vec<_>>()
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
                write!(f, "{}", disc.repr)
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
    if disc_tally < 1 || disc_tally > 59 {
        eprintln!("Maximum number of Discs must be in the range of 1 - 59 inclusive.");
        exit(1);
    }
    // Clear the terminal
    println!("{}", clear::All);
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
    for (l, m, r) in izip!(source.get_peg_repr().iter(),
                           dest.get_peg_repr().iter(),
                           spare.get_peg_repr().iter()) {
        println!("┃{}┃{}┃{}┃", l, m, r);
    }
    // Sleep to ensure the board isn't redrawn too quickly
    // thread::sleep(time::Duration::from_millis(100));
}
