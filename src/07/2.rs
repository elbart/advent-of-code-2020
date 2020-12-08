use std::{fs::File, io::BufRead, io::BufReader, rc::Rc};

const COLOUR: &str = "shiny gold";

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Bag {
    colour: String,
    count: Option<usize>,
    contains: Option<Vec<Rc<Bag>>>,
}

fn parse_line(line: String) -> Bag {
    let tokens: Vec<&str> = line.split(" bags contain ").collect();
    assert_eq!(2, tokens.len());
    let (colour, tail) = (tokens[0], tokens[1]);

    let mut c = Bag {
        colour: colour.to_string(),
        count: None,
        contains: None,
    };

    match tail.trim() {
        "no other bags." => (),
        _ => {
            let sub_tokens: Vec<&str> = tail.split(",").collect();
            c.contains = Some(
                sub_tokens
                    .iter()
                    .map(|&t| {
                        let tokens: Vec<&str> = t.trim().split(' ').collect();
                        assert_eq!(4, tokens.len());

                        Rc::new(Bag {
                            count: Some(tokens[0].parse::<usize>().unwrap()),
                            colour: format!("{} {}", tokens[1], tokens[2]),
                            contains: None,
                        })
                    })
                    .collect(),
            );
        }
    }

    c
}

fn count_subordinate_bags(
    bags: &Vec<Rc<Bag>>,
    bag: Rc<Bag>,
    multi: usize,
    bag_count: usize,
) -> usize {
    let mut bc = bag_count;
    bag.contains
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .for_each(|b| {
            let real_bag = bags
                .iter()
                .find(|&full_bag| b.colour == full_bag.colour)
                .unwrap();
            bc = count_subordinate_bags(bags, real_bag.clone(), multi * b.count.unwrap(), bc);
            bc += b.count.unwrap() * multi;
        });
    bc
}

fn main() {
    let f = File::open("src/07/input.txt").unwrap();
    let mut bags = Vec::new();
    let mut shiny_gold_bag = None;

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        let bag = Rc::new(parse_line(l));
        bags.push(bag.clone());

        if bag.colour.as_str() == COLOUR {
            shiny_gold_bag = Some(bag.clone());
        }
    }

    let bag_count = count_subordinate_bags(&bags, shiny_gold_bag.unwrap(), 1, 0);

    println!("Subordinate bag count is: {}", bag_count);
}
