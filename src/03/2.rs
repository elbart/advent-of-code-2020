use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let mut tree_list: Vec<u64> = vec![];
    let cfg = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for cfg_item in cfg {
        let mut trees = 0;
        let mut pos = 0;
        let f = File::open("src/03/input.txt").unwrap();
        let mut lines = BufReader::new(f).lines();

        // skip first line, just determine the length, assuming the length is constant for
        // all subsequent lines
        let l1 = lines.next().expect("xxx");
        let len = l1.unwrap().len();

        for (idx, line) in lines.enumerate().by_ref() {
            // check if we need to skip that line based on the config
            if (idx + 1) % cfg_item.1 != 0 {
                continue;
            }

            let l = line.unwrap();
            let new_pos = (pos + cfg_item.0) % len;
            let c = l.chars().nth(new_pos).unwrap();
            if c == '#' {
                trees += 1;
            }

            pos = new_pos;
        }

        tree_list.push(trees);
    }

    println!(
        "Matched trees product: {}. Individual tree numbers per slope {:?}",
        tree_list.iter().fold(1, |acc, t| acc * t),
        tree_list
    );
}
