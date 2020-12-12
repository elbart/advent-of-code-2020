use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Eq, PartialEq)]
struct Ship{
    x: isize,
    y: isize,
    direction: Direction,
}


#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_degree(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 90,
            Self::South => 180,
            Self::West => 270,
        }
    }

    fn left(&self, deg: isize) -> Self {
        if deg.abs() > 270 {
            panic!("Unsupported degree - too big: {} (max |270|", deg);
        }

        Self::degree_to_direction(self.from_degree() as isize - deg)
    }

    fn right(&self, deg: isize) -> Self {
        if deg.abs() > 270 {
            panic!("Unsupported degree - too big: {} (max |270|", deg);
        }

        Self::degree_to_direction(self.from_degree() as isize + deg)
    }

    fn degree_to_direction(deg: isize) -> Self {
        let final_deg;
        if deg < 0 {
            final_deg = 360 + deg;
        } else if deg > 360 {
            final_deg = deg - 360;
        } else if deg == 360 {
            final_deg = 0;
        } else {
            final_deg = deg;
        }

        println!("final: {}", final_deg);

        match final_deg {
            0 => Self::North,
            90 => Self::East,
            180 => Self::South,
            270 => Self::West,
            x => panic!("Unsupported degree: {}", x),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::North => "N",
            Self::East => "E",
            Self::South => "S",
            Self::West => "W",
        }
    }
}

impl Ship {
    fn apply(&mut self, instruction: String) {
        let (mut name, value) = instruction.split_at(1);
        let parsed_value = value.parse::<isize>().unwrap();

        if name == "F" {
            name = self.direction.to_str();
        }

        match name {
            "N" => {
                self.y -= parsed_value;
            },
            "S" => {
                self.y += parsed_value;
            },
            "E" => {
                self.x += parsed_value;
            },
            "W" => {
                self.x -= parsed_value;
            },
            "L" => {
                self.direction = self.direction.left(parsed_value);
            },
            "R" => {
                self.direction = self.direction.right(parsed_value);
            }
            x => panic!("Unsupported action: {}", x)
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn main() {
    let f = File::open("src/12/input.txt").unwrap();
    let mut s = Ship{x: 0, y:0, direction: Direction::East};

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
    fn test_direction() {
        assert_eq!(Direction::East, Direction::degree_to_direction(90));
        assert_eq!(Direction::North, Direction::degree_to_direction(0));

        assert_eq!(Direction::South, Direction::East.left(270));
        assert_eq!(Direction::North, Direction::East.right(270));
    }
}