use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Eq, PartialEq)]
struct Waypoint {
    x: isize,
    y: isize,
}

#[derive(Debug, Eq, PartialEq)]
struct Ship {
    x: isize,
    y: isize,
    waypoint: Waypoint,
}

impl Waypoint {
    fn right(&mut self, deg: isize) {
        if deg.abs() > 270 {
            panic!("Unsupported degree - too big: {} (max |270|", deg);
        }

        let tmp_x;

        match deg {
            90 => {
                tmp_x = self.x;
                self.x = -self.y;
                self.y = tmp_x;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                tmp_x = self.x;
                self.x = self.y;
                self.y = -tmp_x;
            }
            x => panic!("Unsupported degree: {}", x),
        }
    }

    fn left(&mut self, deg: isize) {
        if deg.abs() > 270 {
            panic!("Unsupported degree - too big: {} (max |270|", deg);
        }

        let mut final_deg = deg;

        if deg == 90 {
            final_deg = 270
        } else if deg == 270 {
            final_deg = 90;
        }

        self.right(final_deg);
    }
}

impl Ship {
    fn apply(&mut self, instruction: String) {
        let (action, raw_value) = instruction.split_at(1);
        let value = raw_value.parse::<isize>().unwrap();

        match action {
            "N" => {
                self.waypoint.y -= value;
            }
            "S" => {
                self.waypoint.y += value;
            }
            "E" => {
                self.waypoint.x += value;
            }
            "W" => {
                self.waypoint.x -= value;
            }
            "L" => {
                self.waypoint.left(value);
            }
            "R" => {
                self.waypoint.right(value);
            }
            "F" => {
                self.x += self.waypoint.x * value;
                self.y += self.waypoint.y * value;
            }
            x => panic!("Unsupported action: {}", x),
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn main() {
    let f = File::open("src/12/input.txt").unwrap();
    let mut s = Ship {
        x: 0,
        y: 0,
        waypoint: Waypoint { x: 10, y: -1 },
    };

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        s.apply(l);
    }

    println!("Manhattan distance: {}", s.manhattan_distance());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint() {
        let mut wp = Waypoint { x: 10, y: -4 };
        wp.right(90);
        assert_eq!(Waypoint { x: 4, y: 10 }, wp);

        wp.left(90);
        assert_eq!(Waypoint { x: 10, y: -4 }, wp);

        wp.left(180);
        assert_eq!(Waypoint { x: -10, y: 4 }, wp);

        wp.right(180);
        assert_eq!(Waypoint { x: 10, y: -4 }, wp);

        wp.right(270);
        assert_eq!(Waypoint { x: -4, y: -10 }, wp);
    }
}
