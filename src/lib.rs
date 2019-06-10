/* Crates */
#[macro_use] extern crate itertools;
extern crate termion;

/* Imports */
use std::{fmt,thread,time};
use std::cmp::Ordering;
use std::process::exit;
use termion::{cursor, clear};

/* Types */
// `Config` struct, used for holding game configuration
#[derive(Debug)]
struct Config {
    game_size: u8,
    refresh_interval: u64,
    empty_slot_repr: String,
}

impl Config {
    fn new(game_size: u8, refresh_interval: u64) -> Self {

        // write!(f, "{:^width$}", " ┃", width = (*i as usize * 2) + i.to_string().len())
        Config {
            game_size,
            refresh_interval,
            empty_slot_repr: format!("{:^width$}", " ┃", width = (game_size as usize * 2) + game_size.to_string().len()),
        }

    }
}

// `OptionalDisc`:
//     The `u8` carried by the `Disc::Empty` variant signifies the capacity of the `Peg` (A.K.A the
//     largest `Disc` on the `Board`).
#[derive(Debug, Clone)]
enum OptionalDisc<'a> {
    Some(Disc),
    None(&'a Config),
}

// Concrete `Disc` type
#[derive(Debug, Clone)]
struct Disc {
    size: u8,
    max: u8,
    repr: String,
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

// `Peg` represents one of three vertical pegs in a game board
#[derive(Debug, Clone)]
// FIXME: Add a lifetime specifier for the borrowed Config within the OptionalDisc
struct Peg<'a> {
    label: PegLabel,
    capacity: u8,
    stack: Vec<OptionalDisc<'a>>,
}

impl<'a> Peg<'a> {
    // Associated function which constructs a `Peg` loaded with `OptionalDisc::Some`s
    fn new(label: PegLabel, capacity: u8) -> Self {
        Peg {
            label,
            capacity: capacity,
            stack: (0..capacity).map(|x| OptionalDisc::Some(Disc::new(x, capacity)))
                                .rev()
                                .collect::<Vec<_>>(),
        }
    }

    // Associated function which constructs a `Peg` loaded with `OptionalDisc::None`s
    fn new_empty(label: PegLabel, capacity: u8) -> Self {
        Peg {
            label,
            capacity: capacity,
            stack: Vec::with_capacity(capacity as usize),
        }
    }

    // This method returns a `Vec<String>` representing each `OptionalDisc` on the peg
    fn get_peg_repr(&self, config: &Config) -> Vec<String> {
        // Convert Vec<Disc> to Iterator of Strings
        let discs = self.stack.iter()
                              .map(|x| format!("{}", x))
                              .rev();
        // Create the required amount of padding, and chain the `discs` iterator
        // of strings onto the end of this padding.
        //
        // Note that I'm collecting into a Vec<String>, as attempting to return
        // the iterator directly yields a terribly long return type...
        (0..(config.game_size as usize - self.stack.len()))
            .map(|_| format!("{}", OptionalDisc::None(config)))
            .chain(discs)
            .collect::<Vec<_>>()
    }
}

impl<'a> PartialOrd for Peg<'a> {
    fn partial_cmp(&self, other: &Peg<'a>) -> Option<Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

impl<'a> PartialEq for Peg<'a> {
    fn eq(&self, other: &Peg<'a>) -> bool {
        self.label == other.label
    }
}

impl<'a> Ord for Peg<'a> {
    fn cmp(&self, other: &Peg<'a>) -> Ordering {
        self.label.cmp(&other.label)
    }
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
struct Board<'a> (
    Peg<'a>,
    Peg<'a>,
    Peg<'a>,
);

// Note that I have implemented the `Eq` trait here, rather than derived it, as deriving it would
// necessitate that all `Peg` struct members would need to implement this trait as well.
// From the docs:
//
// "This trait can be used with #[derive]. When derived, because Eq has no extra methods, it is only
// informing the compiler that this is an equivalence relation rather than a partial equivalence
// relation. Note that the derive strategy requires all fields are Eq, which isn't always desired."
//
// From the [Rust docs](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
impl<'a> Eq for Peg<'a> {}

impl<'a> Board<'a> {
    // Associated function (which constructs a `Board`)
    fn new(capacity: u8) -> Self {
        Board (
            Peg::new(PegLabel::Left, capacity),
            Peg::new_empty(PegLabel::Middle, capacity),
            Peg::new_empty(PegLabel::Right, capacity),
        )
    }
}

impl<'a> fmt::Display for OptionalDisc<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OptionalDisc::Some(disc) => {
                write!(f, "{}", disc.repr)
            },
            OptionalDisc::None(slot) => {
                // write!(f, "{:^width$}", " ┃", width = (*i as usize * 2) + i.to_string().len())
                write!(f, "{}", slot.empty_slot_repr)
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
    let config = Config::new(game_size, refresh_interval);

    // Clear the terminal
    println!("{}{}", clear::All, cursor::Hide);
    let Board (
        mut left,
        mut middle,
        mut right,
    ) = Board::new(config.game_size);

    display_board(&left, &middle, &right, &config);

    // Note that we must subtract `1` from game_size, to convert between the number of discs, and
    // the size of the largest disc (example: 10 discs, 9 is the largest (0-indexed)).
    // If you _don't_ subtract 1, you'll panic thanks to an out-by-one error.
    move_tower(game_size - 1, &mut left, &mut right, &mut middle, &config);

    display_board(&left, &middle, &right, &config);

    println!("{}", cursor::Show);
}

// Implements an approximation of the famous algorithm which solves the
// "Towers of Hanoi" game using recursion. I've yet to determine the original
// author of this bad boy.
fn move_tower<'a>(disc_size: u8, source: &mut Peg<'a>, dest: &mut Peg<'a>, spare: &mut Peg<'a>, config: &Config) {
    if disc_size == 0 {
        if let Some(i) = source.stack.pop() {
            display_board(source, dest, spare, &config);
            dest.stack.push(i);
            display_board(source, dest, spare, &config);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        move_tower(disc_size - 1, source, spare, dest, &config);
        if let Some(i) = source.stack.pop() {
            display_board(source, dest, spare, &config);
            dest.stack.push(i);
            display_board(source, dest, spare, &config);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        move_tower(disc_size - 1, spare, dest, source, &config);
        display_board(source, dest, spare, &config);
    }
}

// `Board` display function
fn display_board(source: &Peg, dest: &Peg, spare: &Peg, config: &Config) {
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
    for (l, m, r) in izip!(source.get_peg_repr(config).iter(),
                           dest.get_peg_repr(config).iter(),
                           spare.get_peg_repr(config).iter()) {
        println!("{}{}{}", l, m, r);
    }

    // Sleep to ensure the board isn't redrawn too quickly
    thread::sleep(time::Duration::from_millis(config.refresh_interval));
}
