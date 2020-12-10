use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn find_valid_combinations(list: &[usize], computed: &mut HashMap<usize, u64>) -> u64 {
    if list.len() <= 1 {
        return 1;
    }

    let mut it = list.iter();
    let mut combinations = 0;
    let mut pos = 0;
    let current = list[0];
    it.next();

    while let Some(&el) = it.next() {
        if el - current > 3 {
            break;
        }

        pos += 1;

        combinations += match computed.get(&el) {
            Some(&x) => x,
            None => {
                let combos = find_valid_combinations(&list[pos..], computed);
                computed.insert(el, combos);
                combos
            }
        }
    }

    combinations
}

fn main() {
    let f = File::open("src/10/input.txt").unwrap();

    let mut numbers: Vec<usize> = Vec::new();
    numbers.push(0);

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        numbers.push(l.parse().unwrap());
    }

    numbers.sort();
    numbers.push(numbers.iter().max().unwrap() + 3);

    println!(
        "Valid combinations are: {}",
        find_valid_combinations(&numbers[0..], &mut HashMap::new())
    );
}
