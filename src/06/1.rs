use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/06/input.txt").unwrap();
    let mut final_sum = 0;

    let mut group_answers = HashSet::new();
    for line in BufReader::new(f).lines() {
        let l = line.unwrap();

        if l.len() > 0 {
            group_answers = group_answers
                .union(&l.chars().collect::<HashSet<char>>())
                .cloned()
                .collect();
            continue;
        }

        final_sum += group_answers.len();
        group_answers.clear();
    }

    // collect results from last group, too because we do not reach this
    // computation, since there is no last line...
    final_sum += group_answers.len();

    println!(
        "The sum of numbers of questions to which anyone answered yes: {}",
        final_sum
    );
}
