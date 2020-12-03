use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/03/input.txt").unwrap();
    let mut trees = 0;
    let mut pos = 0;
    let mut lines = BufReader::new(f).lines();

    // skip first line, just determine the length, assuming the length is constant for
    // all subsequent lines
    let l1 = lines.next().unwrap().unwrap();
    let len = l1.len();

    for line in lines {
        let l = line.unwrap();
        let new_pos = (pos + 3) % len;
        let c = l.chars().nth(new_pos).unwrap();
        if c == '#' {
            trees += 1;
        }

        pos = new_pos;
    }

    println!("Matched trees: {}", trees);
}
