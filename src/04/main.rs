use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/04/input.txt").unwrap();
    let mut valid_pws = 0;
    for line in BufReader::new(f).lines() {
        let l = line.unwrap();

        let tokens: Vec<&str> = l.split(" ").collect();
        assert_eq!(tokens.len(), 3);

        // pos1, pos2
        let positions: Vec<&str> = tokens[0].split('-').collect();
        assert_eq!(positions.len(), 2);
        let (pos1, pos2) = (
            positions[0].parse::<usize>().unwrap() - 1,
            positions[1].parse::<usize>().unwrap() - 1,
        );

        // character
        let character = &tokens[1][..1];

        // password
        let password = tokens[2];
        let pw_length = password.len();

        if pos1 > pw_length || pos2 > pw_length {
            continue;
        }

        let m1 = &password[pos1..pos1 + 1];
        let m2 = &password[pos2..pos2 + 1];

        if m1 == character {
            if m2 != character {
                valid_pws += 1;
            }
        } else if m2 == character {
            valid_pws += 1;
        }

        println!("{}, {}", m1, m2);
    }

    println!("Valid password count: {}", valid_pws);
}
