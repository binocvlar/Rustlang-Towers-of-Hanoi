use std::fmt;

#[derive(Debug, Clone)]
pub struct Peg(Vec<u8>);

impl Peg {
    pub fn new(largest_disc: Option<u8>) -> Self {
        match largest_disc {
            Some(i) => {
                let mut discs = (0..i + 1).rev().collect::<Vec<u8>>();
                Peg(discs)
            },
            None => Peg(vec![]),
        }
    }
}

/*
impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}
*/

#[derive(Debug, Clone)]
pub struct Board {
    pub left: Peg,
    pub middle: Peg,
    pub right: Peg,
}

impl Board {
    pub fn new(disc_count: u8) -> Self {
        Board {
            left: Peg::new(Some(disc_count)),
            middle: Peg::new(None),
            right: Peg::new(None),
        }
    }
}

pub fn solve_game(disc: u8, board: &Board) -> Board {
    let board2 = board.clone();
    let Board {
        mut left,
        mut middle,
        mut right,
    } = board2;

    move_tower(disc, &mut left, &mut middle, &mut right);
    Board { left, middle, right }
}

pub fn move_tower(disc: u8, source: &mut Peg, dest: &mut Peg, spare: &mut Peg) {
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

