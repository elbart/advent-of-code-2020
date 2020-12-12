use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::BufRead,
    io::BufReader,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum SeatState {
    Empty,
    Floor,
    Occupied,
}

impl Display for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "L"),
            Self::Floor => write!(f, "."),
            Self::Occupied => write!(f, "#"),
        }
    }
}

impl From<char> for SeatState {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Unsupported Seat type: '{}'", c),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Point(usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Board {
    data: HashMap<Point, SeatState>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
struct Neighbours {
    data: HashSet<(Point, SeatState)>,
}

impl Neighbours {
    fn more_than_x(&self, num: usize, s: &SeatState) -> bool {
        self.data.iter().filter(|(_p, state)| s == state).count() > num
    }

    fn all_empty(&self) -> bool {
        self.data
            .iter()
            .all(|(_p, state)| SeatState::Empty == *state || SeatState::Floor == *state)
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl Deref for Board {
    type Target = HashMap<Point, SeatState>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Board {
    fn get_neighbours(&self, p: &Point) -> Neighbours {
        let x_min = max(p.0 as i32 - 1, 0) as usize;
        let x_max = min(p.0 + 2, self.cols);

        let y_min = max(p.1 as i32 - 1, 0) as usize;
        let y_max = min(p.1 + 2, self.rows);
        let mut neighbours = HashSet::new();

        // println!("xmin: {}, xmax: {}", x_min, x_max);
        // println!("ymin: {}, ymax: {}", y_min, y_max);
        for y in y_min..y_max {
            for x in x_min..x_max {
                // println!("x:{}, y:{}", x, y);
                if p.0 == x && p.1 == y {
                    continue;
                }
                let p = Point(x, y);
                neighbours.insert((p.clone(), self.get(&p).unwrap().clone()));
            }
        }

        Neighbours { data: neighbours }
    }

    fn get_new_seat_state(&self, p: &Point, s: &SeatState) -> SeatState {
        match s {
            SeatState::Floor => SeatState::Floor,
            SeatState::Empty => {
                if self.get_neighbours(&p).all_empty() {
                    SeatState::Occupied
                } else {
                    SeatState::Empty
                }
            }
            SeatState::Occupied => {
                if self.get_neighbours(&p).more_than_x(3, &SeatState::Occupied) {
                    SeatState::Empty
                } else {
                    SeatState::Occupied
                }
            }
        }
    }

    fn compute_new_board(&self) -> Board {
        let mut new_board = self.clone();
        self.iter().for_each(|(p, s)| {
            let s = self.get_new_seat_state(p, s);
            *new_board.get_mut(p).unwrap() = s;
        });

        new_board
    }

    fn get_occupied_seats(&self) -> usize {
        self.iter()
            .filter(|&(_p, s)| *s == SeatState::Occupied)
            .count()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.rows {
            for x in 0..self.cols {
                write!(f, "{}", self.data.get(&Point(x, y)).unwrap())?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let f = File::open("src/11/input.txt").unwrap();
    let mut board = Board {
        data: HashMap::new(),
        rows: 0,
        cols: 0,
    };

    for (y, line) in BufReader::new(f).lines().enumerate() {
        board.rows = y + 1;
        let l = line.unwrap();
        for (x, c) in l.char_indices() {
            board.cols = x + 1;
            let s = SeatState::from(c);
            board.insert(Point(x, y), s);
        }
    }
    // println!("{:#?}", board);

    let mut current: Board = board.clone();
    // println!("{}", board);
    // println!("=============================================");
    // current = board.compute_new_board();
    // println!("{}", current);
    // println!("=============================================");
    // current = current.compute_new_board();
    // println!("{}", current);
    // println!("=============================================");
    // current = current.compute_new_board();
    // println!("{}", current);
    loop {
        let new = current.compute_new_board();
        if new == current {
            break;
        }

        current = new;
    }

    println!("Occupied seats: {}", current.get_occupied_seats());
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn bla() {
        let f = File::open("src/11/input.txt").unwrap();
        let mut board = Board {
            data: HashMap::new(),
            rows: 0,
            cols: 0,
        };

        for (y, line) in BufReader::new(f).lines().enumerate() {
            board.rows = y + 1;
            let l = line.unwrap();
            for (x, c) in l.char_indices() {
                board.cols = x + 1;
                let s = SeatState::from(c);
                board.insert(Point(x, y), s);
            }
        }

        // println!("{:?}", board.get_neighbours(&Point(0, 0)));
        // println!("{:?}", board.get_neighbours(&Point(9, 9)));
        assert_eq!(3, board.get_neighbours(&Point(0, 0)).len());
        assert_eq!(5, board.get_neighbours(&Point(1, 0)).len());
        assert_eq!(8, board.get_neighbours(&Point(1, 1)).len());
        assert_eq!(3, board.get_neighbours(&Point(9, 9)).len());
        assert_eq!(5, board.get_neighbours(&Point(9, 8)).len());
    }
}
