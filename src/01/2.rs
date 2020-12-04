use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let mut numbers = vec![];
    let f = File::open("src/01/input.txt").unwrap();
    for line in BufReader::new(f).lines() {
        if let Ok(l) = line {
            numbers.push(l.parse::<usize>().unwrap())
        }
    }

    let numbers2 = numbers.clone();
    let numbers3 = numbers.clone();

    let mult = numbers
        .iter()
        .find_map(|n1| {
            for n2 in &numbers2 {
                for n3 in &numbers3 {
                    if n1 + n2 + n3 == 2020 {
                        return Some((n1, n2, n3));
                    }
                }
            }

            None
        })
        .unwrap();

    println!(
        "The numbers are {} and {} and {} and the product is: {}",
        mult.0,
        mult.1,
        mult.2,
        mult.0 * mult.1 * mult.2
    );
}
