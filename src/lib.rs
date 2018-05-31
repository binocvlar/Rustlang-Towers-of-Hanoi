#[derive(Debug)]
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

#[derive(Debug)]
pub struct Board {
    left: Peg,
    middle: Peg,
    right: Peg,
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

pub fn move_tower(disc: u8, source: &mut Vec<u8>, dest: &mut Vec<u8>, spare: &mut Vec<u8>) {
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

