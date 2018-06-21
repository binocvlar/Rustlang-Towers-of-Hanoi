/* Crates */
#[macro_use] extern crate itertools;
extern crate termion;

/* Imports */
use std::{fmt,thread,time};
use std::cmp::Ordering;
use std::process::exit;
use termion::{cursor, clear};
use std::rc::Rc;

/* Types */
// `Config` struct, used for holding game configuration
struct Config {
    refresh_interval: u64,
}

// `OptionalDisc`:
//     The `u8` carried by the `Disc::Empty` variant signifies the capacity of the `Peg` (A.K.A the
//     largest `Disc` on the `Board`).
#[derive(Debug, Clone)]
enum OptionalDisc {
    Some(Disc),
    None(u8),
}

// Concrete `Disc` type
#[derive(Debug, Clone)]
struct Disc {
    size: u8,
    max: u8,
    repr: String,
}

// `Peg` represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
struct Peg {
    label: PegLabel,
    capacity: u8,
    stack: Vec<OptionalDisc>,
}

// `PegLabel`: The ordering of each variant in the definition below is responsible for the ordering
// of `Peg`'s within a `Board`:
//
// "When derived on enums, variants are ordered by their top-to-bottom declaration order."*
// * From https://doc.rust-lang.org/std/cmp/trait.Ord.html
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum PegLabel {
    Left,
    Middle,
    Right,
}

// `Board` represents a fixed configuration of three pegs
#[derive(Debug, Clone)]
struct Board (
    Peg,
    Peg,
    Peg,
);

/* Implementations */
impl Config {
    fn new(refresh_interval: u64) -> Rc<Config> {
        Rc::new(Config { refresh_interval })
    }
}

impl Disc {
    // Associated function which constructs a new `Disc`
    fn new(size: u8, max: u8) -> Self {
        let label_len = size.to_string().len() as u8;
        let disc_width = 2 * size + label_len - (label_len - 1);

        // Construct the representation of the Disc (e.g. "--2--")
        let disc_repr = format!(" {:▬^width$}", size, width = disc_width as usize);

        // Pad the disc_repr with whitespace (e.g. " --2-- " for a 4 disc game)
        let total_width = 2 * max + max.to_string().len() as u8;
        let repr = format!("{:^width$}", disc_repr, width = total_width as usize);
        Disc {
            size,
            max,
            repr,
        }
    }
}

impl Peg {
    // Associated function which constructs a `Peg` loaded with `OptionalDisc::Some`s
    fn new(label: PegLabel, capacity: u8) -> Self {
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

impl Board {
    // Associated function (which constructs a `Board`)
    fn new(capacity: u8) -> Self {
        Board (
            Peg::new(PegLabel::Left, capacity),
            Peg::new_empty(PegLabel::Middle, capacity),
            Peg::new_empty(PegLabel::Right, capacity),
        )
    }
}

impl fmt::Display for OptionalDisc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionalDisc::Some(disc) => {
                write!(f, "{}", disc.repr)
            },
            OptionalDisc::None(i) => {
                write!(f, "{:^width$}", " ┃", width = (*i as usize * 2) + i.to_string().len())
            },
        }
    }
}

/* Functions */
pub fn solve_game(game_size: u8, refresh_interval: u64) {
    // Constrain the size of user input
    if game_size < 1 || game_size > 32 {
        eprintln!("Maximum number of Discs must be in the range of 1 - 32 inclusive.");
        exit(1);
    }

    // Get an Rc<Config>
    let config = Config::new(refresh_interval);

    // Clear the terminal
    println!("{}{}", clear::All, cursor::Hide);
    let Board (
        mut left,
        mut middle,
        mut right,
    ) = Board::new(game_size);

    display_board(&left, &middle, &right, Rc::clone(&config));

    // Note that we must subtract `1` from game_size, to convert between the number of discs, and
    // the size of the largest disc (example: 10 discs, 9 is the largest (0-indexed)).
    // If you _don't_ subtract 1, you'll panic thanks to an out-by-one error.
    move_tower(game_size - 1, &mut left, &mut middle, &mut right, Rc::clone(&config));

    display_board(&left, &middle, &right, Rc::clone(&config));

    println!("{}", cursor::Show);
}

// Implements an approximation of the famous algorithm which solves the
// "Towers of Hanoi" game using recursion. I've yet to determine the original
// author of this bad boy.
fn move_tower(disc_size: u8, source: &mut Peg, dest: &mut Peg, spare: &mut Peg, config: Rc<Config>) {
    if disc_size == 0 {
        if let Some(i) = source.stack.pop() {
            display_board(source, dest, spare, Rc::clone(&config));
            dest.stack.push(i);
            display_board(source, dest, spare, Rc::clone(&config));
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(disc_size - 1, source, spare, dest, Rc::clone(&config));
        if let Some(i) = source.stack.pop() {
            display_board(source, dest, spare, Rc::clone(&config));
            dest.stack.push(i);
            display_board(source, dest, spare, Rc::clone(&config));
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(disc_size - 1, spare, dest, source, Rc::clone(&config));
        display_board(source, dest, spare, Rc::clone(&config));
    }
}

// `Board` display function
fn display_board(source: &Peg, dest: &Peg, spare: &Peg, config: Rc<Config>) {
    // Get the `x` and `y` coordinates of the terminal. This is used in order to "jump back" and
    // redraw the board. Only the `y` coordinate is important in this case.
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
        println!("{}{}{}", l, m, r);
    }

    // Sleep to ensure the board isn't redrawn too quickly
    thread::sleep(time::Duration::from_millis(config.refresh_interval));
}
