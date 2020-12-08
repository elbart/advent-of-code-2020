use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Sub,
}

#[derive(Debug)]
enum Instruction {
    NOP,
    ACC(Op, usize),
    JMP(Op, usize),
}

fn parse_instruction(s: String) -> Instruction {
    let (name, mut arg) = s.split_at(3);

    arg = arg.trim();
    let op;
    if arg.starts_with('+') {
        op = Op::Add
    } else {
        op = Op::Sub
    }

    match name {
        "nop" => Instruction::NOP,
        "acc" => Instruction::ACC(op, arg[1..].parse().unwrap()),
        "jmp" => Instruction::JMP(op, arg[1..].parse().unwrap()),
        _ => panic!("Unsupported instruction: {}", name),
    }
}

fn main() {
    let f = File::open("src/08/input.txt").unwrap();

    let mut instructions = Vec::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator = 0_i64;
    let mut idx = 0_usize;

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        instructions.push(parse_instruction(l));
    }

    loop {
        if visited.contains(&idx) {
            break;
        }

        visited.insert(idx);

        match instructions.get(idx).unwrap() {
            Instruction::NOP => idx += 1,
            Instruction::ACC(op, val) => {
                if *op == Op::Add {
                    accumulator += *val as i64;
                } else {
                    accumulator -= *val as i64;
                }
                idx += 1;
            }
            Instruction::JMP(op, val) => {
                if *op == Op::Add {
                    idx += val;
                } else {
                    idx -= val;
                }
            }
        }
    }

    println!("Accumulator is: {}", accumulator);
}
