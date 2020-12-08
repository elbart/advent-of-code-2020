use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
enum Instruction {
    NOP(Op, usize),
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
        "nop" => Instruction::NOP(op, arg[1..].parse().unwrap()),
        "acc" => Instruction::ACC(op, arg[1..].parse().unwrap()),
        "jmp" => Instruction::JMP(op, arg[1..].parse().unwrap()),
        _ => panic!("Unsupported instruction: {}", name),
    }
}

fn try_fix(
    mut instructions: Vec<Instruction>,
    last_unsuccessful_idx: usize,
) -> Result<(i64, usize), usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut accumulator = 0_i64;
    let mut idx = 0_usize;
    let (potential_idx, new_instr) = instructions
        .iter()
        .enumerate()
        .rev()
        .find_map(|(idx, x)| {
            if idx > last_unsuccessful_idx {
                return None;
            }

            match x {
                Instruction::NOP(op, val) => {
                    return Some((idx, Instruction::JMP(op.clone(), val.clone())))
                }
                Instruction::JMP(op, val) => {
                    return Some((idx, Instruction::NOP(op.clone(), val.clone())))
                }
                _ => return None,
            }
        })
        .unwrap();

    instructions[potential_idx] = new_instr;

    loop {
        // in this case, we were not successful
        if visited.contains(&idx) {
            return Err(potential_idx);
        }

        // here, we successfully
        if idx == instructions.len() {
            return Ok((accumulator, potential_idx));
        }

        visited.insert(idx);

        match instructions.get(idx).unwrap() {
            Instruction::NOP(_op, _val) => idx += 1,
            Instruction::ACC(op, val) => {
                if *op == Op::Add {
                    accumulator += *val as i64;
                } else {
                    accumulator -= *val as i64;
                }
                idx += 1;
            }
            Instruction::JMP(op, val) => {
                // println!("JMP op: {:?} val: {} ... cur_idx: {}", op, val, idx);
                if *op == Op::Add {
                    idx += val;
                } else {
                    idx -= val;
                }
            }
        }
    }
}

fn main() {
    let f = File::open("src/08/input.txt").unwrap();

    let mut instructions = Vec::new();

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        instructions.push(parse_instruction(l));
    }

    let mut last_unsuccessful_idx = instructions.len() - 1;

    loop {
        match try_fix(instructions.clone(), last_unsuccessful_idx) {
            Err(i) => {
                if last_unsuccessful_idx == i {
                    last_unsuccessful_idx -= 1;
                } else {
                    last_unsuccessful_idx = i;
                }
            }
            Ok((acc, idx)) => {
                println!(
                    "Accumulator is: {}, exchanged op was: {:?} on line: {}",
                    acc,
                    instructions[idx],
                    idx + 1
                );
                break;
            }
        }
    }
}
