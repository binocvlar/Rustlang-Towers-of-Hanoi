fn move_tower(disc: u8, source: &Vec<u8>, dest: &Vec<u8>, spare: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut source = source.clone();
    let mut dest = dest.clone();
    let mut spare = spare.clone();

    // Base case
    if disc == 0 {
        println!("DEBUG BASE CASE: {:?}", source);
        if let Some(i) = source.pop() {
            dest.push(i);
            println!("DEBUGa: _DISC_: {}, Left {:?}, Middle {:?}, Right {:?}", disc, source, dest, spare);
        } else {
            println!("DEBUGb: _DISC_: {}, Left {:?}, Middle {:?}, Right {:?}", disc, source, dest, spare);
            panic!("Unable to pop from \"source\" stack!");
        }
    } else {
        let (mut source, mut spare, mut dest) = move_tower(disc - 1, &source, &spare, &dest);
        println!("DEBUGc: _DISC_: {}, Left {:?}, Middle {:?}, Right {:?}", disc, source, dest, spare);
        if let Some(i) = source.pop() {
            dest.push(i);
        } else {
            panic!("Unable to pop from \"source\" stack!");
        }
        let (mut source, mut dest, mut spare) = move_tower(disc - 1, &spare, &dest, &source);
        println!("DEBUGd: _DISC_: {}, Left {:?}, Middle {:?}, Right {:?}", disc, source, dest, spare);
    }
    (source, dest, spare)
}

fn main() {
    let output = "Left {}, Middle {}, Right {}";
    let left: Vec<u8> = vec![2, 1, 0];
    let middle: Vec<u8> = vec![];
    let right: Vec<u8> = vec![];

    let disc: u8 = 2;

    println!("BEGIN: _DISC_: {}, Left {:?}, Middle {:?}, Right {:?}", disc, left, middle, right);
    let (source, dest, spare) = move_tower(disc, &left, &middle, &right);
    println!("END: _DISC_: {}, Left {:?}, Middle {:?}, Right {:?}", disc, source, dest, spare);
}

