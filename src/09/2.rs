use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

const PREAMBLE: usize = 25;

fn find_summands(sum: usize, idx: usize, numbers: &Vec<usize>, used_idx: &mut HashSet<usize>) -> Option<(usize, usize)> {
    let slice: Vec<(usize,usize)> = numbers[idx-PREAMBLE..idx].iter().enumerate().map(|(slice_idx, &slice_num)| {
        (slice_idx + idx - PREAMBLE, slice_num)
    }).collect();
    let mut it1 = slice.iter();

    it1.find_map(|(it_idx, it_num)| {
        if used_idx.contains(it_idx) {
            return None;
        }

        let mut it2 = slice.iter();
        it2.find_map(|(it_sub_idx, it_sub_num)| {
            if used_idx.contains(it_sub_idx) || it_sub_idx == it_idx {
                return None;
            }

            if it_sub_num + it_num == sum {
                // println!("Found summands: {} + {}", it_num, it_sub_num);
                return Some((*it_num, *it_sub_num));
            }

            None
        })
    })
}

fn find_contiguous_summands(numbers: &Vec<usize>, sum: usize) -> Option<Vec<usize>> {
    let mut temp_sum = Vec::new();
    let mut skip = 0;

    loop {
        let res = numbers.iter().skip(skip).find(|&&n| {
            temp_sum.push(n);

            if temp_sum.iter().sum::<usize>() == sum {
                return true;
            }

            return false;
        });

        if res.is_some() {
                return Some(temp_sum.clone());
        } else {
            temp_sum.clear();
            skip += 1;
        }
    }
        
}

fn main() {
    let f = File::open("src/09/input.txt").unwrap();

    let mut numbers: Vec<usize> = Vec::new();
    let faulty_number: usize;
    let mut used_idx = HashSet::new();
    
    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        numbers.push(l.parse().unwrap());
    }

    faulty_number = numbers.iter().enumerate().skip(PREAMBLE).find_map(|(idx,&n)| {
        let res = find_summands(n, idx, &numbers, &mut used_idx);

        if res.is_none() {
            return Some(n);
        }

        None
    }).unwrap();

    let min: usize;
    let max: usize;

    loop {
        match find_contiguous_summands(&numbers, faulty_number) {
            Some(x) => {
                min = *x.iter().min().unwrap();
                max = *x.iter().max().unwrap();
                break;
            },
            None => ()
        }
    }
    
    println!("The faulty number is: {}", faulty_number);
    println!("Encrypting Weakness: {}", min + max);
}
