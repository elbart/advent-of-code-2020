use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/03/input.txt").unwrap();
    let mut valid_pws = 0;
    for line in BufReader::new(f).lines() {
        let l = line.unwrap();

        let tokens: Vec<&str> = l.split(" ").collect();
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 3);

        // min + max
        let minmax: Vec<&str> = tokens[0].split('-').collect();
        assert_eq!(minmax.len(), 2);
        let (min, max) = (
            minmax[0].parse::<usize>().unwrap(),
            minmax[1].parse::<usize>().unwrap(),
        );

        // character
        let character = &tokens[1][..1];

        // password
        let password = tokens[2];

        let cnt = password.matches(character).collect::<Vec<&str>>().len();

        if cnt >= min && cnt <= max {
            valid_pws += 1;
        }
    }

    println!("Valid password count: {}", valid_pws);
}
