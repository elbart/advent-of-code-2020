use std::{collections::HashMap, collections::HashSet, fs::File, io::BufRead, io::BufReader};

fn compute_results(group_answers: &Vec<HashSet<char>>) -> usize {
    let len = group_answers.len();
    group_answers
        .iter()
        .fold(HashMap::<char, usize>::new(), |mut acc, x| {
            x.iter().for_each(|c| {
                let d = acc.entry(*c).or_insert(0);
                *d += 1;
            });

            acc
        })
        .iter()
        .filter(|(&_k, &v)| v == len)
        .count()
}

fn compute_results_intersect(group_answers: &Vec<HashSet<char>>) -> usize {
    // note: I would have used fold_first() and would have saved the
    // `init` variable creation, but it's not available in rust-stable
    let it = group_answers.iter();
    let init = group_answers
        .iter()
        .next()
        .unwrap_or(&HashSet::new())
        .clone();

    it.into_iter()
        .fold(init, |acc, x| acc.intersection(x).cloned().collect())
        .iter()
        .count()
}

fn main() {
    let f = File::open("src/06/input.txt").unwrap();
    let mut final_sum = 0;
    let mut final_sum_isect = 0;

    let mut group_answers = Vec::new();
    for line in BufReader::new(f).lines() {
        let l = line.unwrap();

        if l.len() > 0 {
            group_answers.push(l.chars().collect::<HashSet<char>>());
            continue;
        }

        final_sum += compute_results(&group_answers);
        final_sum_isect += compute_results_intersect(&group_answers);
        group_answers.clear();
    }

    // collect results from last group, too because we do not reach this
    // computation, since there is no last line...
    final_sum += compute_results(&group_answers);
    final_sum_isect += compute_results_intersect(&group_answers);

    assert_eq!(final_sum_isect, final_sum);

    println!(
        "The sum of numbers of questions to which everyone in each group answered yes: {}",
        final_sum
    );
}
