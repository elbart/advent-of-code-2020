use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/04/input.txt").unwrap();
    let mut valid_passports = 0_u32;
    let mut lines = BufReader::new(f).lines();
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    while let Some(line) = lines.next() {
        let mut l = line.unwrap();
        if l.len() == 0 {
            continue;
        }

        let mut data = String::new();
        // push first chunk
        data.push_str(&format!("{} ", l.as_str()));

        // search for additional lines / chunks
        while let Some(line) = lines.next() {
            l = line.unwrap();
            if l.len() == 0 {
                break;
            }

            data.push_str(&format!("{} ", l.as_str()));
        }

        // we have a full passport string here, let's search for
        // minimum required fields
        if required
            .iter().all(|&field| data.contains(&format!("{}:", field)))
        {
            valid_passports += 1;
        }
    }

    println!("Valid passports: {}", valid_passports);
}
