use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/10/input.txt").unwrap();

    let mut numbers: Vec<usize> = Vec::new();
    let mut jolt_1 = 1;
    let mut jolt_3 = 1;

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        numbers.push(l.parse().unwrap());
    }

    numbers.sort();

    let mut it = numbers.iter().peekable();

    while let Some(n) = it.next() {
        if let Some(&&next) = it.peek() {
            if next - n == 1 {
                jolt_1 += 1;
            } else if next - n == 3 {
                jolt_3 += 1;
            } else {
                panic!(
                    "Difference to the next one is neither 1 nor 3: {} - {} = {}",
                    next,
                    n,
                    next - n
                );
            }
        }
    }

    println!(
        "The product of 1jolts {} and 3jolts {} is {}",
        jolt_1,
        jolt_3,
        jolt_1 * jolt_3
    );
}
