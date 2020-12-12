use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/12/input.txt").unwrap();

    for line in BufReader::new(f).lines() {
        let _l = line.unwrap();
    }

    println!("Manhattan distance: {}", 0);
}
