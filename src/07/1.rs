use std::{collections::HashSet, fmt::Debug, fs::File, io::BufRead, io::BufReader, rc::Rc};

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

fn get_bags_for(bags: &Vec<Rc<Bag>>, colour: String, unique_colors: &mut HashSet<Rc<Bag>>) {
    bags
        .iter()
        .cloned()
        .filter(|c| {
            if c.contains.is_some() {
                return c.contains.as_ref().unwrap().iter().find(|&c| {
                    c.colour.as_str() == colour
                }).is_some()
            }
            
            false
        })
        .map(|c| c.clone())
        .for_each(|c| {
            unique_colors.insert(c.clone());
            get_bags_for(&bags, c.colour.clone(), unique_colors)
        })
}

fn main() {
    let f = File::open("src/07/input.txt").unwrap();
    let mut bags = Vec::new();

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        bags.push(Rc::new(parse_line(l)));
    }

    let mut unique_colors = HashSet::new();
    get_bags_for(&bags, COLOUR.to_string(), &mut unique_colors);

    println!("{}", unique_colors.len());
}
