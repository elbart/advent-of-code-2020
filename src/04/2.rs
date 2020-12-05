use regex::Regex;
use std::{convert::TryFrom, fs::File, io::BufRead, io::BufReader};
#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Length,
    hcl: HexColor,
    ecl: NamedColor,
    pid: &'static str,
}

#[derive(Debug)]
enum Length {
    CM(usize),
    IN(usize),
}

impl TryFrom<&str> for Length {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let idx = value.find(|c: char| c.is_alphabetic()).ok_or(())?;
        let (num, unit) = value.split_at(idx);
        let length = num.parse::<usize>().unwrap();

        match unit {
            "in" => {
                if length < 59 || length > 76 {
                    return Err(());
                }

                Ok(Length::IN(length))
            }
            "cm" => {
                if length < 150 || length > 193 {
                    return Err(());
                }

                Ok(Length::CM(length))
            }
            _ => return Err(()),
        }
    }
}

#[derive(Debug)]
struct HexColor(String);

impl TryFrom<&str> for HexColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#[0-9,a-f]{6}").unwrap();
        }

        if RE.is_match(value) {
            return Ok(Self(value.to_string()));
        }

        Err(())
    }
}

#[derive(Debug)]
enum NamedColor {
    AMB,
    BLU,
    BRN,
    GRY,
    GRN,
    HZL,
    OTH,
}

impl TryFrom<&str> for NamedColor {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "amb" => Ok(Self::AMB),
            "blu" => Ok(Self::BLU),
            "brn" => Ok(Self::BRN),
            "gry" => Ok(Self::GRY),
            "grn" => Ok(Self::GRN),
            "hzl" => Ok(Self::HZL),
            "oth" => Ok(Self::OTH),
            _ => Err(()),
        }
    }
}

impl Passport {
    fn parse(value: String) -> Result<(), ()> {
        let errors: Vec<Result<(&str, &str), (&str, &str)>> = value
            .split(|c: char| c.is_whitespace())
            .into_iter()
            .map(|t| {
                let kv_pair: Vec<&str> = t.split(":").collect();
                assert_eq!(2, kv_pair.len());
                let (key, value) = (kv_pair[0], kv_pair[1]);

                match key {
                    "byr" => {
                        let typed_value = value.parse::<usize>().unwrap();
                        if typed_value < 1920 || typed_value > 2002 {
                            return Err((key, value));
                        }
                    }
                    "iyr" => {
                        let typed_value = value.parse::<usize>().unwrap();
                        if typed_value < 2010 || typed_value > 2020 {
                            return Err((key, value));
                        }
                    }
                    "eyr" => {
                        let typed_value = value.parse::<usize>().unwrap();
                        if typed_value < 2020 || typed_value > 2030 {
                            return Err((key, value));
                        }
                    }
                    "hgt" => Length::try_from(value)
                        .map(|_x| ())
                        .map_err(|_x| (key, value))?,
                    "hcl" => HexColor::try_from(value)
                        .map(|_x| ())
                        .map_err(|_x| (key, value))?,
                    "ecl" => NamedColor::try_from(value)
                        .map(|_x| ())
                        .map_err(|_x| (key, value))?,
                    "pid" => {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
                        }
                        if !RE.is_match(value) {
                            return Err((key, value));
                        }
                    }
                    _ => (),
                }

                Ok((key, value))
            })
            // .inspect(|v| println!("{:?}", v))
            .filter(|v| v.is_err())
            .collect();

        if errors.len() > 0 {
            // println!("{:?}", errors);
            return Err(());
        }

        Ok(())
    }
}

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
        // minimum required fields.
        if required
            .iter()
            .find(|&&field| !data.contains(&format!("{}:", field)))
            .is_none()
        {
            // all required fields are existing, let's try to parse a Passport
            if Passport::parse(data.trim_end().to_string()).is_ok() {
                valid_passports += 1;
            }
        }
    }

    println!("Valid passports: {}", valid_passports);
}
