use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let f = File::open("src/13/input.txt").unwrap();

    let mut it = BufReader::new(f).lines();
    let ts = it.next().unwrap().unwrap().parse::<usize>().unwrap();
    let nearest = it.next().unwrap().unwrap().split(',').filter_map(|c| {
        if c == "x" {
            None
        } else {
            let id = c.parse::<usize>().unwrap();
            Some((id, ts - (ts%id) + id))
        }
    }).min_by(|x, y| x.1.cmp(&y.1)).unwrap();

    println!("Bus ID {} departs: {}. Factor is: {}", nearest.0, nearest.1, nearest.0 * (nearest.1 - ts));
}
