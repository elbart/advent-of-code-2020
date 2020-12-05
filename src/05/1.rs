use std::{fs::File, io::BufRead, io::BufReader};

fn parse_data(raw: &str, low: char, max: usize) -> usize {
    let mut range = (0, max);
    for c in raw.chars() {
        if c == low {
            range = (range.0, ((range.1 + 1 - range.0) / 2) - 1 + range.0);
        } else {
            range = (((range.1 + 1 - range.0) / 2) + range.0, range.1);
        }
    }

    range.0
}

fn parse_row(raw_row: &str) -> usize {
    parse_data(raw_row, 'F', 127)
}

fn parse_column(raw_column: &str) -> usize {
    parse_data(raw_column, 'L', 7)
}

fn main() {
    let f = File::open("src/05/input.txt").unwrap();
    let mut biggest_seat_id = 0;

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        assert_eq!(10, l.len());
        let (raw_row, raw_column) = l.split_at(7);
        let seat_id = parse_row(raw_row) * 8 + parse_column(raw_column);

        if seat_id > biggest_seat_id {
            biggest_seat_id = seat_id;
        }
    }

    println!("Biggest seat ID: {}", biggest_seat_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        assert_eq!(70, parse_row("BFFFBBF"));
        assert_eq!(14, parse_row("FFFBBBF"));
        assert_eq!(102, parse_row("BBFFBBF"));
    }
    
    #[test]
    fn test_parse_column() {
        assert_eq!(7, parse_column("RRR"));
        assert_eq!(4, parse_column("RLL"));
    }
}